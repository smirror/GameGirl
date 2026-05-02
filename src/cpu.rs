use crate::bus::Bus;
use crate::memory_map::{DMG_ENTRY_POINT, DMG_STACK_POINTER, IO_REGISTERS_START};

pub const FLAG_Z: u8 = 0x80;
pub const FLAG_N: u8 = 0x40;
pub const FLAG_H: u8 = 0x20;
pub const FLAG_C: u8 = 0x10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    f: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new_dmg() -> Self {
        Self {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: DMG_STACK_POINTER,
            pc: DMG_ENTRY_POINT,
        }
    }

    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f])
    }

    pub fn set_af(&mut self, value: u16) {
        let [a, f] = value.to_be_bytes();
        self.a = a;
        self.set_f(f);
    }

    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn set_bc(&mut self, value: u16) {
        let [b, c] = value.to_be_bytes();
        self.b = b;
        self.c = c;
    }

    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn set_de(&mut self, value: u16) {
        let [d, e] = value.to_be_bytes();
        self.d = d;
        self.e = e;
    }

    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn set_hl(&mut self, value: u16) {
        let [h, l] = value.to_be_bytes();
        self.h = h;
        self.l = l;
    }

    pub fn flag(&self, mask: u8) -> bool {
        self.f & mask != 0
    }

    pub fn f(&self) -> u8 {
        self.f
    }

    pub fn set_flag(&mut self, mask: u8, enabled: bool) {
        if enabled {
            self.set_f(self.f | mask);
        } else {
            self.set_f(self.f & !mask);
        }
    }

    pub fn set_f(&mut self, value: u8) {
        self.f = value & 0xF0;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cpu {
    pub registers: Registers,
    pub halted: bool,
    pub stopped: bool,
}

impl Cpu {
    pub fn new_dmg() -> Self {
        Self {
            registers: Registers::new_dmg(),
            halted: false,
            stopped: false,
        }
    }

    pub fn step(&mut self, bus: &mut Bus) -> Result<StepResult, CpuError> {
        let pc = self.registers.pc;
        let opcode = bus.read8(pc);

        let machine_cycles = match opcode {
            0x00 | 0x10 | 0x76 => self.step_misc(pc, opcode),
            0x01
            | 0x02
            | 0x06
            | 0x0A
            | 0x0E
            | 0x11
            | 0x12
            | 0x16
            | 0x1A
            | 0x1E
            | 0x21
            | 0x22
            | 0x26
            | 0x2A
            | 0x2E
            | 0x31
            | 0x32
            | 0x36
            | 0x3A
            | 0x3E
            | 0x40..=0x75
            | 0x77..=0x7F
            | 0xE0
            | 0xEA
            | 0xF0
            | 0xFA => self.step_load(bus, pc, opcode),
            0x04
            | 0x05
            | 0x0C
            | 0x0D
            | 0x14
            | 0x15
            | 0x1C
            | 0x1D
            | 0x24
            | 0x25
            | 0x2C
            | 0x2D
            | 0x34
            | 0x35
            | 0x3C
            | 0x3D
            | 0x80..=0x87
            | 0x90..=0x97
            | 0xA0..=0xA7
            | 0xA8..=0xAF
            | 0xB0..=0xB7
            | 0xB8..=0xBF
            | 0xC6
            | 0xD6
            | 0xE6
            | 0xEE
            | 0xF6
            | 0xFE => self.step_alu(bus, pc, opcode),
            0xC1 | 0xC5 | 0xD1 | 0xD5 | 0xE5 | 0xF1 | 0xF5 => self.step_stack(bus, pc, opcode),
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0xC0 | 0xC2 | 0xC3 | 0xC4 | 0xC7 | 0xC8 | 0xC9
            | 0xCA | 0xCB | 0xCC | 0xCD | 0xCF | 0xD0 | 0xD2 | 0xD4 | 0xD7 | 0xD8 | 0xDA | 0xDC
            | 0xDF | 0xE7 | 0xE9 | 0xEF | 0xF7 | 0xFF => self.step_control(bus, pc, opcode)?,
            _ => return Err(CpuError::UnsupportedOpcode { opcode, pc }),
        };

        Ok(StepResult { machine_cycles })
    }

    fn step_misc(&mut self, pc: u16, opcode: u8) -> u8 {
        match opcode {
            0x00 => self.advance(pc, 1, 1),
            0x10 => {
                self.stopped = true;
                self.advance(pc, 2, 1)
            }
            0x76 => {
                self.halted = true;
                self.stopped = false;
                self.advance(pc, 1, 1)
            }
            _ => unreachable!("opcode family dispatch should select only misc opcodes"),
        }
    }

    fn step_load(&mut self, bus: &mut Bus, pc: u16, opcode: u8) -> u8 {
        match opcode {
            0x01 => {
                let value = self.read_immediate16(bus, pc);
                self.registers.set_bc(value);
                self.advance(pc, 3, 3)
            }
            0x02 => {
                bus.write8(self.registers.bc(), self.registers.a);
                self.advance(pc, 1, 2)
            }
            0x06 => {
                self.registers.b = self.read_immediate8(bus, pc);
                self.advance(pc, 2, 2)
            }
            0x0A => {
                self.registers.a = bus.read8(self.registers.bc());
                self.advance(pc, 1, 2)
            }
            0x0E => {
                self.registers.c = self.read_immediate8(bus, pc);
                self.advance(pc, 2, 2)
            }
            0x11 => {
                let value = self.read_immediate16(bus, pc);
                self.registers.set_de(value);
                self.advance(pc, 3, 3)
            }
            0x12 => {
                bus.write8(self.registers.de(), self.registers.a);
                self.advance(pc, 1, 2)
            }
            0x16 => {
                self.registers.d = self.read_immediate8(bus, pc);
                self.advance(pc, 2, 2)
            }
            0x1A => {
                self.registers.a = bus.read8(self.registers.de());
                self.advance(pc, 1, 2)
            }
            0x1E => {
                self.registers.e = self.read_immediate8(bus, pc);
                self.advance(pc, 2, 2)
            }
            0x21 => {
                let value = self.read_immediate16(bus, pc);
                self.registers.set_hl(value);
                self.advance(pc, 3, 3)
            }
            0x22 => {
                let hl = self.registers.hl();
                bus.write8(hl, self.registers.a);
                self.registers.set_hl(hl.wrapping_add(1));
                self.advance(pc, 1, 2)
            }
            0x26 => {
                self.registers.h = self.read_immediate8(bus, pc);
                self.advance(pc, 2, 2)
            }
            0x2A => {
                let hl = self.registers.hl();
                self.registers.a = bus.read8(hl);
                self.registers.set_hl(hl.wrapping_add(1));
                self.advance(pc, 1, 2)
            }
            0x2E => {
                self.registers.l = self.read_immediate8(bus, pc);
                self.advance(pc, 2, 2)
            }
            0x31 => {
                self.registers.sp = self.read_immediate16(bus, pc);
                self.advance(pc, 3, 3)
            }
            0x32 => {
                let hl = self.registers.hl();
                bus.write8(hl, self.registers.a);
                self.registers.set_hl(hl.wrapping_sub(1));
                self.advance(pc, 1, 2)
            }
            0x36 => {
                let value = self.read_immediate8(bus, pc);
                bus.write8(self.registers.hl(), value);
                self.advance(pc, 2, 3)
            }
            0x3A => {
                let hl = self.registers.hl();
                self.registers.a = bus.read8(hl);
                self.registers.set_hl(hl.wrapping_sub(1));
                self.advance(pc, 1, 2)
            }
            0x3E => {
                self.registers.a = self.read_immediate8(bus, pc);
                self.advance(pc, 2, 2)
            }
            0x40..=0x75 | 0x77..=0x7F => self.load_register(bus, pc, opcode),
            0xE0 => {
                let offset = u16::from(self.read_immediate8(bus, pc));
                bus.write8(IO_REGISTERS_START + offset, self.registers.a);
                self.advance(pc, 2, 3)
            }
            0xEA => {
                let address = self.read_immediate16(bus, pc);
                bus.write8(address, self.registers.a);
                self.advance(pc, 3, 4)
            }
            0xF0 => {
                let offset = u16::from(self.read_immediate8(bus, pc));
                self.registers.a = bus.read8(IO_REGISTERS_START + offset);
                self.advance(pc, 2, 3)
            }
            0xFA => {
                let address = self.read_immediate16(bus, pc);
                self.registers.a = bus.read8(address);
                self.advance(pc, 3, 4)
            }
            _ => unreachable!("opcode family dispatch should select only load opcodes"),
        }
    }

    fn step_alu(&mut self, bus: &mut Bus, pc: u16, opcode: u8) -> u8 {
        match opcode {
            0x04 => {
                self.registers.b = self.inc8(self.registers.b);
                self.advance(pc, 1, 1)
            }
            0x05 => {
                self.registers.b = self.dec8(self.registers.b);
                self.advance(pc, 1, 1)
            }
            0x0C => {
                self.registers.c = self.inc8(self.registers.c);
                self.advance(pc, 1, 1)
            }
            0x0D => {
                self.registers.c = self.dec8(self.registers.c);
                self.advance(pc, 1, 1)
            }
            0x14 => {
                self.registers.d = self.inc8(self.registers.d);
                self.advance(pc, 1, 1)
            }
            0x15 => {
                self.registers.d = self.dec8(self.registers.d);
                self.advance(pc, 1, 1)
            }
            0x1C => {
                self.registers.e = self.inc8(self.registers.e);
                self.advance(pc, 1, 1)
            }
            0x1D => {
                self.registers.e = self.dec8(self.registers.e);
                self.advance(pc, 1, 1)
            }
            0x24 => {
                self.registers.h = self.inc8(self.registers.h);
                self.advance(pc, 1, 1)
            }
            0x25 => {
                self.registers.h = self.dec8(self.registers.h);
                self.advance(pc, 1, 1)
            }
            0x2C => {
                self.registers.l = self.inc8(self.registers.l);
                self.advance(pc, 1, 1)
            }
            0x2D => {
                self.registers.l = self.dec8(self.registers.l);
                self.advance(pc, 1, 1)
            }
            0x34 => {
                let hl = self.registers.hl();
                let value = self.inc8(bus.read8(hl));
                bus.write8(hl, value);
                self.advance(pc, 1, 3)
            }
            0x35 => {
                let hl = self.registers.hl();
                let value = self.dec8(bus.read8(hl));
                bus.write8(hl, value);
                self.advance(pc, 1, 3)
            }
            0x3C => {
                self.registers.a = self.inc8(self.registers.a);
                self.advance(pc, 1, 1)
            }
            0x3D => {
                self.registers.a = self.dec8(self.registers.a);
                self.advance(pc, 1, 1)
            }
            0x80..=0x87 => {
                let value = self.read_register(bus, opcode & 0x07);
                self.add(value);
                self.advance(pc, 1, register_operand_cycles(opcode, 0x86))
            }
            0x90..=0x97 => {
                let value = self.read_register(bus, opcode & 0x07);
                self.sub(value);
                self.advance(pc, 1, register_operand_cycles(opcode, 0x96))
            }
            0xA0..=0xA7 => {
                let value = self.read_register(bus, opcode & 0x07);
                self.and(value);
                self.advance(pc, 1, register_operand_cycles(opcode, 0xA6))
            }
            0xA8..=0xAF => {
                let value = self.read_register(bus, opcode & 0x07);
                self.xor(value);
                self.advance(pc, 1, register_operand_cycles(opcode, 0xAE))
            }
            0xB0..=0xB7 => {
                let value = self.read_register(bus, opcode & 0x07);
                self.or(value);
                self.advance(pc, 1, register_operand_cycles(opcode, 0xB6))
            }
            0xB8..=0xBF => {
                let value = self.read_register(bus, opcode & 0x07);
                self.cp(value);
                self.advance(pc, 1, register_operand_cycles(opcode, 0xBE))
            }
            0xC6 => {
                let value = self.read_immediate8(bus, pc);
                self.add(value);
                self.advance(pc, 2, 2)
            }
            0xD6 => {
                let value = self.read_immediate8(bus, pc);
                self.sub(value);
                self.advance(pc, 2, 2)
            }
            0xE6 => {
                let value = self.read_immediate8(bus, pc);
                self.and(value);
                self.advance(pc, 2, 2)
            }
            0xEE => {
                let value = self.read_immediate8(bus, pc);
                self.xor(value);
                self.advance(pc, 2, 2)
            }
            0xF6 => {
                let value = self.read_immediate8(bus, pc);
                self.or(value);
                self.advance(pc, 2, 2)
            }
            0xFE => {
                let value = self.read_immediate8(bus, pc);
                self.cp(value);
                self.advance(pc, 2, 2)
            }
            _ => unreachable!("opcode family dispatch should select only ALU opcodes"),
        }
    }

    fn step_stack(&mut self, bus: &mut Bus, pc: u16, opcode: u8) -> u8 {
        match opcode {
            0xC1 => {
                let value = self.pop16(bus);
                self.registers.set_bc(value);
                self.advance(pc, 1, 3)
            }
            0xC5 => {
                self.push16(bus, self.registers.bc());
                self.advance(pc, 1, 4)
            }
            0xD1 => {
                let value = self.pop16(bus);
                self.registers.set_de(value);
                self.advance(pc, 1, 3)
            }
            0xD5 => {
                self.push16(bus, self.registers.de());
                self.advance(pc, 1, 4)
            }
            0xE5 => {
                self.push16(bus, self.registers.hl());
                self.advance(pc, 1, 4)
            }
            0xF1 => {
                let value = self.pop16(bus);
                self.registers.set_af(value);
                self.advance(pc, 1, 3)
            }
            0xF5 => {
                self.push16(bus, self.registers.af());
                self.advance(pc, 1, 4)
            }
            _ => unreachable!("opcode family dispatch should select only stack opcodes"),
        }
    }

    fn step_control(&mut self, bus: &mut Bus, pc: u16, opcode: u8) -> Result<u8, CpuError> {
        let machine_cycles = match opcode {
            0x18 => {
                self.jump_relative(bus, pc);
                3
            }
            0x20 => self.conditional_jr(bus, pc, !self.registers.flag(FLAG_Z)),
            0x28 => self.conditional_jr(bus, pc, self.registers.flag(FLAG_Z)),
            0x30 => self.conditional_jr(bus, pc, !self.registers.flag(FLAG_C)),
            0x38 => self.conditional_jr(bus, pc, self.registers.flag(FLAG_C)),
            0xC0 => self.conditional_ret(bus, pc, !self.registers.flag(FLAG_Z)),
            0xC2 => self.conditional_jp(bus, pc, !self.registers.flag(FLAG_Z)),
            0xC3 => {
                self.registers.pc = self.read_immediate16(bus, pc);
                4
            }
            0xC4 => self.conditional_call(bus, pc, !self.registers.flag(FLAG_Z)),
            0xC7 => self.rst(bus, pc, 0x00),
            0xC8 => self.conditional_ret(bus, pc, self.registers.flag(FLAG_Z)),
            0xC9 => {
                self.registers.pc = self.pop16(bus);
                4
            }
            0xCA => self.conditional_jp(bus, pc, self.registers.flag(FLAG_Z)),
            0xCB => {
                let subopcode = bus.read8(pc.wrapping_add(1));
                return Err(CpuError::UnsupportedCbOpcode {
                    opcode: subopcode,
                    pc,
                });
            }
            0xCC => self.conditional_call(bus, pc, self.registers.flag(FLAG_Z)),
            0xCD => {
                let target = self.read_immediate16(bus, pc);
                self.push16(bus, pc.wrapping_add(3));
                self.registers.pc = target;
                6
            }
            0xCF => self.rst(bus, pc, 0x08),
            0xD0 => self.conditional_ret(bus, pc, !self.registers.flag(FLAG_C)),
            0xD2 => self.conditional_jp(bus, pc, !self.registers.flag(FLAG_C)),
            0xD4 => self.conditional_call(bus, pc, !self.registers.flag(FLAG_C)),
            0xD7 => self.rst(bus, pc, 0x10),
            0xD8 => self.conditional_ret(bus, pc, self.registers.flag(FLAG_C)),
            0xDA => self.conditional_jp(bus, pc, self.registers.flag(FLAG_C)),
            0xDC => self.conditional_call(bus, pc, self.registers.flag(FLAG_C)),
            0xDF => self.rst(bus, pc, 0x18),
            0xE7 => self.rst(bus, pc, 0x20),
            0xE9 => {
                self.registers.pc = self.registers.hl();
                1
            }
            0xEF => self.rst(bus, pc, 0x28),
            0xF7 => self.rst(bus, pc, 0x30),
            0xFF => self.rst(bus, pc, 0x38),
            _ => unreachable!("opcode family dispatch should select only control opcodes"),
        };

        Ok(machine_cycles)
    }

    fn advance(&mut self, pc: u16, bytes: u16, machine_cycles: u8) -> u8 {
        self.registers.pc = pc.wrapping_add(bytes);
        machine_cycles
    }

    fn read_immediate8(&self, bus: &Bus, pc: u16) -> u8 {
        bus.read8(pc.wrapping_add(1))
    }

    fn read_immediate16(&self, bus: &Bus, pc: u16) -> u16 {
        u16::from_le_bytes([bus.read8(pc.wrapping_add(1)), bus.read8(pc.wrapping_add(2))])
    }

    fn load_register(&mut self, bus: &mut Bus, pc: u16, opcode: u8) -> u8 {
        let destination = (opcode >> 3) & 0x07;
        let source = opcode & 0x07;
        let value = self.read_register(bus, source);
        self.write_register(bus, destination, value);

        let machine_cycles = if destination == 6 || source == 6 {
            2
        } else {
            1
        };

        self.advance(pc, 1, machine_cycles)
    }

    fn read_register(&self, bus: &Bus, code: u8) -> u8 {
        match code {
            0 => self.registers.b,
            1 => self.registers.c,
            2 => self.registers.d,
            3 => self.registers.e,
            4 => self.registers.h,
            5 => self.registers.l,
            6 => bus.read8(self.registers.hl()),
            7 => self.registers.a,
            _ => unreachable!("register code is three bits"),
        }
    }

    fn write_register(&mut self, bus: &mut Bus, code: u8, value: u8) {
        match code {
            0 => self.registers.b = value,
            1 => self.registers.c = value,
            2 => self.registers.d = value,
            3 => self.registers.e = value,
            4 => self.registers.h = value,
            5 => self.registers.l = value,
            6 => bus.write8(self.registers.hl(), value),
            7 => self.registers.a = value,
            _ => unreachable!("register code is three bits"),
        }
    }

    fn inc8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        let carry = self.registers.flag(FLAG_C);

        self.registers.set_flag(FLAG_Z, result == 0);
        self.registers.set_flag(FLAG_N, false);
        self.registers
            .set_flag(FLAG_H, (value & 0x0F).wrapping_add(1) > 0x0F);
        self.registers.set_flag(FLAG_C, carry);

        result
    }

    fn dec8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        let carry = self.registers.flag(FLAG_C);

        self.registers.set_flag(FLAG_Z, result == 0);
        self.registers.set_flag(FLAG_N, true);
        self.registers.set_flag(FLAG_H, value & 0x0F == 0);
        self.registers.set_flag(FLAG_C, carry);

        result
    }

    fn add(&mut self, value: u8) {
        let a = self.registers.a;
        let (result, carry) = a.overflowing_add(value);
        self.registers.a = result;

        self.registers.set_f(
            zero_flag(result) | half_flag((a & 0x0F) + (value & 0x0F) > 0x0F) | carry_flag(carry),
        );
    }

    fn sub(&mut self, value: u8) {
        let a = self.registers.a;
        let (result, carry) = a.overflowing_sub(value);
        self.registers.a = result;

        self.registers.set_f(
            zero_flag(result) | FLAG_N | half_flag((a & 0x0F) < (value & 0x0F)) | carry_flag(carry),
        );
    }

    fn and(&mut self, value: u8) {
        self.registers.a &= value;
        self.registers.set_f(zero_flag(self.registers.a) | FLAG_H);
    }

    fn or(&mut self, value: u8) {
        self.registers.a |= value;
        self.registers.set_f(zero_flag(self.registers.a));
    }

    fn xor(&mut self, value: u8) {
        self.registers.a ^= value;
        self.registers.set_f(zero_flag(self.registers.a));
    }

    fn cp(&mut self, value: u8) {
        let a = self.registers.a;
        let (result, carry) = a.overflowing_sub(value);

        self.registers.set_f(
            zero_flag(result) | FLAG_N | half_flag((a & 0x0F) < (value & 0x0F)) | carry_flag(carry),
        );
    }

    fn jump_relative(&mut self, bus: &Bus, pc: u16) {
        let offset = self.read_immediate8(bus, pc) as i8;
        let base = pc.wrapping_add(2);
        self.registers.pc = (i32::from(base) + i32::from(offset)) as u16;
    }

    fn conditional_jr(&mut self, bus: &Bus, pc: u16, condition: bool) -> u8 {
        if condition {
            self.jump_relative(bus, pc);
            3
        } else {
            self.advance(pc, 2, 2)
        }
    }

    fn conditional_jp(&mut self, bus: &Bus, pc: u16, condition: bool) -> u8 {
        let target = self.read_immediate16(bus, pc);
        if condition {
            self.registers.pc = target;
            4
        } else {
            self.advance(pc, 3, 3)
        }
    }

    fn conditional_call(&mut self, bus: &mut Bus, pc: u16, condition: bool) -> u8 {
        let target = self.read_immediate16(bus, pc);
        if condition {
            self.push16(bus, pc.wrapping_add(3));
            self.registers.pc = target;
            6
        } else {
            self.advance(pc, 3, 3)
        }
    }

    fn conditional_ret(&mut self, bus: &mut Bus, pc: u16, condition: bool) -> u8 {
        if condition {
            self.registers.pc = self.pop16(bus);
            5
        } else {
            self.advance(pc, 1, 2)
        }
    }

    fn push16(&mut self, bus: &mut Bus, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        bus.write8(self.registers.sp, high);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        bus.write8(self.registers.sp, low);
    }

    fn pop16(&mut self, bus: &mut Bus) -> u16 {
        let low = bus.read8(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let high = bus.read8(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        u16::from_le_bytes([low, high])
    }

    fn rst(&mut self, bus: &mut Bus, pc: u16, vector: u16) -> u8 {
        self.push16(bus, pc.wrapping_add(1));
        self.registers.pc = vector;
        4
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepResult {
    pub machine_cycles: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpuError {
    UnsupportedOpcode { opcode: u8, pc: u16 },
    UnsupportedCbOpcode { opcode: u8, pc: u16 },
}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedOpcode { opcode, pc } => {
                write!(f, "unsupported opcode 0x{opcode:02X} at PC 0x{pc:04X}")
            }
            Self::UnsupportedCbOpcode { opcode, pc } => {
                write!(f, "unsupported CB opcode 0x{opcode:02X} at PC 0x{pc:04X}")
            }
        }
    }
}

impl std::error::Error for CpuError {}

fn zero_flag(value: u8) -> u8 {
    if value == 0 {
        FLAG_Z
    } else {
        0
    }
}

fn half_flag(enabled: bool) -> u8 {
    if enabled {
        FLAG_H
    } else {
        0
    }
}

fn carry_flag(enabled: bool) -> u8 {
    if enabled {
        FLAG_C
    } else {
        0
    }
}

fn register_operand_cycles(opcode: u8, hl_opcode: u8) -> u8 {
    if opcode == hl_opcode {
        2
    } else {
        1
    }
}

#[cfg(test)]
mod tests;
