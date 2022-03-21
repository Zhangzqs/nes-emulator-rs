pub trait FlagRegister: From<Self> + Into<u8> {
    fn update(&mut self, data: u8);
}
