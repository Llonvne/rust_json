use crate::number::JsonNumber::*;

#[derive(Debug)]
pub enum JsonNumber {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    F32(f32),
    F64(f64),
}

impl From<i8> for JsonNumber {
    fn from(value: i8) -> Self {
        I8(value)
    }
}

impl From<i16> for JsonNumber {
    fn from(value: i16) -> Self {
        I16(value)
    }
}
impl From<i32> for JsonNumber {
    fn from(value: i32) -> Self {
        I32(value)
    }
}
impl From<i64> for JsonNumber {
    fn from(value: i64) -> Self {
        I64(value)
    }
}
impl From<i128> for JsonNumber {
    fn from(value: i128) -> Self {
        I128(value)
    }
}
impl From<u8> for JsonNumber {
    fn from(value: u8) -> Self {
        U8(value)
    }
}

impl From<u16> for JsonNumber {
    fn from(value: u16) -> Self {
        U16(value)
    }
}
impl From<u32> for JsonNumber {
    fn from(value: u32) -> Self {
        U32(value)
    }
}
impl From<u64> for JsonNumber {
    fn from(value: u64) -> Self {
        U64(value)
    }
}
impl From<u128> for JsonNumber {
    fn from(value: u128) -> Self {
        U128(value)
    }
}

impl From<f32> for JsonNumber {
    fn from(value: f32) -> Self {
        F32(value)
    }
}
impl From<f64> for JsonNumber {
    fn from(value: f64) -> Self {
        F64(value)
    }
}
