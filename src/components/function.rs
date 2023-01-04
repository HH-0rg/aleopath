use crate::ByteCode;
use crate::util;
use super::registers::{ IoRegister, IOType };

#[derive(Clone, Copy, Debug)]
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

#[derive(Default, Debug)]
pub struct Function {
    name: String,
    function_type: FunctionType,
    num_inputs: u16,
    inputs: Vec<IoRegister>,
}

impl Function {
    pub fn read(bytes: &mut ByteCode, function_type: FunctionType) -> Self {
        let name = util::read_identifier(bytes);
        let num_inputs = bytes.read_u16();
        let inputs: Vec<IoRegister> = (0..num_inputs).map(|_| IoRegister::read(bytes, function_type, IOType::Input)).collect();
        Self {
            name,
            function_type,
            num_inputs,
            inputs,
        }
    }
}
