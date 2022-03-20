pub trait IMapper {
    fn read(address: u16) -> u8;
    fn write(address: u16, data: u8);
}
