#![no_std]

#[macro_use]
mod macros;
mod bytecode;
mod disassembler;
mod util;
mod components;
mod output;
#[macro_use]
extern crate alloc;
use alloc::{vec::Vec, string::String};
pub use disassembler::Disassembler;
pub use bytecode::ByteCode;
use output::Assembly;
use wasm_bindgen::prelude::*;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn disassemble(bytes: &str) -> String {
    let v: Vec<u8> = (0..bytes.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&bytes[i..i + 2], 16).unwrap())
        .collect();
    let mut a = Disassembler::from_bytes(v);
    a.disassemble();
    a.assembly()
}

#[wasm_bindgen]
pub fn decompile(bytes: &str) -> String {
    let v: Vec<u8> = (0..bytes.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&bytes[i..i + 2], 16).unwrap())
        .collect();
    let mut a = Disassembler::from_bytes(v);
    a.disassemble();
    a.leo()
}

// #[cfg(test)]
// mod tests {
//     use std::fs;
//     use crate::{Disassembler, output::Assembly};

//     // #[test]
//     // fn function() {
//     //     let file = "examples/function/main.avm";
//     //     let file_contents = fs::read(file).expect("couldn't read file");
//     //     println!("length: {}", file_contents.len());
//     //     let mut a = Disassembler::from_bytes(file_contents);
//     //     a.disassemble();
//     //     println!("{}", a.assembly());
//     // }

//     // #[test]
//     // fn mapping() {
//     //     let file = "examples/mapping/main.avm";
//     //     let file_contents = fs::read(file).expect("couldn't read file");
//     //     println!("length: {}", file_contents.len());
//     //     let mut a = Disassembler::from_bytes(file_contents);
//     //     a.disassemble();
//     //     println!("{}", a.assembly());
//     // }

//     #[test]
//     fn all_components() {
//         let file = "examples/all_components/main.avm";
//         let file_contents = fs::read(file).expect("couldn't read file");
//         println!("length: {}", file_contents.len());
//         let mut a = Disassembler::from_bytes(file_contents);
//         a.disassemble();
//         println!("{}", a.leo());
//     }
// }