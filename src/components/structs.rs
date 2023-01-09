use crate::output::Assembly;
use crate::{ByteCode, util};
use super::types::{Type, self};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
struct Entry {
    name: String,
    value_type: Type
}

impl Assembly for Entry {
    fn assembly(&self) -> String {
        format!("{} as {}", self.name, self.value_type.assembly())
    }

    fn leo(&self) -> String {
        format!("{}: {},", self.name, self.value_type.assembly())
     }
}

#[derive(Debug)]
pub struct Struct {
    name: String,
    entries: Vec<Entry>,
}

impl Struct {
    pub fn read(bytes: &mut ByteCode) -> Self {
        let name = util::read_identifier(bytes);
        let num_entries = bytes.read_u16();
        let entries: Vec<Entry> = (0..num_entries)
            .map(|_| Entry{
                name: util::read_identifier(bytes),
                value_type: types::read_plaintext_type(bytes),
            })
            .collect();
        Self {
            name,
            entries
        }
    }
}

impl Assembly for Struct {
    fn assembly(&self) -> String {
        let entries = self.entries.iter().map(|i| format!("\t{};", i.assembly())).collect::<Vec<String>>().join("\n"); 
        format!("struct {}:\n{}\n", self.name, entries)
    }

    fn leo(&self) -> String {
        let entries = self.entries.iter().map(|i| format!("\t{}", i.leo())).collect::<Vec<String>>().join("\n"); 
        format!("struct {} {{\n{}}}\n", self.name, entries)
     }
}