use std::{io::Cursor, mem::offset_of};

use anyhow::{Result, anyhow, bail};
use nohash_hasher::IntMap;

use crate::{
    decrypt::file_offset_to_va,
    unity::{
        binary::elf::{Elf, POINTER_SIZE},
        generated::il2cpp_2022333f1::root::{
            IL2CPP_TYPE_ENUM, Il2CppCodeRegistration, Il2CppMetadataRegistration, Il2CppType,
            Il2CppTypeEnum,
        },
        utils::{global_metadata::Metadata, read_only::ReadOnly},
    },
};

pub struct Il2Cpp<'a> {
    pub elf: Elf<'a>,
    pub metadata: Metadata,
    code_registration: ReadOnly<Il2CppCodeRegistration>,
    metadata_registration: ReadOnly<Il2CppMetadataRegistration>,
    pub types: Vec<ReadOnly<&'a Il2CppType>>,
    type_ptr_map: IntMap<u64, usize>,
}

impl<'a> Il2Cpp<'a> {
    pub fn load_from_vec(elf_vec: Vec<u8>, global_metadata_data: Vec<u8>) -> Result<Self> {
        let elf = Elf::new(elf_vec)?;
        let reader = Cursor::new(global_metadata_data);
        let metadata = Metadata::load_from_reader(reader)?;

        let code_registration = Self::find_code_registration(&elf, &metadata)?;
        let metadata_registration = Self::find_metadata_registration(&elf, &metadata)?;

        let types = Self::inner_load_data_array::<Il2CppType>(
            &elf,
            metadata_registration.types,
            metadata_registration.typesCount as usize,
        )?
        .into_iter()
        .map(ReadOnly::new)
        .collect();

        let type_ptr_map = elf
            .read_pointer_array(
                metadata_registration.types as u64,
                metadata_registration.typesCount as usize,
            )
            .into_iter()
            .enumerate()
            .map(|(idx, ptr)| (ptr, idx))
            .collect::<IntMap<_, _>>();

        Ok(Il2Cpp {
            elf,
            metadata,
            code_registration,
            metadata_registration,
            types,
            type_ptr_map,
        })
    }

    pub fn find_code_registration(
        elf: &Elf,
        metadata: &Metadata,
    ) -> Result<ReadOnly<Il2CppCodeRegistration>> {
        const PATTERN: &[u8; 13] = b"mscorlib.dll\0";
        let mscorlib_file_offsets = elf.search_elf(PATTERN);

        let mut mscorlib_vaddrs = Vec::new();
        for &file_off in &mscorlib_file_offsets {
            if let Some(program_header) = elf
                .inner
                .program_headers
                .iter()
                .find(|ph| ph.p_offset <= file_off && file_off < ph.p_offset + ph.p_filesz)
            {
                let offset_in_segment = file_off - program_header.p_offset;
                let va = program_header.p_vaddr + offset_in_segment;
                mscorlib_vaddrs.push(va);
            }
        }
        if mscorlib_vaddrs.is_empty() {
            bail!("No occurrences of 'mscorlib.dll' found in ELF");
        }

        let mut mscorlib_refs: Vec<&u64> = Vec::new();
        for &mscorlib_va in &mscorlib_vaddrs {
            if let Some(relocs) = elf.relocations.get(&(mscorlib_va as i64)) {
                mscorlib_refs.extend(relocs);
            }
        }
        if mscorlib_refs.is_empty() {
            bail!("No references to 'mscorlib.dll' found");
        }

        let mut second_level_refs: Vec<u64> = Vec::new();
        for &ref_a in &mscorlib_refs {
            if let Some(relocs) = elf.relocations.get(&(*ref_a as i64)) {
                second_level_refs.extend(relocs);
            }
        }
        if second_level_refs.is_empty() {
            bail!("No second-level references found");
        }

        let mut image_names: Vec<String> = metadata
            .images
            .iter()
            .map(|img| metadata.get_string_by_index(img.nameIndex))
            .collect();

        image_names.sort();

        let mscorlib_idx = match image_names.binary_search(&"mscorlib.dll".to_string()) {
            Ok(idx) => idx as u64,
            Err(_) => bail!("mscorlib.dll not found in Metadata images"),
        };

        let images_ref_start = mscorlib_idx * POINTER_SIZE as u64;

        let mut possible_code_reg_bases: Vec<u64> = Vec::new();
        for &b_ref in &second_level_refs {
            let base_candidate = b_ref.wrapping_sub(images_ref_start);
            if let Some(relocs) = elf.relocations.get(&(base_candidate as i64)) {
                possible_code_reg_bases.extend(relocs);
            }
        }

        const CODEGEN_MODULES_OFFSET: usize = offset_of!(Il2CppCodeRegistration, codeGenModules);
        const CODE_REGISTRATION_SIZE: usize = size_of::<Il2CppCodeRegistration>();

        let total_image_count = image_names.len() as u32;
        for &candidate_va in &possible_code_reg_bases {
            let struct_start_va = candidate_va.saturating_sub(CODEGEN_MODULES_OFFSET as u64);
            if let Some(bytes) = elf.read_bytes_at_va(struct_start_va, CODE_REGISTRATION_SIZE) {
                let code_reg = unsafe { *(bytes.as_ptr() as *const Il2CppCodeRegistration) };
                if code_reg.codeGenModulesCount == total_image_count {
                    return Ok(ReadOnly::new(code_reg));
                }
            }
        }

        Err(anyhow!("Could not find a valid Il2CppCodeRegistration"))
    }

