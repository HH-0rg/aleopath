use crate::ByteCode;
use crate::util;
use super::instructions;
use super::registers::{ IoRegister, IOType };
use super::instructions::Instruction;

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
    num_instructions: u32,
    instructions: Vec<Instruction>,
    num_outputs: u16,
    outputs: Vec<IoRegister>,
}

impl Function {
    pub fn read(bytes: &mut ByteCode, function_type: FunctionType) -> Self {
        let name = util::read_identifier(bytes);
        let num_inputs = bytes.read_u16();
        let inputs: Vec<IoRegister> = (0..num_inputs).map(|_| IoRegister::read(bytes, function_type, IOType::Input)).collect();
        let (num_instructions, instructions) = Instruction::read_instructions(bytes);
        let num_outputs = bytes.read_u16();
        let outputs: Vec<IoRegister> = (0..num_outputs).map(|_| IoRegister::read(bytes, function_type, IOType::Output)).collect();
        Self {
            name,
            function_type,
            num_inputs,
            inputs,
            num_instructions,
            instructions,
            num_outputs,
            outputs,
        }
    }
}
