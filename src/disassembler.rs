use std::fmt::Write;

use crate::bytecode::ByteCode;
use crate::util;
use crate::components::function::{ Function, FunctionType };
use crate::output::Assembly;

#[derive(Default, Debug)]
pub struct Disassembler {
    bytes: ByteCode,
    version: u16,
    program_name: String,
    network: String,
    num_imports: u8,
    imports: Vec<(String, String)>,
    num_components: u16,
    functions: Vec<Function>,
}

impl Disassembler {
    pub fn from_bytes(buf: Vec<u8>) -> Self {
        Self { bytes: ByteCode::new(buf), ..Default::default() }
    }

    fn read_header(&mut self) {
        self.version = self.bytes.read_u16();
        (self.program_name, self.network) = util::read_programid(&mut self.bytes);
        self.num_imports = self.bytes.read_u8();
        self.imports = (0..self.num_imports).map(|_| util::read_programid(&mut self.bytes)).collect();
    }

    fn read_num_components(&mut self) {
        self.num_components = self.bytes.read_u16();
    }

    fn read_components(&mut self) {
        for _ in 0..self.num_components {
            match self.bytes.read_u8() {
                3 => self.functions.push(Function::read(&mut self.bytes, FunctionType::Closure)),
                4 => self.functions.push(Function::read(&mut self.bytes, FunctionType::Function)),
                _ => todo!()
            }
        }
    }

    pub fn get_version(&self) -> u16 {
        self.version
    }

    pub fn get_program_id(&self) -> (&str, &str) {
        (&self.program_name, &self.network)
    }


    pub fn disassemble(&mut self) {
        self.read_header();
        self.read_num_components();
        self.read_components();
    }

}

impl Assembly for Disassembler {
    fn assembly(&self) -> String {
        let mut o = String::new();
        o.write_fmt(format_args!("program {}.{}\n\n", self.program_name, self.network)).unwrap();
        let functions = self.functions.iter().map(|f| format!("{}", f.assembly())).collect::<Vec<String>>().join("\n\n");
        o.write_fmt(format_args!("{}\n", functions)).unwrap();
        o
    }
}