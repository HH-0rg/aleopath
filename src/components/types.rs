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
}

pub enum Visibility {
    Constant,
    Public,
    Private,
    Record,
    ExternalRecord,
}

impl From<usize> for Type {
    fn from(value: usize) -> Self {
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

impl From<usize> for Visibility {
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