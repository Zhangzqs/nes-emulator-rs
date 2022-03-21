pub struct AddressRegister {
    value: u16,
    hi_ptr: bool,
}

impl AddressRegister {
    pub fn new() -> Self {
        AddressRegister {
            value: 0, // high byte first, lo byte second
            hi_ptr: true,
        }
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            // data赋值到高8位
            // 获取原数据的低8位，再拼接即可
            let low = self.value & 0xFF;
            let high = data as u16;
            self.value = (high << 8) | low
        } else {
            // data赋值到低8位
            self.value &= 0xFF;
            self.value |= data as u16;
        }
        if self.get() > 0x3FFF {
            self.set(self.get() & 0x3FFF);
        }
    }

    pub fn increment(&mut self, inc: u8) {
        self.value += inc as u16;
        if self.get() > 0x3FFF {
            self.set(self.get() & 0x3FFF);
        }
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }

    fn set(&mut self, data: u16) {
        self.value = data;
    }

    pub fn get(&self) -> u16 {
        self.value
    }
}
