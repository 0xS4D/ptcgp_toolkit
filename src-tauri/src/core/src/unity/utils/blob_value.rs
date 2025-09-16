use crate::unity::generated::il2cpp_2022333f1::root::{Il2CppType, Il2CppTypeEnum};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum BlobValueData {
    Boolean(bool),
    U1(u8),
    I1(i8),
    Char(char),
    U2(u16),
    I2(i16),
    U4(u32),
    I4(i32),
    U8(u64),
    I8(i64),
    R4(f32),
    R8(f64),
    String(String),
    Array(Vec<BlobValue>),
    TypeIndex(Option<Il2CppType>),
}

#[derive(Debug, Clone)]
pub struct BlobValue {
    pub il2cpp_type_enum: Il2CppTypeEnum,
    pub enum_type: Option<Il2CppType>,
    pub value: BlobValueData,
}

impl TryFrom<&BlobValue> for u64 {
    type Error = anyhow::Error;

    fn try_from(v: &BlobValue) -> Result<Self> {
        use BlobValueData::*;
        Ok(match &v.value {
            Boolean(b) => if *b { 1 } else { 0 },
            Char(c) => *c as u32 as u64,
            U1(x) => *x as u64,
            I1(x) => *x as u64,
            U2(x) => *x as u64,
            I2(x) => *x as u64,
            U4(x) => *x as u64,
            I4(x) => *x as u64,
            I8(x) => *x as u64,
            U8(x) => *x,
            other => return Err(anyhow!("Not convertible to number: {:?}", other)),
        })
    }
}

impl TryFrom<&BlobValue> for f64 {
    type Error = anyhow::Error;

    fn try_from(v: &BlobValue) -> Result<Self> {
        use BlobValueData::*;
        Ok(match &v.value {
            Boolean(b) => if *b { 1.0 } else { 0.0 },
            Char(c) => *c as u32 as f64,
            R4(x) => *x as f64,
            R8(x) => *x,
            U1(x) => *x as f64,
            I1(x) => *x as f64,
            U2(x) => *x as f64,
            I2(x) => *x as f64,
            U4(x) => *x as f64,
            I4(x) => *x as f64,
            U8(x) => *x as f64,
            I8(x) => *x as f64,
            other => return Err(anyhow!("Not convertible to float: {:?}", other)),
        })
    }
}

impl BlobValue {
    pub fn as_num(&self) -> Result<u64> {
        u64::try_from(self)
    }

    pub fn as_float(&self) -> Result<f64> {
        f64::try_from(self)
    }
}