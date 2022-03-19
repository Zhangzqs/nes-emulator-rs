use crate::bus::Addressable;

pub struct Memory {
    data: [u8; 0x10000],
}
impl Memory {
    pub fn new() -> Self {
        Self { data: [0; 65536] }
    }
}
impl Addressable for Memory {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.data[addr as usize] = data;
    }
}
