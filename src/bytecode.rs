use std::mem::transmute;

#[derive(Default, Debug)]
pub struct ByteCode {
    bytes: Vec<u8>,
}

impl ByteCode {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn read_u8(&mut self) -> u8 {
        self.bytes.remove(0)
    }

    // Read two bytes as unsigned int in little endian
    pub fn read_u16(&mut self) -> u16 {
        let a: Vec<u8> = self.bytes.drain(0..2).collect();
        unsafe { transmute::<[u8; 2], u16>([a[0], a[1]]) }
    }

    // Read four bytes as unsigned int in little endian
    pub fn read_u32(&mut self) -> u32 {
        let a: Vec<u8> = self.bytes.drain(0..4).collect();
        unsafe { transmute::<[u8; 4], u32>([a[0], a[1], a[2], a[3]]) }
    }

    pub fn read_u64(&mut self) -> u64 {
        self.read_u32() as u64 + (self.read_u32() as u64) << 32
    }

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

    pub fn read_n(&mut self, n: usize) -> Vec<u8> {
        self.bytes.drain(0..n).collect()
    }
}