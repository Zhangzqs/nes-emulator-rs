use crate::{addressable::*, flag::FlagRegister, meta::Mirror};

use self::register::PpuRegister;
mod register;
// NES的分辨率为256x240

pub struct Ppu {
    /// 卡带上的数据
    pub chr_rom: Vec<u8>,
    /// 调色板
    pub palette_table: [u8; 32],
    /// 背景信息
    pub vram: [u8; 2048],
    /// 精灵数据
    pub oam_address: u8,
    pub oam_data: [u8; 256],

    pub mirror: Mirror,
    /// 寄存器数据
    pub register: PpuRegister,
    internal_data_buffer: u8,
}

impl Ppu {
    pub fn new_empty() -> Self {
        Self::new(vec![0; 2048], Mirror::Horizontal)
    }
    pub fn new(chr_rom: Vec<u8>, mirror: Mirror) -> Self {
        Self {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            mirror,
            register: PpuRegister::new(),
            oam_address: 0,
            internal_data_buffer: 0,
        }
    }
}

impl Ppu {
    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]

    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400;
        match (&self.mirror, name_table) {
            (Mirror::Vertical, 2) | (Mirror::Vertical, 3) => vram_index - 0x800,
            (Mirror::Horizontal, 2) => vram_index - 0x400,
            (Mirror::Horizontal, 1) => vram_index - 0x400,
            (Mirror::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.register
            .address
            .increment(self.register.control.vram_address_increment());
    }

    fn write_to_ctrl(&mut self, value: u8) {
        let before_nmi_status = self.register.control.generate_vblank_nmi();
        self.register.control.update(value);
    }

    fn write_to_mask(&mut self, value: u8) {
        self.register.mask.update(value);
    }

    fn read_status(&mut self) -> u8 {
        let data = self.register.status.snapshot();
        self.register.status.reset_vblank_status();
        self.register.address.reset_latch();
        self.register.scroll.reset_latch();
        data
    }

    fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_address = value;
    }

    fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_address as usize] = value;
        self.oam_address = self.oam_address.wrapping_add(1);
    }

    fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_address as usize]
    }

    fn write_to_scroll(&mut self, value: u8) {
        self.register.scroll.write(value);
    }

    fn write_to_ppu_addr(&mut self, value: u8) {
        self.register.address.update(value);
    }

    fn write_to_data(&mut self, value: u8) {
        let addr = self.register.address.get();
        match addr {
            0..=0x1fff => println!("attempt to write to chr rom space {}", addr),
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3eff => unimplemented!("addr {} shouldn't be used in reallity", addr),

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = value;
            }
            0x3f00..=0x3fff => {
                self.palette_table[(addr - 0x3f00) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
        self.increment_vram_addr();
    }

    fn read_data(&mut self) -> u8 {
        let addr = self.register.address.get();

        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buffer;
                self.internal_data_buffer = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buffer;
                self.internal_data_buffer = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => unimplemented!("addr {} shouldn't be used in reallity", addr),

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }

            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.oam_data[self.oam_address as usize] = *x;
            self.oam_address = self.oam_address.wrapping_add(1);
        }
    }
}

/// cpu通过总线内存访问与ppu通信，共暴露8个字节的寄存器
impl ReadableMut for Ppu {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0 | 1 | 3 | 5 | 6 => {
                panic!("Attempt to read from write-only PPU address {}", addr);
                // 0
            }
            2 => self.read_status(),
            4 => self.read_oam_data(),
            7 => self.read_data(),
            _ => panic!("Can't read ppu register: {}", addr),
        }
    }
}
impl Writable for Ppu {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0 => self.write_to_ctrl(data),
            1 => self.write_to_mask(data),
            2 => panic!("attempt to write to PPU status register"),
            3 => self.write_to_oam_addr(data),
            4 => self.write_to_oam_data(data),
            5 => self.write_to_scroll(data),
            6 => self.write_to_ppu_addr(data),
            7 => self.write_to_data(data),
            _ => panic!("Can't write ppu register: {}, data: {}", addr, data),
        }
    }
}

impl AddressableMut for Ppu {}
