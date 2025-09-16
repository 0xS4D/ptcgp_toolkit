pub const SIZEOF_ARM64_INSTRUCTION: usize = 4;

pub const RET_INSTRUCTION_BYTES: [u8; SIZEOF_ARM64_INSTRUCTION] = [0xC0, 0x03, 0x5F, 0xD6];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    Xzr,
}

impl TryFrom<u8> for Register {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::X0),
            1 => Ok(Register::X1),
            2 => Ok(Register::X2),
            3 => Ok(Register::X3),
            4 => Ok(Register::X4),
            5 => Ok(Register::X5),
            6 => Ok(Register::X6),
            7 => Ok(Register::X7),
            8 => Ok(Register::X8),
            9 => Ok(Register::X9),
            10 => Ok(Register::X10),
            11 => Ok(Register::X11),
            12 => Ok(Register::X12),
            13 => Ok(Register::X13),
            14 => Ok(Register::X14),
            15 => Ok(Register::X15),
            16 => Ok(Register::X16),
            17 => Ok(Register::X17),
            18 => Ok(Register::X18),
            19 => Ok(Register::X19),
            20 => Ok(Register::X20),
            21 => Ok(Register::X21),
            22 => Ok(Register::X22),
            23 => Ok(Register::X23),
            24 => Ok(Register::X24),
            25 => Ok(Register::X25),
            26 => Ok(Register::X26),
            27 => Ok(Register::X27),
            28 => Ok(Register::X28),
            29 => Ok(Register::X29),
            30 => Ok(Register::X30),
            31 => Ok(Register::Xzr),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftAmount {
    Lsl0,
    Lsl16,
    Lsl32,
    Lsl48,
}

impl ShiftAmount {
    pub fn to_u8(self) -> u8 {
        match self {
            ShiftAmount::Lsl0 => 0,
            ShiftAmount::Lsl16 => 1,
            ShiftAmount::Lsl32 => 2,
            ShiftAmount::Lsl48 => 3,
        }
    }

    pub fn to_shift_bits(self) -> u8 {
        self.to_u8() * 16
    }
}

impl TryFrom<u8> for ShiftAmount {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ShiftAmount::Lsl0),
            1 => Ok(ShiftAmount::Lsl16),
            2 => Ok(ShiftAmount::Lsl32),
            3 => Ok(ShiftAmount::Lsl48),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct MovRegister {
    pub sf: u8,
    pub rm: Register,
    pub rd: Register,
}

#[derive(Debug)]
pub struct MovBitmaskImmediate {
    pub sf: u8,
    pub n: u8,
    pub immr: u8,
    pub imms: u8,
    pub rd: Register,
}

#[derive(Debug)]
pub enum Mov {
    Register(MovRegister),
    BitmaskImmediate(MovBitmaskImmediate),
}

impl Mov {
    pub fn rd(&self) -> Register {
        match self {
            Mov::Register(r) => r.rd,
            Mov::BitmaskImmediate(bi) => bi.rd,
        }
    }
}

pub fn parse_mov(inst: u32) -> Option<Mov> {
    if ((inst >> 5) & 0x1F) != 0x1F {
        return None;
    }
    let sf = ((inst >> 31) & 0x1) as u8;

    if ((inst >> 24) & 0x7F) == 0x2A {
        if ((inst >> 22) & 0x3) != 0 {
            return None;
        }
        if ((inst >> 21) & 0x1) != 0 {
            return None;
        }
        if ((inst >> 10) & 0x3F) != 0 {
            return None;
        }
        let rm_val = ((inst >> 16) & 0x1F) as u8;
        let rd_val = (inst & 0x1F) as u8;
        let rm = Register::try_from(rm_val).ok()?;
        let rd = Register::try_from(rd_val).ok()?;
        return Some(Mov::Register(MovRegister { sf, rm, rd }));
    }

    if ((inst >> 23) & 0xFF) == 0x64 {
        let n = ((inst >> 22) & 0x1) as u8;
        let immr = ((inst >> 16) & 0x3F) as u8;
        let imms = ((inst >> 10) & 0x3F) as u8;
        let rd_val = (inst & 0x1F) as u8;
        let rd = Register::try_from(rd_val).ok()?;
        return Some(Mov::BitmaskImmediate(MovBitmaskImmediate {
            sf,
            n,
            immr,
            imms,
            rd,
        }));
    }

    None
}

