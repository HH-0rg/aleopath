use crate::{ByteCode, util};
use super::types::{Type, self};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
struct Entry {
    name: String,
    value_type: Type
}

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
