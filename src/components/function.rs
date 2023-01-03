use crate::ByteCode;
use crate::util;

#[derive(Default)]
pub struct Function {
    name: String,
    num_inputs: u16,   
}

impl From<&mut ByteCode> for Function {
    fn from(bytes: &mut ByteCode) -> Self {
        let name = util::read_identifier(bytes);
        let num_inputs = bytes.read_u16();
        Self::default()
    }
}