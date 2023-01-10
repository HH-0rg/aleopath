# Aleopathy

Aleopathy is a WASM-compatible Aleo Virtual Machine (AVM) bytecode to Leo decompiler, written in Rust.

## Example

```rust
    fn main() {
        let file = "path/to/main.avm";
        let file_contents = fs::read(file).expect("couldn't read file");
        let mut a = Disassembler::from_bytes(file_contents);
        a.disassemble();
        println!("{}", a.assembly());
    }
```

## Compiling to WASM

```bash
cargo install wasm-pack
wasm-pack build --target web --out-dir pkg
```

This outputs a folder `pkg` which contains a node package with the following APIs exposed.
- `export function disassemble(bytes: string): string;`
- `export function decompile(bytes: string): [string, string];`

