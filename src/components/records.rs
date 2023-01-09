use crate::output::Assembly;
use crate::{ByteCode, util};

use super::types::{Type, Attribute, self};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
struct Entry {
    name: String,
    value_type: Type,
    attribute_type: Attribute,
}

impl Entry {
    fn read(bytes: &mut ByteCode) -> Self {
        Self {
            name: util::read_identifier(bytes),
            attribute_type: types::read_attribute(bytes),
            value_type: types::read_plaintext_type(bytes),
        }
    }
}

impl Assembly for Entry {
    fn assembly(&self) -> String {
        format!("{} as {}.{}", self.name, self.value_type.assembly(), self.attribute_type.assembly())
    }
}

#[derive(Debug)]
pub struct Record {
    name: String,
    owner_attribute: Attribute,
    gates_attribute: Attribute,
    entries: Vec<Entry>,
}

impl Record {
    pub fn read(bytes: &mut ByteCode) -> Self {
        let name = util::read_identifier(bytes);
        
        let owner_attribute = match bytes.read_u8() {
            0 => Attribute::Public,
            1 => Attribute::Private,
            _ => unreachable!(),
        };

        let gates_attribute = match bytes.read_u8() {
            0 => Attribute::Public,
            1 => Attribute::Private,
            _ => unreachable!(),
        };

        let num_entries = bytes.read_u16();
        let entries = (0..num_entries).map(|_| Entry::read(bytes)).collect();

        Self {
            name,
            owner_attribute,
            gates_attribute,
            entries
        }
    }
}

impl Assembly for Record {
    fn assembly(&self) -> String {
        let entries = self.entries.iter().map(|i| format!("\t{}", i.assembly())).collect::<Vec<String>>().join("\n");
        format!("record {}:\n\towner as address.{}\n\tgates as u64.{}\n{}", self.name, self.owner_attribute.assembly(), self.gates_attribute.assembly(), entries)
    }
}