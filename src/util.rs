use crate::ByteCode;
use crate::components::types::Attribute;

pub fn read_identifier(bytes: &mut ByteCode) -> String {
    let n = bytes.read_u8();
    String::from_utf8(bytes.read_n(n as usize)).unwrap()
}

pub fn read_variable_length_int(bytes: &mut ByteCode) -> usize {
    match bytes.read_u8() {
        253 => bytes.read_u16() as usize,
        254 => bytes.read_u32() as usize,
        255 => bytes.read_u64() as usize,
        f => f as usize,
    }
}
pub fn read_identifiers(bytes: &mut ByteCode) -> Vec<String> {
    let n = bytes.read_u16();
    (0..n).map(|_| read_identifier(bytes)).collect()
}

pub fn read_attribute_type(bytes: &mut ByteCode) -> Attribute {
    Attribute::from(bytes.read_u8() as usize)
}