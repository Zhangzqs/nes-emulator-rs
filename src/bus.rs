use crate::{addressable::Addressable, memory::Memory};

//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | I/O Registers |       |               |

// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|

// $0000-$1FFF 2K字节RAM，做4次镜象（即$0000-$07FF可用)
// $2000- $2007 寄存器
// $2008-$3FFF 寄存器（$2000-$2008的镜像，每8个字节镜像一次）
// $4000-$401F 寄存器
// $4020-$5FFF 扩展ROM
// $6000-$7FFF 卡带的SRAM（需要有电池支持）
// $8000-$BFFF 卡带的下层ROM
// $C000-$FFFF 卡带的上层ROM
pub struct Bus {
    ram: Box<dyn Addressable>,
    rom: Box<dyn Addressable>,
    ppu: Box<dyn Addressable>,
    sram: Box<dyn Addressable>,
}

pub struct BusBuilder {
    ram: Option<Box<dyn Addressable>>,
    rom: Option<Box<dyn Addressable>>,
    ppu: Option<Box<dyn Addressable>>,
    sram: Option<Box<dyn Addressable>>,
}
impl BusBuilder {
    pub fn new() -> Self {
        Self {
            ram: None,
            rom: None,
            ppu: None,
            sram: None,
        }
    }
    pub fn ram(mut self, ram: Box<dyn Addressable>) -> Self {
        self.ram = Some(ram);
        self
    }
    pub fn rom(mut self, rom: Box<dyn Addressable>) -> Self {
        self.rom = Some(rom);
        self
    }
    pub fn ppu(mut self, ppu: Box<dyn Addressable>) -> Self {
        self.ppu = Some(ppu);
        self
    }
    pub fn sram(mut self, sram: Box<dyn Addressable>) -> Self {
        self.sram = Some(sram);
        self
    }
    pub fn build(mut self) -> Result<Bus, String> {
        if let None = self.ram {
            return Err("No ram".to_string());
        }

        if let None = self.rom {
            return Err("No rom".to_string());
        }

        if let None = self.ppu {
            return Err("No ppu".to_string());
        }

        if let None = self.sram {
            self.sram = Some(Box::new(Memory::new(0x1FFF)))
        }
        let ram = self.ram.unwrap();
        let rom = self.rom.unwrap();
        let ppu = self.ppu.unwrap();
        let sram = self.sram.unwrap();
        Ok(Bus {
            ram,
            rom,
            ppu,
            sram,
        })
    }
}

enum Device {
    Ram(u16),
    Rom(u16),
    Ppu(u16),
    Sram(u16),
    Unknown,
}

fn address_translation(addr: u16) -> Device {
    match addr {
        0x0000..=0x1FFF => {
            let mirror_down_addr = addr & 0b00000111_11111111;
            Device::Ram(mirror_down_addr)
        }
        0x2000..=0x3FFF => {
            let mirror_down_addr = addr & 0b00100000_00000111;
            Device::Ppu(mirror_down_addr - 0x2000)
        }
        0x6000..=0x7FFF => Device::Sram(addr - 6000),
        0x8000..=0xFFFF => Device::Rom(addr - 0x8000),
        _ => {
            println!("Ignoring mem access at 0x{:04X}", addr);
            Device::Unknown
        }
    }
}

impl Addressable for Bus {
    fn read(&self, addr: u16) -> u8 {
        match address_translation(addr) {
            Device::Ram(addr) => self.ram.read(addr),
            Device::Rom(addr) => self.rom.read(addr),
            Device::Ppu(addr) => self.ppu.read(addr),
            Device::Sram(addr) => self.sram.read(addr),
            Device::Unknown => todo!(),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match address_translation(addr) {
            Device::Ram(addr) => self.ram.write(addr, data),
            Device::Rom(addr) => self.rom.write(addr, data),
            Device::Ppu(addr) => self.ppu.write(addr, data),
            Device::Sram(addr) => self.sram.write(addr, data),
            Device::Unknown => todo!(),
        }
    }
}
