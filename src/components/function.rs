use core::fmt::Write;

use crate::ByteCode;
use crate::output::Assembly;
use crate::util;
use super::registers::{ IoRegister, IOType };
use super::instructions::Instruction;
use alloc::string::String;
use alloc::vec::Vec;

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
        let _finalize = bytes.read_u8();
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

impl Assembly for Function {
    fn assembly(&self) -> String {
        let mut o = String::new();
        o.write_str("function ").unwrap();
        o.write_fmt(format_args!("{}\n", self.name)).unwrap();
        for i in self.inputs.clone() {
            o.write_str("\t").unwrap();
            o.write_fmt(format_args!("{}\n", i.assembly())).unwrap();
        }
        let instructions = self.instructions.iter().map(|i| format!("\t{}", i.assembly())).collect::<Vec<String>>().join("\n"); 
        o.write_fmt(format_args!("{instructions}\n")).unwrap();
        for i in self.outputs.clone() {
            o.write_str("\t").unwrap();
            o.write_fmt(format_args!("{}\n", i.assembly())).unwrap();
        }
        o
    }

    fn leo(&self) -> String {
        // return the leo source code
        // function signature followed by code
        let mut o = String::new();
        o.write_str("function ").unwrap();
        o.write_fmt(format_args!("{}(", self.name)).unwrap();
        for i in self.inputs.clone() {
            o.write_fmt(format_args!("{}", i.leo())).unwrap();
            //comma
            o.write_str(", ").unwrap();
        }
        //remove last comma
        o.pop();
        o.pop();
        o.write_str(") {\n").unwrap();
        let instructions = self.instructions.iter().map(|i| format!("\t{}", i.leo())).collect::<Vec<String>>().join("\n"); 
        o.write_fmt(format_args!("{instructions}\n}}\n")).unwrap();
        o

    }
}