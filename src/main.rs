use crate::bus::Addressable;
use crate::register::Register;
use crate::status::StatusFlagRegister;
use crate::AddressingMode::{ZeroPage, ZeroPageX};

mod bus;
mod memory;
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
    /// LDA--由存储器取数送入累加器 M→A
    fn lda(&mut self, mode: &AddressingMode) {
        self.load_register(&mut self.register.a, mode);
    }
    /// LDX--由存储器取数送入累加器 M→X
    fn ldx(&mut self, mode: &AddressingMode) {
        self.load_register(&mut self.register.x, mode);
    }
    /// LDY--由存储器取数送入累加器 M→Y
    fn ldy(&mut self, mode: &AddressingMode) {
        self.load_register(&mut self.register.y, mode);
    }
    /// STA--将累加器的内容送入存储器 A--M
    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.write(addr, self.register.a);
    }
    /// STX--将寄存器X的内容送入存储器 X--M
    fn stx(&mut self) {
        let addr = self.get_operand_address(mode);
        self.write(addr, self.register.a);
    }
    /// STY--将寄存器Y的内容送入存储器 Y--M
    fn sty(&mut self) {
        let addr = self.get_operand_address(mode);
        self.write(addr, self.register.a);
    }

    fn tax(&mut self) {}
    fn txa(&mut self) {}
    fn tay(&mut self) {}
    fn tya(&mut self) {}
    fn txs(&mut self) {}
    fn tsx(&mut self) {}
}

/// 算术运算指令实现
impl CPU {
    fn adc(&mut self) {}
    fn sbc(&mut self) {}
    fn inc(&mut self) {}
    fn dec(&mut self) {}
    fn inx(&mut self) {}
    fn dex(&mut self) {}
    fn iny(&mut self) {}
    fn dey(&mut self) {}
}

/// 逻辑运算指令实现
impl CPU {
    fn and(&mut self) {}
    fn ora(&mut self) {}
    fn eor(&mut self) {}
}

/// 置标志位指令实现
impl CPU {
    fn clc(&mut self) {}
    fn sec(&mut self) {}
    fn cld(&mut self) {}
    fn sed(&mut self) {}
    fn clv(&mut self) {}
    fn cli(&mut self) {}
    fn sei(&mut self) {}
}

/// 比较指令实现
impl CPU {
    fn cmp(&mut self) {}
    fn cpx(&mut self) {}
    fn cpy(&mut self) {}
    fn bit(&mut self) {}
}

/// 移位指令
impl CPU {
    fn asl(&mut self) {}
    fn lsr(&mut self) {}
    fn rol(&mut self) {}
    fn ror(&mut self) {}
}

/// 堆栈指令
impl CPU {
    fn pha(&mut self) {}
    fn pla(&mut self) {}
    fn php(&mut self) {}
    fn plp(&mut self) {}
}

/// 跳转指令
impl CPU {
    fn jmp(&mut self) {}
    fn beq(&mut self) {}
    fn bne(&mut self) {}
    fn bcs(&mut self) {}
    fn bcc(&mut self) {}
    fn bmi(&mut self) {}
    fn bpl(&mut self) {}
    fn bvs(&mut self) {}
    fn bvc(&mut self) {}
}

/// 中断指令
impl CPU {
    fn int(&mut self) {}
}

fn main() {
    println!("Hello, world!");
}