    pub fn find_metadata_registration(
        elf: &Elf,
        metadata: &Metadata,
    ) -> Result<ReadOnly<Il2CppMetadataRegistration>> {
        let pattern = (metadata.type_definitions.len() as u64)
            .to_le_bytes()
            .to_vec();

        let field_count_file_offsets = elf.search_elf(&pattern);

        const TYPEDEF_SIZES_COUNT_OFFSET: usize =
            offset_of!(Il2CppMetadataRegistration, typeDefinitionsSizesCount);
        const METADATA_REGISTRATION_SIZE: usize = size_of::<Il2CppCodeRegistration>();

        let possible_metadata_regs = field_count_file_offsets
            .into_iter()
            .filter_map(|field_count_offset| {
                let type_count_offset = field_count_offset as usize + (POINTER_SIZE * 2);
                if type_count_offset > elf.data.len() - POINTER_SIZE {
                    return None;
                }
                if elf.data[type_count_offset..type_count_offset + pattern.len()] != pattern {
                    return None;
                }
                if let Some(candidate_va) = file_offset_to_va(&elf.inner, type_count_offset as u64)
                {
                    let struct_start_va =
                        candidate_va.saturating_sub(TYPEDEF_SIZES_COUNT_OFFSET as u64);
                    if let Some(bytes) =
                        elf.read_bytes_at_va(struct_start_va, METADATA_REGISTRATION_SIZE)
                    {
                        let metadata_reg =
                            unsafe { &*(bytes.as_ptr() as *const Il2CppMetadataRegistration) };
                        return Some(metadata_reg);
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        match possible_metadata_regs.len() {
            0 => Err(anyhow!("Could not find a valid Il2CppMetadataRegistration")),
            1 => Ok(ReadOnly::new(*possible_metadata_regs[0])),
            _ => {
                for metadata_reg in possible_metadata_regs {
                    let type_defs_sizes_ptr_va = metadata_reg.typeDefinitionsSizes as u64;

                    if !elf.is_valid_pointer(type_defs_sizes_ptr_va) {
                        continue;
                    }

                    let type_defs_sizes_array = elf.read_pointer_array(
                        type_defs_sizes_ptr_va,
                        metadata_reg.typeDefinitionsSizesCount as usize,
                    );

                    if !type_defs_sizes_array
                        .iter()
                        .any(|&ptr| !elf.is_valid_pointer(ptr))
                    {
                        continue;
                    }

                    return Ok(ReadOnly::new(*metadata_reg));
                }
                Err(anyhow!("Could not find a valid Il2CppMetadataRegistration"))
            }
        }
    }

    pub fn load_data_instance<T>(&'a self, data_ptr: u64) -> Result<&'a T> {
        Self::inner_load_data_instance(&self.elf, data_ptr)
    }

    pub fn inner_load_data_instance<T>(elf: &Elf, data_ptr: u64) -> Result<&'a T> {
        if !elf.is_valid_pointer(data_ptr) {
            bail!("Invalid pointer");
        }

        let data_size = size_of::<T>();
        let data_bytes = elf
            .read_bytes_at_va(data_ptr, data_size)
            .ok_or(anyhow!("Failed to read data"))?;

        let reference: &T = unsafe { &*(data_bytes.as_ptr() as *const T) };

        Ok(reference)
    }

    pub fn inner_load_data_array<T>(
        elf: &Elf,
        ptr: *const *const T,
        count: usize,
    ) -> Result<Vec<&'a T>> {
        if !elf.is_valid_pointer(ptr as u64) {
            bail!("Invalid pointer");
        }

        let data_ptr_array = elf.read_pointer_array(ptr as u64, count);

        let mut arr_refs = Vec::with_capacity(count);

        for &data_ptr in &data_ptr_array {
            arr_refs.push(Self::inner_load_data_instance::<T>(elf, data_ptr)?);
        }

        Ok(arr_refs)
    }

    pub fn read_encoded_type_enum(
        &'a self,
        data: &[u8],
        offset: usize,
    ) -> (Il2CppTypeEnum, Option<&'a ReadOnly<&'a Il2CppType>>) {
        let ty = self.metadata.read_u8(data, offset) as Il2CppTypeEnum;
        if ty == IL2CPP_TYPE_ENUM {
            let ty_idx = self.metadata.read_compressed_i32(data, offset + 1);
            let ty = &self.types[ty_idx as usize];
            let ty_def = unsafe { self.metadata.type_definitions[ty.data.__klassIndex as usize] };
            (
                self.types[ty_def.elementTypeIndex as usize].type_(),
                Some(ty),
            )
        } else {
            (ty, None)
        }
    }

    pub fn type_by_ptr(&self, ptr: u64) -> Option<&ReadOnly<&Il2CppType>> {
        let idx = *self.type_ptr_map.get(&ptr)?;
        let ty = self.types.get(idx)?;
        Some(ty)
    }
}
