// 7  bit  0
// ---- ----
// BGRs bMmG
// |||| ||||
// |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
// |||| +---- 1: Show background
// |||+------ 1: Show sprites
// ||+------- Emphasize red
// |+-------- Emphasize green
// +--------- Emphasize blue
pub enum Color {
    Red,
    Green,
    Blue,
}

use crate::flag_reg;
flag_reg!(
    MaskRegister,
    is_grey_scale,
    leftmost_8pxl_background,
    leftmost_8pxl_sprite,
    show_background,
    show_sprite,
    emphasise_red,
    emphasise_green,
    emphasise_blue
);
impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister::from(0)
    }
    pub fn emphasise(&self) -> Vec<Color> {
        let mut result = Vec::<Color>::new();
        if self.emphasise_red {
            result.push(Color::Red);
        }
        if self.emphasise_green {
            result.push(Color::Green);
        }
        if self.emphasise_blue {
            result.push(Color::Blue);
        }
        result
    }
}
