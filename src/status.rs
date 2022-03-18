/// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
///
///  7 6 5 4 3 2 1 0
///  N V _ B D I Z C
///  | |   | | | | +--- Carry Flag
///  | |   | | | +----- Zero Flag
///  | |   | | +------- Interrupt Disable
///  | |   | +--------- Decimal Mode (not used on NES)
///  | |   +----------- Break Command
///  | +--------------- Overflow Flag
///  +----------------- Negative Flag
pub struct StatusFlagRegister {
    /// 进位标志(一般对于无符号数来说)，如果最近一条指令有溢出——上溢：超出了 255，下溢：低于 0，则设置该 bit 为 1，
    /// 比如说执行 255 + 1 会上溢，将 Carry Flag 置 1。
    /// 有了 Carry Flag，使得可以进行长度超过 8 位的运算。
    pub carry: bool,
    /// 最近一条指令的结果是否为 0，如果是，则置 1，否则清零
    pub zero: bool,
    /// 置 1 会使得系统忽略 IRQ 中断，清零则响应，
    /// SEI(Set Interrupt Disable) 指令将该位置 1，
    /// CLI(Clear Interrupt Disable)将该位清 0
    pub interrupt_disable: bool,
    /// 该位用来将 6502 切换到 BCD 模式，但 NES 里面不用
    pub decimal_mode: bool,
    /// 该位用来表示一个 BRK(Break) 指令在执行，该指令就是发出一个 IRQ 中断，类似 x86 下的 INT
    pub break_command: bool,
    /// 未使用寄存器
    pub unused: bool,
    /// 溢出标志(一般对于有符号数来说)，如果运算有溢出，则置 1
    pub overflow: bool,
    /// 该位就是运算结果的最高位
    pub negative: bool,
}

impl From<u8> for StatusFlagRegister {
    fn from(bits: u8) -> Self {
        Self {
            carry: ((bits >> 0) & 1) != 0,
            zero: ((bits >> 1) & 1) != 0,
            interrupt_disable: ((bits >> 2) & 1) != 0,
            decimal_mode: ((bits >> 1) & 3) != 0,
            break_command: ((bits >> 1) & 4) != 0,
            unused: ((bits >> 1) & 5) != 0,
            overflow: ((bits >> 1) & 6) != 0,
            negative: ((bits >> 1) & 7) != 0,
        }
    }
}

impl Into<u8> for StatusFlagRegister {
    fn into(self) -> u8 {
        let bit0 = self.carry as u8;
        let bit1 = self.zero as u8;
        let bit2 = self.interrupt_disable as u8;
        let bit3 = self.decimal_mode as u8;
        let bit4 = self.break_command as u8;
        let bit5 = self.unused as u8;
        let bit6 = self.overflow as u8;
        let bit7 = self.negative as u8;
        let bits = [bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7];
        let mut result: u8 = 0;
        for (i, bit) in bits.iter().enumerate() {
            result = result | (bit << i);
        }
        result
    }
}

impl Default for StatusFlagRegister {
    fn default() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt_disable: true,
            decimal_mode: false,
            break_command: false,
            unused: true,
            overflow: false,
            negative: false,
        }
    }
}