impl MovBitmaskImmediate {
    pub fn imm(&self) -> u64 {
        let reg_size = if self.sf == 1 { 64 } else { 32 };

        let combined: u32 = ((self.n as u32) << 6) | (self.imms as u32);
        let mut len = 0;
        for i in (0..7).rev() {
            if ((combined >> i) & 1) == 1 {
                len = i;
                break;
            }
        }

        let esize = if len == 6 { 64 } else { 1 << (len + 1) };

        let mask = (1 << (len + 1)) - 1;
        let d = ((self.imms as u32) & mask) + 1;

        let mut welem: u64 = if d >= 64 { u64::MAX } else { (1u64 << d) - 1 };

        let r = (self.immr as u32) % esize;
        welem = welem.rotate_right(r);

        let repeats = reg_size / esize;
        let mut imm: u64 = 0;
        for i in 0..repeats {
            imm |= welem << (i * esize);
        }

        imm
    }
}

#[derive(Debug)]
pub struct Movz {
    pub sf: u8,
    pub opc: u8,
    pub hw: ShiftAmount,
    pub imm16: u16,
    pub rd: Register,
}

pub fn parse_movz(inst: u32) -> Option<Movz> {
    if ((inst >> 23) & 0xFF) != 0xA5 {
        return None;
    }
    let sf = ((inst >> 31) & 0x1) as u8;
    let opc = ((inst >> 29) & 0x3) as u8;
    let hw_val = ((inst >> 21) & 0x3) as u8;
    let hw = ShiftAmount::try_from(hw_val).ok()?;
    let imm16 = ((inst >> 5) & 0xffff) as u16;
    let rd_val = (inst & 0x1F) as u8;
    let rd = Register::try_from(rd_val).ok()?;
    Some(Movz {
        sf,
        opc,
        hw,
        imm16,
        rd,
    })
}

#[derive(Debug)]
pub struct Movk {
    pub sf: u8,
    pub opc: u8,
    pub hw: ShiftAmount,
    pub imm16: u16,
    pub rd: Register,
}

pub fn parse_movk(inst: u32) -> Option<Movk> {
    if ((inst >> 23) & 0xFF) != 0xE5 {
        return None;
    }
    let sf = ((inst >> 31) & 0x1) as u8;
    let opc = ((inst >> 29) & 0x3) as u8;
    let hw_val = ((inst >> 21) & 0x3) as u8;
    let hw = ShiftAmount::try_from(hw_val).ok()?;
    let imm16 = ((inst >> 5) & 0xffff) as u16;
    let rd_val = (inst & 0x1F) as u8;
    let rd = Register::try_from(rd_val).ok()?;
    Some(Movk {
        sf,
        opc,
        hw,
        imm16,
        rd,
    })
}

#[derive(Debug)]
pub struct Madd {
    pub sf: u8,
    pub rm: Register,
    pub ra: Register,
    pub rn: Register,
    pub rd: Register,
}
pub fn parse_madd(inst: u32) -> Option<Madd> {
    if ((inst >> 21) & 0x3FF) != 0b0011011000 {
        return None;
    }
    if ((inst >> 15) & 0x1) != 0 {
        return None;
    }
    let sf = ((inst >> 31) & 0x1) as u8;
    let rm_val = ((inst >> 16) & 0x1F) as u8;
    let ra_val = ((inst >> 10) & 0x1F) as u8;
    let rn_val = ((inst >> 5) & 0x1F) as u8;
    let rd_val = (inst & 0x1F) as u8;
    let rm = Register::try_from(rm_val).ok()?;
    let ra = Register::try_from(ra_val).ok()?;
    let rn = Register::try_from(rn_val).ok()?;
    let rd = Register::try_from(rd_val).ok()?;
    Some(Madd { sf, rm, ra, rn, rd })
}

