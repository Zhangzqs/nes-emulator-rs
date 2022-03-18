mod bus;

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
    /// 立即数寻址(操作码，操作数)
    Immediate,
    /// 零页寻址(操作码，零页地址(零页即0x00~0xFF))
    ZeroPage,
    /// 零页X寻址(操作码，零页基地址)
    ZeroPageX,
    /// 零页Y寻址(操作码，零页基地址)
    ZeroPageY,
    /// 绝对寻址(操作码，操作数地址低字节，操作数地址高字节)
    Absolute,
    /// 绝对X寻址(操作码，基地址低字节，基地址高字节)
    AbsoluteX,
    /// 绝对Y寻址(操作码，基地址低字节，基地址高字节)
    AbsoluteY,
    /// X间接寻址(操作码，零页基地址)
    /// *(X+base) | *(X+base+1) << 8
    IndirectX,
    /// Y间接寻址(操作码，零页间接地址)
    /// *(base) | *(base+1) << 8 + Y
    IndirectY,
    /// 无效寻址
    NoneAddressing,
}


fn main() {
    println!("Hello, world!");
}
