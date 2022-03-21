use crate::addressable::Addressable;

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
}

pub struct BusBuilder {
    ram: Option<Box<dyn Addressable>>,
}
impl BusBuilder {
    pub fn new() -> Self {
        Self { ram: None }
    }
    pub fn ram(mut self, ram: Box<dyn Addressable>) -> Self {
        self.ram = Some(ram);
        self
    }
    pub fn build(self) -> Result<Bus, String> {
        if let None = self.ram {
            return Err("No ram".to_string());
        }

        let ram = self.ram.unwrap();
        Ok(Bus { ram })
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

impl Addressable for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.ram.read(mirror_down_addr)
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet")
            }
            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;
                self.ram.write(mirror_down_addr, data)
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet");
            }
            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }
}
