use crate::bus::Addressable;
use crate::register::Register;
use crate::status::StatusFlagRegister;
use crate::AddressingMode::{ZeroPage, ZeroPageX};

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

/// 两字节打包成u16
fn pack_u16(high: u8, low: u8) -> u16 {
    let high = high as u16;
    let low = low as u16;
    (high << 8) | low
}
/// 解构u16(high,low)
fn unpack_u16(val: u16) -> (u8, u8) {
    let low = (val & 0xff) as u8;
    let high = (val >> 8) as u8;
    (high, low)
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
    /// 获取当前的操作数
    fn get_operand(&self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        self.read(addr)
    }
    fn get_operand_u16(&self, mode: &AddressingMode) -> u16 {
        let addr = self.get_operand_address(mode);
        self.read_u16(addr)
    }
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
}

/// 更新标志位
impl CPU {
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
    /// 根据结果更新进位标志
    fn update_carry_flag(&mut self, result: u16) {
        self.register.status.carry = result > 0xFF;
    }
}

impl CPU {
    fn set_register_a(&mut self, value: u8) {
        self.register.a = value;
        self.update_zero_and_negative_flags(self.register.a);
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
        self.write(addr, self.register.x);
    }
    /// STY--将寄存器Y的内容送入存储器 Y--M
    fn sty(&mut self) {
        let addr = self.get_operand_address(mode);
        self.write(addr, self.register.y);
    }

    /// 将源寄存器的值传递到目的寄存器
    fn transport_register(&mut self, src: u8, dist: &mut u8) {
        *dist = src;
        self.update_zero_and_negative_flags(*dist);
    }
    /// 将累加器A的内容送入变址寄存器X
    fn tax(&mut self) {
        self.transport_register(self.register.a, &mut self.register.x)
    }
    /// 将变址寄存器X的内容送入累加器A
    fn txa(&mut self) {
        self.transport_register(self.register.x, &mut self.register.a)
    }
    /// 将累加器A的内容送入变址寄存器Y
    fn tay(&mut self) {
        self.transport_register(self.register.a, &mut self.register.y)
    }
    ///	将变址寄存器Y的内容送入累加器A
    fn tya(&mut self) {
        self.transport_register(self.register.y, &mut self.register.a)
    }
    /// 将变址寄存器X的内容送入堆栈指针S
    fn txs(&mut self) {
        self.transport_register(self.register.x, &mut self.register.sp)
    }
    /// 将堆栈指针S的内容送入变址寄存器X
    fn tsx(&mut self) {
        self.transport_register(self.register.sp, &mut self.register.x)
    }
}

