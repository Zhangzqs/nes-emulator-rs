
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

impl FlagRegister for ControlRegister {
    fn update(&mut self, data: u8) {
        self.nametable_1 = ((data >> 0) & 1) != 0;
        self.nametable_2 = ((data >> 1) & 1) != 0;
        self.vram_address_increment = ((data >> 2) & 1) != 0;
        self.sprite_pattern_address = ((data >> 3) & 1) != 0;
        self.background_pattern_address = ((data >> 4) & 1) != 0;
        self.sprite_size = ((data >> 5) & 1) != 0;
        self.master_slave_select = ((data >> 6) & 1) != 0;
        self.generate_vblank_nmi = ((data >> 7) & 1) != 0;
    }
}

