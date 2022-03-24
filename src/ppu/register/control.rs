// 7  bit  0
// ---- ----
// VPHB SINN
// |||| ||||
// |||| ||++- Base nametable address
// |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
// |||| |+--- VRAM address increment per CPU read/write of PPUDATA
// |||| |     (0: add 1, going across; 1: add 32, going down)
// |||| +---- Sprite pattern table address for 8x8 sprites
// ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
// |||+------ Background pattern table address (0: $0000; 1: $1000)
// ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
// |+-------- PPU master/slave select
// |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
// +--------- Generate an NMI at the start of the
//            vertical blanking interval (0: off; 1: on)

use crate::flag_reg;

flag_reg!(
    ControlRegister,
    nametable_1,
    nametable_2,
    vram_address_increment,
    sprite_pattern_address,
    background_pattern_address,
    sprite_size,
    master_slave_select,
    generate_vblank_nmi
);

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from(0x00)
    }

    pub fn nametable_address(&self) -> u16 {
        match (self.nametable_1, self.nametable_2) {
            (false, false) => 0x2000,
            (true, false) => 0x2400,
            (false, true) => 0x2800,
            (true, true) => 0x2c00,
        }
    }

    pub fn vram_address_increment(&self) -> u8 {
        if self.vram_address_increment {
            32
        } else {
            1
        }
    }

    pub fn sprite_pattern_address(&self) -> u16 {
        if self.sprite_pattern_address {
            0x1000
        } else {
            0
        }
    }

    pub fn background_pattern_address(&self) -> u16 {
        if self.background_pattern_address {
            0x1000
        } else {
            0
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if self.sprite_size {
            16
        } else {
            8
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        if self.master_slave_select {
            1
        } else {
            0
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        self.generate_vblank_nmi
    }
}