/// 算术运算指令实现
impl CPU {
    /// 向累加器A添加一个数
    fn add_to_reg_a(&mut self, data: u8) {
        let a = self.register.a as u16;
        let data = data as u16;
        let carry = self.register.status.carry as u16;
        let sum = a + data + carry;
        self.update_carry_flag(sum);

        let result = sum as u8;
        // TODO update overflow flag
        self.set_register_a(result);
    }
    /// ADC--累加器,存储器,进位标志C相加,结果送累加器A  A+M+C→A
    fn adc(&mut self, mode: &AddressingMode) {
        let val = self.get_operand(mode);
        self.add_to_reg_a(val);
    }
    /// SBC--从累加器减去存储器和进位标志C,结果送累加器  A-M-C→A
    fn sbc(&mut self, mode: &AddressingMode) {
        let val = self.get_operand(mode);
        let data = val as i8;
        // add_to_reg_a函数内部完成了累加器与进位标志相加
        // self.add_to_reg_a()
    }
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
    /// 清除进位标志
    fn clc(&mut self) {
        self.register.status.carry = false;
    }
    /// 置进位标志C  
    fn sec(&mut self) {
        self.register.status.carry = true;
    }
    /// 清除十进制运算标志D
    fn cld(&mut self) {
        self.register.status.decimal_mode = false;
    }
    /// 置十进制运算标志D
    fn sed(&mut self) {
        self.register.status.decimal_mode = true;
    }
    /// 清除溢出标志V
    fn clv(&mut self) {
        self.register.status.overflow = false;
    }
    /// 清除中断禁止指令I
    fn cli(&mut self) {
        self.register.status.interrupt_disable = false;
    }
    /// 置位中断禁止标志I
    fn sei(&mut self) {
        self.register.status.interrupt_disable = true;
    }
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
    fn asl_reg_a(&mut self) {
        let data = (self.register.a as u16) << 1;
        self.update_carry_flag(data);
        self.set_register_a(data as u8);
    }
    /// 算术左移指令ASL
    /// ASL移位功能是将字节内各位依次向左移1位，最高位移进标志位C中，最底位补0
    fn asl_memory(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode) as u16;
        let data = (self.read(addr) as u16) << 1;
        self.update_carry_flag(data);
        let data = data as u8;
        self.write(addr, data);
        self.update_zero_and_negative_flags(data);
        data
    }
    /// 逻辑右移指令LSR
    /// 该指令功能是将字节内各位依次向右移1位，最低位移进标志位C，最高位补0.
    fn lsr_memory(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode) as u16;
        let data = self.read(addr);
        self.register.status.carry = data & 1 == 1;
        let data = data >> 1;
        self.write(addr, data);
        self.update_zero_and_negative_flags(data);
        data
    }
    fn lsr_reg_a(&mut self) {
        let data = self.register.a;
        self.register.status.carry = data & 1 == 1;
        let data = data >> 1;
        self.set_register_a(data);
    }
    /// 循环左移指令ROL
    /// ROL的移位功能是将字节内容连同进位C一起依次向左移1位
    fn rol_memory(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode) as u16;
        let data = self.read(addr);
        let old_carry = self.register.status.carry;
        self.register.status.carry = data >> 7 == 1;
        let data = (data << 1) | (old_carry as u8);
        self.write(addr, data);
        self.update_negative_flag(data);
        data
    }
    fn rol_reg_a(&mut self) {
        let data = self.register.a;
        let old_carry = self.register.status.carry;
        self.register.status.carry = data >> 7 == 1;
        let data = (data << 1) | (old_carry as u8);
        self.set_register_a(data);
    }
    /// 循环右移指令ROR
    /// ROR的移位功能是将字节内容连同进位C一起依次向右移1位
    fn ror_memory(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode) as u16;
        let data = self.read(addr);
        let old_carry = self.register.status.carry;
        self.register.status.carry = data & 1 == 1;
        let data = (data >> 1) | ((old_carry as u8) << 7);
        self.write(addr, data);
        self.update_negative_flag(data);
        data
    }

    fn ror_reg_a(&mut self) {
        let data = self.register.a;
        let old_carry = self.register.status.carry;
        self.register.status.carry = data & 1 == 1;
        let data = (data >> 1) | ((old_carry as u8) << 7);
        self.set_register_a(data);
    }
}

/// 堆栈控制函数
/// 栈的push内存地址由高到低
/// 栈的pop内存地址由低到高
const STACK_START: u16 = 0x0100;
impl CPU {
    fn stack_pop(&mut self) -> u8 {
        self.register.sp = self.register.sp.wrapping_add(1);
        self.read(STACK_START + self.register.sp)
    }
    fn stack_push(&mut self, data: u8) {
        self.write(STACK_START + self.register.sp, data);
        self.register.sp = self.register.sp.wrapping_sub(1)
    }
    fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop();
        let high = self.stack_pop();
        pack_u16(high, low)
    }
    fn stack_push_u16(&mut self, data: u16) {
        let (high, low) = unpack_u16(data);
        self.stack_push(high);
        self.stack_push(low);
    }
}

/// 堆栈指令
impl CPU {
    /// 累加器进栈指令 PHA
    /// PHA是隐含寻址方式的单字节指令，操作码是 48
    /// 功能是把累加器A的内容按堆栈指针S所指示的位置送入堆栈，然后堆栈指针减1
    /// 该指令不影响标志寄存器的状态
    fn pha(&mut self) {
        self.stack_push(self.register.a)
    }
    /// 累加器出栈指令 PLA
    /// PLA是隐含寻址方式的单字节指令，操作码是 68
    /// 功能是先让堆栈指针S+1，然后取加过1的S所指向的单元的内容，把它送累加器A
    /// 该指令影响标志寄存器P中的N，Z两标志位
    fn pla(&mut self) {
        let data = self.stack_pop();
        self.set_register_a(data);
    }
    /// 标志寄存器P进栈指令 PHP
    /// PHP是隐含寻址方式的单字节指令，操作码是 08
    /// 功能是把标志寄存器P的内容按堆栈指针S所指示的位置送入堆栈，然后堆栈指针减1
    /// 该指令不影响标志寄存器P的状态
    fn php(&mut self) {
        let mut status = self.register.status.clone();
        status.break_command = true;
        status.unused = true;
        self.stack_push(status.into())
    }
    /// PLP是隐含寻址方式的单字节指令，操作码是 28
    /// 功能是先让堆栈指针S+1，然后取加过1的S所指向的单元的内容，把它送标志寄存器P
    fn plp(&mut self) {
        let flags = self.stack_pop();
        self.register.status = StatusFlagRegister::from(flags);
        self.register.status.break_command = false;
        self.register.status.unused = true;
    }
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
