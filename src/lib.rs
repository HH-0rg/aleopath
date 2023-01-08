mod bytecode;
mod disassembler;
mod util;
mod components;
mod output;

pub use disassembler::Disassembler;
pub use bytecode::ByteCode;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::Disassembler;

    #[test]
    fn disassemble() {
        let file = "examples/hello_test/main.avm";
        let file_contents = fs::read(file).expect("couldn't read file");
        println!("length: {}", file_contents.len());
        let mut a = Disassembler::from_bytes(file_contents);
        a.disassemble();
        println!("{}, {:?}", a.get_version(), a);
    }
}