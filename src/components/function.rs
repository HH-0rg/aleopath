use crate::ByteCode;
use crate::util;
use super::registers::IoRegister;

pub enum FunctionType {
    Function,
    Closure,
    Finalize,
    Uninitialized
}

impl Default for FunctionType {
    fn default() -> Self {
        FunctionType::Uninitialized
    }
}

#[derive(Default)]
pub struct Function {
    name: String,
    variant: FunctionType,
    num_inputs: u16,
    inputs: Vec<IoRegister>,
}

impl Function {
    pub fn read(bytes: &mut ByteCode, variant: FunctionType) -> Self {
        let name = util::read_identifier(bytes);
        let num_inputs = bytes.read_u16();
        Self::default()
    }
}
