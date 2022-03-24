use crate::flag_reg;

flag_reg!(
    StatusRegister,
    unused_1,
    unused_2,
    unused_3,
    unused_4,
    unused_5,
    sprite_overflow,
    sprite_zero_hit,
    vblank_started
);

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister::from(0)
    }

    pub fn snapshot(&self) -> u8 {
        self.clone().into()
    }
    pub fn reset_vblank_status(&mut self) {
        self.vblank_started = false;
    }
}
