pub trait Readable {
    fn read(&self, addr: u16) -> u8 {
        unimplemented!("unimplemented Readable trait")
    }
    fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read(addr) as u16;
        let high = self.read(addr + 1) as u16;
        (high << 8) | low
    }
}
pub trait Writable {
    fn write(&mut self, addr: u16, data: u8) {
        unimplemented!("unimplemented Writable trait")
    }
    fn write_u16(&mut self, addr: u16, data: u16) {
        let low = (data & 0xFF) as u8;
        let high = (data >> 8) as u8;
        self.write(addr, low);
        self.write(addr + 1, high);
    }
}

pub trait Addressable: Readable + Writable {}

pub trait ReadableMut {
    fn read(&mut self, addr: u16) -> u8 {
        unimplemented!("unimplemented ReadableMut trait")
    }
    fn read_u16(&mut self, addr: u16) -> u16 {
        let low = self.read(addr) as u16;
        let high = self.read(addr + 1) as u16;
        (high << 8) | low
    }
}

pub trait AddressableMut: ReadableMut + Writable {}
