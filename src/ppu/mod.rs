use crate::{addressable::Addressable, meta::Mirror};

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

impl Addressable for Ppu {
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }
}
