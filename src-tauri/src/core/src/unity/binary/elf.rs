use anyhow::{Result, bail};
use goblin::elf32::program_header::{PF_W, PF_X, PT_LOAD};
use goblin::elf64::reloc;
use hashbrown::HashMap;
use nohash_hasher::IntMap;
use std::mem::{size_of, transmute};
use std::ops::Range;

use crate::decrypt::va_to_file_offset;
use crate::unity::binary::arm64::SIZEOF_ARM64_INSTRUCTION;
use crate::unity::binary::search::find_pattern;

type RelocMap = IntMap<i64, Vec<u64>>;

pub const POINTER_SIZE: usize = size_of::<u64>();

pub struct Elf<'a> {
    pub inner: goblin::elf::Elf<'a>,
    pub data: Vec<u8>,
    pub original_data: Vec<u8>,
    pub relocations: RelocMap,
    pub sections: HashMap<String, Range<usize>>,
    pub instructions: HashMap<String, Vec<u32>>,
}

impl<'a> Elf<'a> {
    pub fn new(data: Vec<u8>) -> Result<Elf<'a>> {
        let original_data = data;
        let elf = goblin::elf::Elf::parse(&original_data)?;

        if !elf.is_64 {
            bail!("Only 64-bit ELF files are supported");
        }

        let (data, relocations) = Self::apply_dynamic_relocations(&elf, original_data.clone())?;

        let sections = Self::get_section_slices(&elf);

        let instructions = Self::get_instruction_chunks(&elf, &data);

        let inner = unsafe { transmute::<goblin::elf::Elf<'_>, goblin::elf::Elf<'a>>(elf) };

