pub trait Addressable {
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, data: u8);

    fn read_u16(&self, pos: u16) -> u16 {
        let low = self.read(pos) as u16;
        let high = self.read(pos + 1) as u16;
        (high << 8) | low
    }
    fn write_u16(&self, pos: u16, data: u16) {
        let low = (data & 0xFF) as u8;
        let high = (data >> 8) as u8;
        self.write(pos, low);
        self.write(pos + 1, high);
    }
}
