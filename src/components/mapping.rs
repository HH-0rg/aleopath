use crate::output::Assembly;
use crate::ByteCode;
use super::{ Type, Attribute, types };
use crate::util;
use alloc::string::String;

#[derive(Debug)]
struct KeyValue {
    name: String,
    attribute_type: Attribute,
    value_type: Type,
}

impl KeyValue {
    fn read(bytes: &mut ByteCode) -> Self {
        let name = util::read_identifier(bytes);
        let attribute_type = match bytes.read_u8() {
            0 => Attribute::Public,
            1 => Attribute::Record,
            2 => Attribute::ExternalRecord,
            _ => unreachable!(),
        };
        let value_type = types::read_plaintext_type(bytes);
        Self {
            name,
            attribute_type,
            value_type
        }
    }
}

impl Assembly for KeyValue {
    fn assembly(&self) -> String {
        format!("{} as {}.{}", self.name, self.value_type.assembly(), self.attribute_type.assembly())
    }

    fn leo(&self) -> String {
        return String::new();
    }
}

#[derive(Debug)]
pub struct Mapping {
    name: String,
    key: KeyValue,
    value: KeyValue,
}

impl Mapping {
    pub fn read(bytes: &mut ByteCode) -> Self {
        Self {
            name: util::read_identifier(bytes),
            key: KeyValue::read(bytes),
            value: KeyValue::read(bytes),
        }
    }
}

impl Assembly for Mapping {
    fn assembly(&self) -> String {
        format!("mapping {}\n\tkey {}\n\tvalue {}\n", self.name, self.key.assembly(), self.value.assembly())
    }

    fn leo(&self) -> String {
        format!("mapping {}: {} => {};\n", self.name, self.key.value_type.assembly(), self.value.value_type.assembly())
    }
}