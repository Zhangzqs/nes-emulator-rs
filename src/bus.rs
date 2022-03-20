use std::ops::{Add, Range, RangeInclusive};

pub trait Addressable {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);

    fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read(addr) as u16;
        let high = self.read(addr + 1) as u16;
        (high << 8) | low
    }
    fn write_u16(&mut self, addr: u16, data: u16) {
        let low = (data & 0xFF) as u8;
        let high = (data >> 8) as u8;
        self.write(addr, low);
        self.write(addr + 1, high);
    }
}

type ReadCallback = Box<dyn Fn(u16) -> u8>;
type WriteCallback = Box<dyn Fn(u16, u8)>;

struct IoDevice {
    start_addr: u16,
    length: u16,
    read_callback: ReadCallback,
    write_callback: WriteCallback,
}
impl IoDevice {
    fn is_it_address(&self, addr: u16) -> bool {
        addr >= self.start_addr && addr < self.start_addr + self.length
    }
}
pub struct Bus {
    devices: Vec<IoDevice>,
}

impl Bus {
    pub fn new() -> Self {
        Self { devices: vec![] }
    }
    pub fn register_device(
        &mut self,
        start_addr: u16,
        length: u16,
        read_callback: ReadCallback,
        write_callback: WriteCallback,
    ) {
        self.devices.push(IoDevice {
            start_addr,
            length,
            read_callback,
            write_callback,
        })
    }

    fn find_device(&self, addr: u16) -> &IoDevice {
        for device in &self.devices {
            if device.is_it_address(addr) {
                return device;
            }
        }
        panic!("Device not found!!! Address: {}", addr)
    }
}

impl Addressable for Bus {
    fn read(&self, addr: u16) -> u8 {
        (self.find_device(addr).read_callback)(addr)
    }

    fn write(&mut self, addr: u16, data: u8) {
        (self.find_device(addr).write_callback)(addr, data)
    }
}

#[test]
fn test_bus_map() {
    let mut bus = Bus::new();
    bus.register_device(
        0,
        100,
        Box::new(|addr| (addr + 1) as u8),
        Box::new(|addr, data| {}),
    );
    bus.register_device(
        100,
        100,
        Box::new(|addr| (addr + 2) as u8),
        Box::new(|addr, data| {}),
    );
    for addr in 0..100 {
        assert_eq!(bus.read(addr), addr as u8 + 1)
    }
    for addr in 100..200 {
        assert_eq!(bus.read(addr), addr as u8 + 2)
    }
}