#[derive(Debug)]
pub struct Movn {
    pub sf: u8,
    pub opc: u8,
    pub hw: ShiftAmount,
    pub imm16: u16,
    pub rd: Register,
}

pub fn parse_movn(inst: u32) -> Option<Movn> {
    let sf = ((inst >> 31) & 0x1) as u8;
    let top9 = inst >> 23;
    let expected_top9 = ((sf as u32) << 8) | 0x25;
    if top9 != expected_top9 {
        return None;
    }
    let opc = ((inst >> 23) & 0xFF) as u8;
    let hw_val = ((inst >> 21) & 0x3) as u8;
    if sf == 0 && (hw_val >> 1) == 1 {
        return None;
    }
    let hw = ShiftAmount::try_from(hw_val).ok()?;
    let imm16 = ((inst >> 5) & 0xffff) as u16;
    let rd_val = (inst & 0x1F) as u8;
    let rd = Register::try_from(rd_val).ok()?;
    Some(Movn {
        sf,
        opc,
        hw,
        imm16,
        rd,
    })
}

#[derive(Debug)]
pub struct Bl {
    pub imm26: i32,
    pub offset: i64,
}
pub fn parse_bl(inst: u32) -> Option<Bl> {
    if ((inst >> 26) & 0x3F) != 0b100101 {
        return None;
    }
    let imm26 = (inst & 0x03FF_FFFF) as i32;
    let imm26_signed = (imm26 << 6) >> 6;
    let offset = (imm26_signed as i64) << 2;
    Some(Bl {
        imm26: imm26_signed,
        offset,
    })
}

#[derive(Debug)]
pub struct Adrp {
    pub immlo: u8,
    pub immhi: u32,
    pub rd: Register,
}

impl Adrp {
    pub fn compute_imm(&self) -> i64 {
        let imm21 = ((self.immhi as i64) << 2) | (self.immlo as i64);
        let imm33 = imm21 << 12;
        let shift = 64 - 33;
        (imm33 << shift) >> shift
    }
}
pub fn parse_adrp(inst: u32) -> Option<Adrp> {
    if ((inst >> 31) & 0x1) != 1 {
        return None;
    }
    if ((inst >> 24) & 0x1F) != 0b10000 {
        return None;
    }
    let immlo = ((inst >> 29) & 0x3) as u8;
    let immhi = (inst >> 5) & 0x7FFFF;
    let rd_val = (inst & 0x1F) as u8;
    let rd = Register::try_from(rd_val).ok()?;
    Some(Adrp { immlo, immhi, rd })
}

#[derive(Debug)]
pub struct AddImmediate {
    pub sf: u8,
    pub sh: u8,
    pub imm12: u16,
    pub rn: Register,
    pub rd: Register,
}

impl AddImmediate {
    pub fn immediate(&self) -> u64 {
        if self.sh == 1 {
            (self.imm12 as u64) << 12
        } else {
            self.imm12 as u64
        }
    }
}

pub fn parse_add_immediate(inst: u32) -> Option<AddImmediate> {
    let sf = ((inst >> 31) & 0x1) as u8;
    let op = ((inst >> 23) & 0xFF) as u8;
    if op != 0x22 {
        return None;
    }
    let sh = ((inst >> 22) & 0x1) as u8;
    let imm12 = ((inst >> 10) & 0xFFF) as u16;
    let rn_val = ((inst >> 5) & 0x1F) as u8;
    let rn = Register::try_from(rn_val).ok()?;
    let rd_val = (inst & 0x1F) as u8;
    let rd = Register::try_from(rd_val).ok()?;

    Some(AddImmediate {
        sf,
        sh,
        imm12,
        rn,
        rd,
    })
}
