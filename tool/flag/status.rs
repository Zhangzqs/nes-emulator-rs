
pub struct StatusRegister {
    unused_1: bool,
    unused_2: bool,
    unused_3: bool,
    unused_4: bool,
    unused_5: bool,
    sprite_overflow: bool,
    sprite_zero_hit: bool,
    vblank_started: bool,
}

impl From<u8> for StatusRegister {
    fn from(bits: u8) -> Self {
        Self {
            unused_1: ((bits >> 0) & 1) != 0,
            unused_2: ((bits >> 1) & 1) != 0,
            unused_3: ((bits >> 2) & 1) != 0,
            unused_4: ((bits >> 3) & 1) != 0,
            unused_5: ((bits >> 4) & 1) != 0,
            sprite_overflow: ((bits >> 5) & 1) != 0,
            sprite_zero_hit: ((bits >> 6) & 1) != 0,
            vblank_started: ((bits >> 7) & 1) != 0,
        }
    }
}

impl Into<u8> for StatusRegister {
    fn into(self) -> u8 {
        let bit0 = self.unused_1 as u8;
        let bit1 = self.unused_2 as u8;
        let bit2 = self.unused_3 as u8;
        let bit3 = self.unused_4 as u8;
        let bit4 = self.unused_5 as u8;
        let bit5 = self.sprite_overflow as u8;
        let bit6 = self.sprite_zero_hit as u8;
        let bit7 = self.vblank_started as u8;
        let bits = [bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7];
        let mut result: u8 = 0;
        for (i, bit) in bits.iter().enumerate() {
            result = result | (bit << i);
        }
        result
    }
}

impl FlagRegister for StatusRegister {
    fn update(&mut self, data: u8) {
        self.unused_1 = ((data >> 0) & 1) != 0;
        self.unused_2 = ((data >> 1) & 1) != 0;
        self.unused_3 = ((data >> 2) & 1) != 0;
        self.unused_4 = ((data >> 3) & 1) != 0;
        self.unused_5 = ((data >> 4) & 1) != 0;
        self.sprite_overflow = ((data >> 5) & 1) != 0;
        self.sprite_zero_hit = ((data >> 6) & 1) != 0;
        self.vblank_started = ((data >> 7) & 1) != 0;
    }
}

