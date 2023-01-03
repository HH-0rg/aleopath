mod bytecode;
mod disassembler;
mod util;
mod components;

pub use disassembler::Disassembler;
pub use bytecode::ByteCode;

#[cfg(test)]
mod tests {
    use std::fs;
    use  crate::Disassembler;

    #[test]
    fn disassemble() {
        let file = "/home/scar/Development/aleo/test/build/main.avm";
        let file_contents = fs::read(file).expect("couldn't read file");
        println!("length: {}", file_contents.len());
        let mut a = Disassembler::from_bytes(file_contents);
        a.disassemble();
        println!("{}, {:?}", a.get_version(), a.get_program_id());
    }
}