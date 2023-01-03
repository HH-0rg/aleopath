use crate::ByteCode;
use crate::util;
use super::function::FunctionType;

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

pub struct IoRegister {
    register: Register,
    function_type: FunctionType,
}

impl IoRegister {
    pub fn read(bytes: &mut ByteCode, function_type: FunctionType) -> Self {
        match function_type {
            FunctionType::Function => {
                let register = Register::read(bytes);
                Self {
                    register,
                    function_type,
                }
            },
            _ => todo!()
        }
    }
}