
pub struct MaskRegister {
    is_grey_scale: bool,
    leftmost_8pxl_background: bool,
    leftmost_8pxl_sprite: bool,
    show_background: bool,
    show_sprite: bool,
    emphasise_red: bool,
    emphasise_green: bool,
    emphasise_blue: bool,
}

impl From<u8> for MaskRegister {
    fn from(bits: u8) -> Self {
        Self {
            is_grey_scale: ((bits >> 0) & 1) != 0,
            leftmost_8pxl_background: ((bits >> 1) & 1) != 0,
            leftmost_8pxl_sprite: ((bits >> 2) & 1) != 0,
            show_background: ((bits >> 3) & 1) != 0,
            show_sprite: ((bits >> 4) & 1) != 0,
            emphasise_red: ((bits >> 5) & 1) != 0,
            emphasise_green: ((bits >> 6) & 1) != 0,
            emphasise_blue: ((bits >> 7) & 1) != 0,
        }
    }
}

impl Into<u8> for MaskRegister {
    fn into(self) -> u8 {
        let bit0 = self.is_grey_scale as u8;
        let bit1 = self.leftmost_8pxl_background as u8;
        let bit2 = self.leftmost_8pxl_sprite as u8;
        let bit3 = self.show_background as u8;
        let bit4 = self.show_sprite as u8;
        let bit5 = self.emphasise_red as u8;
        let bit6 = self.emphasise_green as u8;
        let bit7 = self.emphasise_blue as u8;
        let bits = [bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7];
        let mut result: u8 = 0;
        for (i, bit) in bits.iter().enumerate() {
            result = result | (bit << i);
        }
        result
    }
}

impl FlagRegister for MaskRegister {
    fn update(&mut self, data: u8) {
        self.is_grey_scale = ((data >> 0) & 1) != 0;
        self.leftmost_8pxl_background = ((data >> 1) & 1) != 0;
        self.leftmost_8pxl_sprite = ((data >> 2) & 1) != 0;
        self.show_background = ((data >> 3) & 1) != 0;
        self.show_sprite = ((data >> 4) & 1) != 0;
        self.emphasise_red = ((data >> 5) & 1) != 0;
        self.emphasise_green = ((data >> 6) & 1) != 0;
        self.emphasise_blue = ((data >> 7) & 1) != 0;
    }
}