        Ok(Elf {
            inner,
            data,
            original_data,
            relocations,
            sections,
            instructions,
        })
    }

    pub fn apply_dynamic_relocations(
        elf: &goblin::elf::Elf,
        mut data: Vec<u8>,
    ) -> Result<(Vec<u8>, RelocMap)> {
        let mut relocs = RelocMap::default();
        for rela in &elf.dynrelas {
            let target_va = rela.r_offset;
            let addend = match rela.r_addend {
                Some(addend) => {
                    relocs.entry(addend).or_default().push(target_va);
                    addend
                }
                None => 0,
            };

            if let Some(file_offset) = Self::inner_va_to_file_offset(elf, target_va) {
                match rela.r_type {
                    reloc::R_AARCH64_RELATIVE => {
                        let bytes = (addend as u64).to_le_bytes();
                        data[file_offset as usize..file_offset as usize + 8]
                            .copy_from_slice(&bytes);
                    }

                    reloc::R_AARCH64_GLOB_DAT | reloc::R_AARCH64_JUMP_SLOT => {
                        let symbol_addr = Self::inner_resolve_symbol(elf, rela.r_sym)?;
                        let bytes = symbol_addr.to_le_bytes();
                        data[file_offset as usize..file_offset as usize + 8]
                            .copy_from_slice(&bytes);
                    }

                    reloc::R_AARCH64_ABS64 => {
                        let symbol_addr = Self::inner_resolve_symbol(elf, rela.r_sym)?;
                        let relocation_value = symbol_addr + addend as u64;
                        let bytes = relocation_value.to_le_bytes();

                        data[file_offset as usize..file_offset as usize + 8]
                            .copy_from_slice(&bytes);
                    }

                    _ => {
                        bail!(
                            "Unhandled relocation type: {} at 0x{:x}",
                            rela.r_type,
                            target_va
                        );
                    }
                }
            } else {
                bail!(
                    "Could not find file offset for relocation at VA 0x{:x}",
                    target_va
                );
            }
        }

        Ok((data, relocs))
    }

    fn get_section_slices(elf: &goblin::elf::Elf) -> HashMap<String, Range<usize>> {
        elf.section_headers
            .iter()
            .filter_map(|section_hdr| {
                elf.shdr_strtab.get_at(section_hdr.sh_name).map(|name| {
                    let section_offset = section_hdr.sh_offset as usize;
                    let section_range =
                        section_offset..section_offset + section_hdr.sh_size as usize;
                    (name.to_string(), section_range)
                })
            })
            .collect()
    }

    fn get_instruction_chunks(elf: &goblin::elf::Elf, data: &[u8]) -> HashMap<String, Vec<u32>> {
        let mut instructions = HashMap::new();

        for section_hdr in &elf.section_headers {
            if section_hdr.is_executable() {
                if let Some(name) = elf.shdr_strtab.get_at(section_hdr.sh_name) {
                    let start = section_hdr.sh_offset as usize;
                    let size = section_hdr.sh_size as usize;

                    if start + size <= data.len() {
                        let section_data = &data[start..start + size];
                        let mut insn_vec = Vec::new();

                        for chunk in section_data.chunks_exact(SIZEOF_ARM64_INSTRUCTION) {
                            let insn = u32::from_le_bytes(chunk.try_into().unwrap());
                            insn_vec.push(insn);
                        }
                        instructions.insert(name.to_string(), insn_vec);
                    }
                }
            }
        }
        instructions
    }

    fn inner_va_to_file_offset(elf: &goblin::elf::Elf, va: u64) -> Option<u64> {
        for ph in &elf.program_headers {
            let ph_start = ph.p_vaddr;
            let ph_end = ph.p_vaddr + ph.p_filesz;
            if ph_start <= va && va < ph_end {
                let offset_in_segment = va - ph.p_vaddr;
                return Some(ph.p_offset + offset_in_segment);
            }
        }
        None
    }

    fn inner_resolve_symbol(elf: &goblin::elf::Elf, sym_index: usize) -> Result<u64> {
        let symbol = match elf.dynsyms.get(sym_index) {
            Some(sym) => sym,
            None => {
                bail!("Symbol not found for index: {}", sym_index);
            }
        };

        let resolved_address = symbol.st_value;

        Ok(resolved_address)
    }

    pub fn is_valid_pointer(&self, va: u64) -> bool {
        self.inner.program_headers.iter().any(|ph| {
            if ph.p_type == PT_LOAD {
                let is_executable = (ph.p_flags & PF_X) != 0;
                let is_writable = (ph.p_flags & PF_W) != 0;

                let in_range = ph.p_vaddr <= va && va < (ph.p_vaddr + ph.p_memsz);
                return in_range && (is_executable || is_writable);
            }
            false
        })
    }

    pub fn read_pointer_array(&self, va: u64, count: usize) -> Vec<u64> {
        let mut pointers = Vec::new();
        let mut current_va = va;

        for _ in 0..count {
            if let Some(file_offset) = va_to_file_offset(&self.inner, current_va) {
                if file_offset as u64 + POINTER_SIZE as u64 > self.data.len() as u64 {
                    break;
                }

                let ptr_bytes =
                    &self.data[file_offset as usize..file_offset as usize + POINTER_SIZE];
                let ptr_value = u64::from_le_bytes(ptr_bytes.try_into().unwrap());

                pointers.push(ptr_value);
                current_va += POINTER_SIZE as u64;
            } else {
                break;
            }
        }

        pointers
    }

    pub fn search_elf(&self, pattern: &[u8]) -> Vec<u64> {
        let mut results = Vec::new();

        for shdr in &self.inner.section_headers {
            if shdr.sh_size == 0 {
                continue;
            }
            let start = shdr.sh_offset as usize;
            let end = start + shdr.sh_size as usize;
            if end > self.data.len() {
                continue;
            }

            let section_bytes = &self.data[start..end];
            for result in find_pattern(section_bytes, pattern) {
                results.push((start + result) as u64);
            }
        }

        results
    }

    pub fn read_bytes_at_va(&'a self, va: u64, num_bytes: usize) -> Option<&'a [u8]> {
        let file_offset = va_to_file_offset(&self.inner, va)?;
        let slice_end = file_offset as usize + num_bytes;
        if slice_end > self.data.len() {
            return None;
        }
        Some(&self.data[file_offset as usize..slice_end])
    }
}
