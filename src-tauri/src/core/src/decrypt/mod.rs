use aes::Aes128;
use anyhow::{Result, anyhow, bail};
use ctr::{
    Ctr128BE,
    cipher::{KeyIvInit, StreamCipher},
};
use goblin::elf::Elf;
use goblin::elf::program_header::PT_LOAD;

fn find_text_section<'a>(elf: &Elf, data: &'a [u8]) -> Result<(&'a [u8], u64, usize)> {
    for sh in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(sh.sh_name) {
            if name == ".text" {
                let start = sh.sh_offset as usize;
                let end = start + sh.sh_size as usize;
                let va = sh.sh_addr;
                return Ok((&data[start..end], va, start));
            }
        }
    }
    bail!("Cannot found .text section.")
}

pub fn file_offset_to_va(elf: &Elf, file_offset: u64) -> Option<u64> {
    elf.program_headers
        .iter()
        .find(|ph| {
            ph.p_type == PT_LOAD
                && file_offset >= ph.p_offset
                && file_offset < ph.p_offset + ph.p_filesz
        })
        .map(|ph| ph.p_vaddr + (file_offset - ph.p_offset))
}

pub fn va_to_file_offset(elf: &Elf, va: u64) -> Option<usize> {
    elf.program_headers
        .iter()
        .find(|ph| ph.p_type == PT_LOAD && va >= ph.p_vaddr && va < ph.p_vaddr + ph.p_filesz)
        .map(|ph| (ph.p_offset + (va - ph.p_vaddr)) as usize)
}

fn parse_movz_x1(inst: u32) -> Option<u64> {
    ((inst >> 23) & 0xFF == 0xA5 && (inst & 0x1F) == 1).then(|| ((inst >> 5) & 0xFFFF) as u64)
}

fn parse_movk_x1(inst: u32, expected_hw: u8) -> Option<u64> {
    let matches = (inst >> 23) & 0xFF == 0xE5
        && (inst & 0x1F) == 1
        && ((inst >> 21) & 0x3) == expected_hw as u32;

    matches.then(|| ((inst >> 5) & 0xFFFF) as u64)
}

fn parse_adrp(inst: u32) -> Option<(u8, i64)> {
    if (inst >> 31) & 1 != 1 || (inst >> 24) & 0x1F != 0b10000 {
        return None;
    }

    let rd = (inst & 0x1F) as u8;
    let immlo = ((inst >> 29) & 0x3) as u32;
    let immhi = (inst >> 5) & 0x7FFFF;
    let combined = ((immhi << 2) | immlo) as u64;

    let imm = if combined & 0x100000 != 0 {
        (combined | 0xFFFFFFFFFFE00000) << 12
    } else {
        combined << 12
    } as i64;

    Some((rd, imm))
}

fn parse_add_imm(inst: u32) -> Option<(u8, u8, u64)> {
    ((inst >> 23) & 0xFF == 0x22).then(|| {
        let rd = (inst & 0x1F) as u8;
        let rn = ((inst >> 5) & 0x1F) as u8;
        let imm12 = ((inst >> 10) & 0xFFF) as u64;
        (rd, rn, imm12)
    })
}

fn parse_bl(inst: u32) -> Option<i64> {
    if (inst >> 26) != 0b100101 {
        return None;
    }
    let imm26 = inst & 0x3FFFFFF;
    let offset = if imm26 & 0x2000000 != 0 {
        (imm26 | 0xFC000000) as i32 as i64
    } else {
        imm26 as i64
    };
    Some(offset * 4)
}

pub fn extract_metadata_key_xor(elf: &Elf, elf_data: &[u8]) -> Result<(usize, u64)> {
    let (text_data, _text_va, _text_file_offset) = find_text_section(elf, elf_data)?;

    let instructions: Vec<u32> = text_data
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    for (i, window) in instructions.windows(5).enumerate() {
        if let Some(key_xor) = try_extract_key_xor_pattern(window) {
            return Ok((i * 4, key_xor));
        }
    }

    bail!("Cannot find key_xor.")
}

fn try_extract_key_xor_pattern(window: &[u32]) -> Option<u64> {
    let imm1 = parse_movz_x1(window[0])?;

    let imm3 = parse_movk_x1(window[2], 1)?;

    let imm4 = parse_movk_x1(window[3], 2)?;

    let imm5 = parse_movk_x1(window[4], 3)?;

    Some((imm5 << 48) | (imm4 << 32) | (imm3 << 16) | imm1)
}

pub fn extract_metadata_key(elf: &Elf, elf_data: &[u8], key_xor_offset: usize) -> Result<[u8; 16]> {
    let (text_data, _, text_file_offset) = find_text_section(elf, elf_data)?;

    let instructions: Vec<u32> = text_data
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    let key_xor_inst_offset = key_xor_offset / 4;

    let bl_inst_offset = instructions[key_xor_inst_offset..]
        .iter()
        .enumerate()
        .find_map(|(offset, &inst)| {
            parse_bl(inst).and_then(|bl_offset| {
                let current_offset = (key_xor_inst_offset + offset) as i64;
                let target_offset = current_offset + (bl_offset / 4);
                (target_offset >= 0 && target_offset < instructions.len() as i64)
                    .then(|| target_offset as usize)
            })
        })
        .ok_or_else(|| anyhow!("Cannot find BL instruction."))?;

    for (i, window) in instructions[bl_inst_offset..].windows(2).enumerate() {
        if let (Some((adrp_rd, adrp_imm)), Some((_, add_rn, add_imm))) =
            (parse_adrp(window[0]), parse_add_imm(window[1]))
        {
            if add_rn == adrp_rd {
                let adrp_index = bl_inst_offset + i;
                let adrp_file_offset = text_file_offset + adrp_index * 4;

                if let Some(adrp_va) = file_offset_to_va(elf, adrp_file_offset as u64) {
                    let page_base = adrp_va & !0xfff;
                    let key_va = (page_base as i64 + adrp_imm + add_imm as i64) as u64;

                    if let Some(key_offset) = va_to_file_offset(elf, key_va) {
                        if key_offset + 16 <= elf_data.len() {
                            let mut key = [0u8; 16];
                            key.copy_from_slice(&elf_data[key_offset..key_offset + 16]);
                            return Ok(key);
                        }
                    }
                }
            }
        }
    }

    bail!("Cannot find key.")
}

pub fn decrypt_metadata(data: &[u8], key: &[u8; 16], key_xor: u64) -> Result<Vec<u8>> {
    if data.len() < 4 {
        bail!("Data too short.");
    }

    let ciphertext_len = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    if data.len() != 4 + ciphertext_len {
        bail!("Data length mismatch.");
    }

    let ciphertext = &data[4..];
    let key_xor_bytes = key_xor.to_le_bytes();
    let key_bytes: [u8; 16] = key
        .iter()
        .zip(key_xor_bytes.iter().cycle())
        .map(|(&b, &k)| b ^ k)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut iv = [0u8; 16];
    iv[8..].copy_from_slice(&1u64.to_be_bytes());

    let mut cipher = Ctr128BE::<Aes128>::new(&key_bytes.into(), &iv.into());
    let mut plaintext = ciphertext.to_vec();
    cipher.apply_keystream(&mut plaintext);

    Ok(plaintext)
}
