use crate::meta::Mirror;
mod register;
// NES的分辨率为256x240

pub struct PPU {
    /// 卡带上的数据
    pub chr_rom: Vec<u8>,
    /// 调色板
    pub palette_table: [u8; 32],
    /// 背景信息
    pub vram: [u8; 2048],
    /// 精灵数据
    pub oam_data: [u8; 256],
    pub mirror: Mirror,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirror: Mirror) -> Self {
        Self {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            mirror,
        }
    }
}
