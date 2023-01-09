use alloc::{vec::Vec};

#[derive(Default, Debug)]
pub struct ByteCode {
    bytes: Vec<u8>,
    pub idx: usize,
}

impl ByteCode {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, idx: 0 }
    }

    // Peek one byte as unsigned int in little endian
    pub fn peek(&self) -> u8 {
        self.bytes[0]
    }

    // Read one byte as unsigned int in little endian
    pub fn read_u8(&mut self) -> u8 {
        // let x = self.bytes[self.idx];
        // self.idx += 1;
        // x
        self.bytes.remove(0)
    }

    // Read two bytes as unsigned int in little endian
    pub fn read_u16(&mut self) -> u16 {
        (self.read_u8() as u16) + ((self.read_u8() as u16) << 8)
    }

    // Read four bytes as unsigned int in little endian
    pub fn read_u32(&mut self) -> u32 {
        (self.read_u16() as u32) + ((self.read_u16() as u32) << 16)
    }

    // Read eight bytes as unsigned int in little endian
    pub fn read_u64(&mut self) -> u64 {
        (self.read_u32() as u64) + ((self.read_u32() as u64) << 32)
    }

    // Read 16 bytes as unsigned int in little endian
    pub fn read_u128(&mut self) -> u128 {
        (self.read_u64() as u128) + ((self.read_u64() as u128) << 64)
    }

    // Read one byte as signed int in little endian
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    // Read two bytes as signed int in little endian
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }
    
    // Read four bytes as signed int in little endian
    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }

    // Read eight bytes as signed int in little endian
    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }

    // Read eight bytes as signed int in little endian
    pub fn read_i128(&mut self) -> i128 {
        self.read_u128() as i128
    }

    // Read n bytes
    pub fn read_n(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.read_u8()).collect()
    }
}