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

pub struct ControlRegister {
    nametable_1: bool,
    nametable_2: bool,
    vram_address_increment: bool,
    sprite_pattern_address: bool,
    background_pattern_address: bool,
    sprite_size: bool,
    master_slave_select: bool,
    generate_vblank_nmi: bool,
}

impl From<u8> for ControlRegister {
    fn from(bits: u8) -> Self {
        Self {
            nametable_1: ((bits >> 0) & 1) != 0,
            nametable_2: ((bits >> 1) & 1) != 0,
            vram_address_increment: ((bits >> 2) & 1) != 0,
            sprite_pattern_address: ((bits >> 3) & 1) != 0,
            background_pattern_address: ((bits >> 4) & 1) != 0,
            sprite_size: ((bits >> 5) & 1) != 0,
            master_slave_select: ((bits >> 6) & 1) != 0,
            generate_vblank_nmi: ((bits >> 7) & 1) != 0,
        }
    }
}

impl Into<u8> for ControlRegister {
    fn into(self) -> u8 {
        let bit0 = self.nametable_1 as u8;
        let bit1 = self.nametable_2 as u8;
        let bit2 = self.vram_address_increment as u8;
        let bit3 = self.sprite_pattern_address as u8;
        let bit4 = self.background_pattern_address as u8;
        let bit5 = self.sprite_size as u8;
        let bit6 = self.master_slave_select as u8;
        let bit7 = self.generate_vblank_nmi as u8;
        let bits = [bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7];
        let mut result: u8 = 0;
        for (i, bit) in bits.iter().enumerate() {
            result = result | (bit << i);
        }
        result
    }
}

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

    pub fn update(mut self, data: u8) {
        self = ControlRegister::from(data);
    }
}
