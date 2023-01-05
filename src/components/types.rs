use std::fmt::Debug;

use crate::ByteCode;
use crate::util;

#[derive(Debug)]
pub enum Type {
    // The Address type
    Address,
    // The boolean type
    Boolean,
    // The field type (base field)
    Field,
    // The group type (affine)
    Group,
    // The 8-bit signed integer type
    I8,
    // The 16-bit signed integer type
    I16,
    // The 32-bit signed integer type
    I32,
    // The 64-bit signed integer type
    I64,
    // The 128-bit signed integer type
    I128,
    // The 8-bit unsigned integer type
    U8,
    // The 16-bit unsigned integer type
    U16,
    // The 32-bit unsigned integer type
    U32,
    // The 64-bit unsigned integer type
    U64,
    // The 128-bit unsigned integer type
    U128,
    // The scalar type
    Scalar,
    // The string type
    String,
    // User defined Type
    Other(String)
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Address,
            1 => Self::Boolean,
            2 => Self::Field,
            3 => Self::Group,
            4 => Self::I8,
            5 => Self::I16,
            6 => Self::I32,
            7 => Self::I64,
            8 => Self::I128,
            9 => Self::U8,
            10 => Self::U16,
            11 => Self::U32,
            12 => Self::U64,
            13 => Self::U128,
            14 => Self::Scalar,
            15 => Self::String,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum Attribute {
    Constant,
    Public,
    Private,
    Record,
    ExternalRecord,
}

impl From<usize> for Attribute {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Constant,
            1 => Self::Public,
            2 => Self::Private,
            3 => Self::Record,
            4 => Self::ExternalRecord,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Address(Vec<u8>),
    Boolean(bool),
    Field(Vec<u8>),
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
    Scalar(Vec<u8>),
}

impl Literal {
    pub fn read(bytes: &mut ByteCode) -> Self {
        let literal_type  = bytes.read_u16();
        match literal_type {
            // Address
            0 => Literal::Address(bytes.read_n(32)),
            // Boolean
            1 => Literal::Boolean(bytes.read_u8() != 0),
            // Field
            2 => Literal::Field(bytes.read_n(32)),
            // No group literals allowed
            3 => unreachable!(),
            // I8
            4 => Literal::I8(bytes.read_i8()),
            // I16
            5 => Literal::I16(bytes.read_i16()),
            // I32
            6 => Literal::I32(bytes.read_i32()),
            // I64
            7 => Literal::I64(bytes.read_i64()),
            // I128
            8 => Literal::I128(bytes.read_i128()),
            // U8
            9 => Literal::U8(bytes.read_u8()),
            // U16
            10 => Literal::U16(bytes.read_u16()),
            // U32
            11 => Literal::U32(bytes.read_u32()),
            // U64
            12 => Literal::U64(bytes.read_u64()),
            // U128
            13 => Literal::U128(bytes.read_u128()),
            // Scalar
            14 => Literal::Scalar(bytes.read_n(32)),
            // Unhandled Literal Type
            _ => unreachable!()
        }
    }

    pub fn literal_type(&self) -> Type {
        match self {
            Self::Address(_) =>  Type::Address,
            Self::Boolean(_) =>  Type::Boolean,
            Self::Field(_) =>  Type::Field,
            Self::I8(_) =>  Type::I8,
            Self::I16(_) =>  Type::I16,
            Self::I32(_) =>  Type::I32,
            Self::I64(_) =>  Type::I64,
            Self::I128(_) =>  Type::I128,
            Self::U8(_) =>  Type::U8,
            Self::U16(_) =>  Type::U16,
            Self::U32(_) =>  Type::U32,
            Self::U64(_) =>  Type::U64,
            Self::U128(_) =>  Type::U128,
            Self::Scalar(_) =>  Type::Scalar,
        }
    }
}


pub fn read_attribute(bytes: &mut ByteCode) -> Attribute {
    Attribute::from(bytes.read_u8() as usize)
}

pub fn read_plaintext_type(bytes: &mut ByteCode) -> Type {
    match bytes.read_u8() {
        // Literal Type
        0 => Type::from(bytes.read_u16()),
        // Identifier for user defined type
        1 => Type::Other(util::read_identifier(bytes)),
        _ => unreachable!(),
    }
}

pub fn read_function_register_type(bytes: &mut ByteCode) -> (Type, Attribute) {
    let attribute = read_attribute(bytes);
    let value_type = match attribute {
        Attribute::Private | Attribute::Public | Attribute::Constant => read_plaintext_type(bytes),
        _ => todo!(),
    };
    (value_type, attribute)
}
