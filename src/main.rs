use crate::bus::Addressable;
use crate::register::Register;
use crate::status::StatusFlagRegister;
use crate::AddressingMode::{ZeroPage, ZeroPageX};

mod bus;
mod register;
mod status;

#[derive(Debug)]
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

pub struct CPU {
    pub register: Register,
    pub bus: Box<dyn Addressable>,
}

/// 读写总线的便捷方法
impl CPU {
    fn read(&self, addr: u16) -> u8 {
        self.bus.read(addr)
    }
    fn read_u16(&self, addr: u16) -> u16 {
        self.bus.read_u16(addr)
    }
    fn write(&self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }
    fn write_u16(&self, addr: u16, data: u16) {
        self.bus.write_u16(addr, data);
    }
}

impl CPU {
    /// 获取当前的操作数的地址
    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        let pc = self.register.pc;
        let x = self.register.x;
        let y = self.register.y;
        let read = self.read;
        let read_u16 = self.read_u16;

        /// 此时寄存器pc的值为指令码地址的后一个地址
        match mode {
            AddressingMode::Immediate => pc,

            AddressingMode::ZeroPage => read(pc) as u16,
            AddressingMode::Absolute => read_u16(pc),

            AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY => {
                // 零页基地址
                let base = match mode {
                    AddressingMode::ZeroPageX | AddressingMode::ZeroPageY => read(pc) as u16,
                    AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => read_u16(pc),
                    _ => todo!(),
                };
                let x_or_y = match mode {
                    AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => x,
                    AddressingMode::ZeroPageY | AddressingMode::AbsoluteY => y,
                    _ => todo!(),
                };
                base.wrapping_add(x_or_y as u16)
            }
            AddressingMode::IndirectX => {
                /// X间接寻址(操作码，零页基地址)
                /// *(X+base) | *(X+base+1) << 8
                let base = read(pc);
                let ptr = base.wrapping_add(x);
                let low = read(ptr as u16);
                let high = read(ptr.wrapping_add(1) as u16);
                (low as u16) | ((high as u16) << 8)
            }
            AddressingMode::IndirectY => {
                /// Y间接寻址(操作码，零页间接地址)
                /// *(base) | *(base+1) << 8 + Y
                let base_ptr = read(pc);
                let low = read(base_ptr as u16);
                let high = read(base_ptr.wrapping_add(1) as u16);
                let base = (low as u16) | ((high as u16) << 8);
                base.wrapping_add(y as u16)
            }
            AddressingMode::NoneAddressing => panic!("mode {:?} is not supported", mode),
        }
    }

    /// 根据执行结果更新负数标志
    fn update_negative_flag(&mut self, result: u8) {
        /// 8位整数的最高位符号位为负数标志位
        self.register.status.negative = result >> 7 == 1;
    }
    /// 根据执行结果更新零标志
    fn update_zero_flag(&mut self, result: u8) {
        /// 是否为0
        self.register.status.zero = result == 0;
    }
    /// 根据执行结果更新零标志和负数标志
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }
}

/// 数据传送指令实现
impl CPU {
    fn load_register(&mut self, register_ref: &mut u8, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.read(addr);
        *register_ref = data;
        self.update_zero_and_negative_flags(*register_ref);
    }
    fn ldx(&mut self, mode: &AddressingMode) {
        self.load_register(&mut self.register.x, mode);
    }
    fn ldy(&mut self, mode: &AddressingMode) {
        self.load_register(&mut self.register.y, mode);
    }
    fn lda(&mut self, mode: &AddressingMode) {
        self.load_register(&mut self.register.a, mode);
    }
    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.write(addr, self.register.a);
    }
    fn stx() {}
    fn sty() {}
    fn tax() {}
    fn txa() {}
    fn tay() {}
    fn tya() {}
    fn txs() {}
    fn tsx() {}
}

/// 算术运算指令实现
impl CPU {
    fn adc() {}
    fn sbc() {}
    fn inc() {}
    fn dec() {}
    fn inx() {}
    fn dex() {}
    fn iny() {}
    fn dey() {}
}

/// 逻辑运算指令实现
impl CPU {
    fn and() {}
    fn ora() {}
    fn eor() {}
}

/// 置标志位指令实现
impl CPU {
    fn clc() {}
    fn sec() {}
    fn cld() {}
    fn sed() {}
    fn clv() {}
    fn cli() {}
    fn sei() {}
}

/// 比较指令实现
impl CPU {
    fn cmp() {}
    fn cpx() {}
    fn cpy() {}
    fn bit() {}
}

/// 移位指令
impl CPU {
    fn asl() {}
    fn lsr() {}
    fn rol() {}
    fn ror() {}
}

/// 堆栈指令
impl CPU {
    fn pha() {}
    fn pla() {}
    fn php() {}
    fn plp() {}
}

/// 跳转指令
impl CPU {
    fn jmp() {}
    fn beq() {}
    fn bne() {}
    fn bcs() {}
    fn bcc() {}
    fn bmi() {}
    fn bpl() {}
    fn bvs() {}
    fn bvc() {}
}

/// 中断指令
impl CPU {
    fn int() {}
}

fn main() {
    println!("Hello, world!");
}
