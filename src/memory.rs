use crate::addressable::Addressable;

pub struct Memory {
    data: Vec<u8>,
}
impl Memory {
    pub fn new(max_address: u16) -> Self {
        Self {
            data: vec![0; max_address as usize],
        }
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
