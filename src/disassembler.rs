use core::fmt::Write;
use crate::bytecode::ByteCode;
use crate::util;
use crate::components::{ Function, Mapping, Struct, Record, records };
use crate::components::function::FunctionType;
use crate::output::Assembly;
use alloc::{vec::Vec, string::String};

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
    mappings: Vec<Mapping>,
    structs: Vec<Struct>,
    records: Vec<Record>,
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
            let x = self.bytes.read_u8();
            match x {
                0 => self.mappings.push(Mapping::read(&mut self.bytes)),
                1 => self.structs.push(Struct::read(&mut self.bytes)),
                2 => self.records.push(Record::read(&mut self.bytes)),
                3 => self.functions.push(Function::read(&mut self.bytes, FunctionType::Closure)),
                4 => self.functions.push(Function::read(&mut self.bytes, FunctionType::Function)),
                _ => unimplemented!()
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
        let mappings = self.mappings.iter().map(|m| format!("{}", m.assembly())).collect::<Vec<String>>().join("\n\n");
        let functions = self.functions.iter().map(|f| format!("{}", f.assembly())).collect::<Vec<String>>().join("\n\n");
        let structs = self.structs.iter().map(|s| format!("{}", s.assembly())).collect::<Vec<String>>().join("\n\n");
        let records = self.records.iter().map(|s| format!("{}", s.assembly())).collect::<Vec<String>>().join("\n\n");
        o.write_fmt(format_args!("{}\n", mappings)).unwrap();
        o.write_fmt(format_args!("{}\n", records)).unwrap();
        o.write_fmt(format_args!("{}\n", structs)).unwrap();
        o.write_fmt(format_args!("{}\n", functions)).unwrap();
        o
    }
    
    fn leo(&self) -> String {
        let mut o = String::new();
        let functions = self.functions.iter().map(|f| format!("{}", f.leo())).collect::<Vec<String>>().join("\n\n");
        o.write_fmt(format_args!("{}\n", functions)).unwrap();
        o
    }
}