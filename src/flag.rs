pub trait FlagRegister: From<Self> + Into<u8> {
    fn update(&mut self, data: u8);
}

#[macro_export]
macro_rules! flag_reg {
    ($sn:ident,$fn0:ident,$fn1:ident,$fn2:ident,$fn3:ident,$fn4:ident,$fn5:ident,$fn6:ident,$fn7:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $sn {
            pub $fn0: bool,
            pub $fn1: bool,
            pub $fn2: bool,
            pub $fn3: bool,
            pub $fn4: bool,
            pub $fn5: bool,
            pub $fn6: bool,
            pub $fn7: bool,
        }

        impl From<u8> for $sn {
            fn from(bits: u8) -> Self {
                Self {
                    $fn0: ((bits >> 0) & 1) != 0,
                    $fn1: ((bits >> 1) & 1) != 0,
                    $fn2: ((bits >> 2) & 1) != 0,
                    $fn3: ((bits >> 3) & 1) != 0,
                    $fn4: ((bits >> 4) & 1) != 0,
                    $fn5: ((bits >> 5) & 1) != 0,
                    $fn6: ((bits >> 6) & 1) != 0,
                    $fn7: ((bits >> 7) & 1) != 0,
                }
            }
        }
        impl Into<u8> for $sn {
            fn into(self) -> u8 {
                let bit0 = self.$fn0 as u8;
                let bit1 = self.$fn1 as u8;
                let bit2 = self.$fn2 as u8;
                let bit3 = self.$fn3 as u8;
                let bit4 = self.$fn4 as u8;
                let bit5 = self.$fn5 as u8;
                let bit6 = self.$fn6 as u8;
                let bit7 = self.$fn7 as u8;
                let bits = [bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7];
                let mut result: u8 = 0;
                for (i, bit) in bits.iter().enumerate() {
                    result = result | (bit << i);
                }
                result
            }
        }
        use crate::flag::FlagRegister;
        impl FlagRegister for $sn {
            fn update(&mut self, data: u8) {
                self.$fn0 = ((data >> 0) & 1) != 0;
                self.$fn1 = ((data >> 1) & 1) != 0;
                self.$fn2 = ((data >> 2) & 1) != 0;
                self.$fn3 = ((data >> 3) & 1) != 0;
                self.$fn4 = ((data >> 4) & 1) != 0;
                self.$fn5 = ((data >> 5) & 1) != 0;
                self.$fn6 = ((data >> 6) & 1) != 0;
                self.$fn7 = ((data >> 7) & 1) != 0;
            }
        }
    };
}
