use crate::ByteCode;

pub fn read_identifier(bytes: &mut ByteCode) -> String {
    let n = bytes.read_u8();
    String::from_utf8(bytes.read_n(n as usize)).unwrap()
}
