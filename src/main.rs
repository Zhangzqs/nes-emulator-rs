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
pub struct StatusFlagRegister{
    /// 进位标志
    pub carry: bool,
    /// 零标志
    pub zero: bool,
    /// 禁用中断
    pub interrupt_disable: bool,
    /// 十进制模式(nes中不使用)
    pub decimal_mode: bool,
    ///
    pub break_command: bool,
    /// 溢出标志
    pub overflow: bool,
    /// 负数标志
    pub negative: bool,
}

pub struct Register{
    /// 寄存器A
    pub a: u8,
    /// 寄存器X
    pub x: u8,
    /// 寄存器Y
    pub y: u8,
    /// 标志位寄存器
    pub status: StatusFlagRegister,
    /// 程序计数器(program counter)
    pub pc: u16,
    /// 栈指针寄存器(stack pointer)
    pub sp: u8,
}

pub enum AddressingMode {
    /// 立即数寻址
    Immediate,
    /// 零页寻址
    ZeroPage,
    /// 零页X
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}


fn main() {
    println!("Hello, world!");
}
