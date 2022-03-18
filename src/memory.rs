use crate::{Addressable, AddressingMode};

pub struct Memory {
    data: [u8; 0x10000],
}

impl Addressable for Memory {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr]
    }

    fn write(&self, addr: u16, data: u8) {
        self.data[addr] = data;
    }
}
