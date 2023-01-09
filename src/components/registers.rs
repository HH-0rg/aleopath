use crate::ByteCode;
use crate::util;
use super::function::FunctionType;
use super::{Type, Attribute};
use super::types;
use crate::output::Assembly;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Register {
    locator: usize,
    identifiers: Vec<String>,
}

impl Register {
    pub fn read(bytes: &mut ByteCode) -> Self {
        let f = bytes.read_u8();
        let locator = util::read_variable_length_int(bytes);
        let identifiers = {
            if f == 1 {
                util::read_identifiers(bytes)
            } else {
                vec![]
            }
        };
        Self {locator, identifiers}
    }
}

impl Assembly for Register {
    fn assembly(&self) -> String {
        if self.identifiers.len() == 0 {
            format!("r{}", self.locator)
        } else {
            format!("r{}.{}", self.locator, self.identifiers.join("."))
        }
    }
}

#[derive(Debug, Clone)]
pub enum IOType {
    Input,
    Output
}

#[derive(Debug, Clone)]
pub struct IoRegister {
    register: Register,
    io_type: IOType,
    function_type: FunctionType,
    value_type: Type,
    attribute_type: Attribute,
}

impl IoRegister {
    pub fn read(bytes: &mut ByteCode, function_type: FunctionType, io_type: IOType) -> Self {
        match function_type {
            FunctionType::Function => {
                let register = Register::read(bytes);
                let (value_type, attribute_type) = types::read_function_register_type(bytes);
                Self {
                    register,
                    io_type,
                    function_type,
                    value_type,
                    attribute_type,
                }
            },
            FunctionType::Closure => todo!(),
            FunctionType::Finalize => todo!(),
            FunctionType::Uninitialized => unimplemented!(),
        }
    }
}

impl Assembly for IoRegister {
    fn assembly(&self) -> String {
        let io = match self.io_type {
            IOType::Input => "input".to_string(),
            IOType::Output => "output".to_string(),
        };
        format!("{} {} as {}.{}", io, self.register.assembly(), self.value_type.assembly(), self.attribute_type.assembly())
    }
}