use crate::bus::Bus;

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
    pub f: u8,
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
            sp: 0xFFFE,
            pc: 0x0100,
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
            0x00 => {
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x01 => {
                let value = self.read_immediate16(bus, pc);
                self.registers.set_bc(value);
                self.registers.pc = pc.wrapping_add(3);
                3
            }
            0x02 => {
                bus.write8(self.registers.bc(), self.registers.a);
                self.registers.pc = pc.wrapping_add(1);
                2
            }
            0x04 => {
                self.registers.b = self.inc8(self.registers.b);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x05 => {
                self.registers.b = self.dec8(self.registers.b);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x06 => {
                self.registers.b = self.read_immediate8(bus, pc);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0x0A => {
                self.registers.a = bus.read8(self.registers.bc());
                self.registers.pc = pc.wrapping_add(1);
                2
            }
            0x0C => {
                self.registers.c = self.inc8(self.registers.c);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x0D => {
                self.registers.c = self.dec8(self.registers.c);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x0E => {
                self.registers.c = self.read_immediate8(bus, pc);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0x10 => {
                self.stopped = true;
                self.registers.pc = pc.wrapping_add(2);
                1
            }
            0x11 => {
                let value = self.read_immediate16(bus, pc);
                self.registers.set_de(value);
                self.registers.pc = pc.wrapping_add(3);
                3
            }
            0x12 => {
                bus.write8(self.registers.de(), self.registers.a);
                self.registers.pc = pc.wrapping_add(1);
                2
            }
            0x14 => {
                self.registers.d = self.inc8(self.registers.d);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x15 => {
                self.registers.d = self.dec8(self.registers.d);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x16 => {
                self.registers.d = self.read_immediate8(bus, pc);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0x18 => {
                self.jump_relative(bus, pc);
                3
            }
            0x1A => {
                self.registers.a = bus.read8(self.registers.de());
                self.registers.pc = pc.wrapping_add(1);
                2
            }
            0x1C => {
                self.registers.e = self.inc8(self.registers.e);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x1D => {
                self.registers.e = self.dec8(self.registers.e);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x1E => {
                self.registers.e = self.read_immediate8(bus, pc);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0x20 => self.conditional_jr(bus, pc, !self.registers.flag(FLAG_Z)),
            0x21 => {
                let value = self.read_immediate16(bus, pc);
                self.registers.set_hl(value);
                self.registers.pc = pc.wrapping_add(3);
                3
            }
            0x22 => {
                let hl = self.registers.hl();
                bus.write8(hl, self.registers.a);
                self.registers.set_hl(hl.wrapping_add(1));
                self.registers.pc = pc.wrapping_add(1);
                2
            }
            0x24 => {
                self.registers.h = self.inc8(self.registers.h);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x25 => {
                self.registers.h = self.dec8(self.registers.h);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x26 => {
                self.registers.h = self.read_immediate8(bus, pc);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0x28 => self.conditional_jr(bus, pc, self.registers.flag(FLAG_Z)),
            0x2A => {
                let hl = self.registers.hl();
                self.registers.a = bus.read8(hl);
                self.registers.set_hl(hl.wrapping_add(1));
                self.registers.pc = pc.wrapping_add(1);
                2
            }
            0x2C => {
                self.registers.l = self.inc8(self.registers.l);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x2D => {
                self.registers.l = self.dec8(self.registers.l);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x2E => {
                self.registers.l = self.read_immediate8(bus, pc);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0x30 => self.conditional_jr(bus, pc, !self.registers.flag(FLAG_C)),
            0x31 => {
                self.registers.sp = self.read_immediate16(bus, pc);
                self.registers.pc = pc.wrapping_add(3);
                3
            }
            0x32 => {
                let hl = self.registers.hl();
                bus.write8(hl, self.registers.a);
                self.registers.set_hl(hl.wrapping_sub(1));
                self.registers.pc = pc.wrapping_add(1);
                2
            }
            0x34 => {
                let hl = self.registers.hl();
                let value = self.inc8(bus.read8(hl));
                bus.write8(hl, value);
                self.registers.pc = pc.wrapping_add(1);
                3
            }
            0x35 => {
                let hl = self.registers.hl();
                let value = self.dec8(bus.read8(hl));
                bus.write8(hl, value);
                self.registers.pc = pc.wrapping_add(1);
                3
            }
            0x36 => {
                let value = self.read_immediate8(bus, pc);
                bus.write8(self.registers.hl(), value);
                self.registers.pc = pc.wrapping_add(2);
                3
            }
            0x38 => self.conditional_jr(bus, pc, self.registers.flag(FLAG_C)),
            0x3A => {
                let hl = self.registers.hl();
                self.registers.a = bus.read8(hl);
                self.registers.set_hl(hl.wrapping_sub(1));
                self.registers.pc = pc.wrapping_add(1);
                2
            }
            0x3C => {
                self.registers.a = self.inc8(self.registers.a);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x3D => {
                self.registers.a = self.dec8(self.registers.a);
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x3E => {
                self.registers.a = self.read_immediate8(bus, pc);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0x40..=0x7F if opcode != 0x76 => self.load_register(bus, pc, opcode),
            0x76 => {
                self.halted = true;
                self.stopped = false;
                self.registers.pc = pc.wrapping_add(1);
                1
            }
            0x80..=0x87 => {
                let value = self.read_register(bus, opcode & 0x07);
                self.add(value);
                self.registers.pc = pc.wrapping_add(1);
                if opcode == 0x86 {
                    2
                } else {
                    1
                }
            }
            0x90..=0x97 => {
                let value = self.read_register(bus, opcode & 0x07);
                self.sub(value);
                self.registers.pc = pc.wrapping_add(1);
                if opcode == 0x96 {
                    2
                } else {
                    1
                }
            }
            0xA0..=0xA7 => {
                let value = self.read_register(bus, opcode & 0x07);
                self.and(value);
                self.registers.pc = pc.wrapping_add(1);
                if opcode == 0xA6 {
                    2
                } else {
                    1
                }
            }
            0xA8..=0xAF => {
                let value = self.read_register(bus, opcode & 0x07);
                self.xor(value);
                self.registers.pc = pc.wrapping_add(1);
                if opcode == 0xAE {
                    2
                } else {
                    1
                }
            }
            0xB0..=0xB7 => {
                let value = self.read_register(bus, opcode & 0x07);
                self.or(value);
                self.registers.pc = pc.wrapping_add(1);
                if opcode == 0xB6 {
                    2
                } else {
                    1
                }
            }
            0xB8..=0xBF => {
                let value = self.read_register(bus, opcode & 0x07);
                self.cp(value);
                self.registers.pc = pc.wrapping_add(1);
                if opcode == 0xBE {
                    2
                } else {
                    1
                }
            }
            0xC0 => self.conditional_ret(bus, pc, !self.registers.flag(FLAG_Z)),
            0xC1 => {
                let value = self.pop16(bus);
                self.registers.set_bc(value);
                self.registers.pc = pc.wrapping_add(1);
                3
            }
            0xC2 => self.conditional_jp(bus, pc, !self.registers.flag(FLAG_Z)),
            0xC3 => {
                self.registers.pc = self.read_immediate16(bus, pc);
                4
            }
            0xC4 => self.conditional_call(bus, pc, !self.registers.flag(FLAG_Z)),
            0xC5 => {
                self.push16(bus, self.registers.bc());
                self.registers.pc = pc.wrapping_add(1);
                4
            }
            0xC6 => {
                let value = self.read_immediate8(bus, pc);
                self.add(value);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
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
            0xD1 => {
                let value = self.pop16(bus);
                self.registers.set_de(value);
                self.registers.pc = pc.wrapping_add(1);
                3
            }
            0xD2 => self.conditional_jp(bus, pc, !self.registers.flag(FLAG_C)),
            0xD4 => self.conditional_call(bus, pc, !self.registers.flag(FLAG_C)),
            0xD5 => {
                self.push16(bus, self.registers.de());
                self.registers.pc = pc.wrapping_add(1);
                4
            }
            0xD6 => {
                let value = self.read_immediate8(bus, pc);
                self.sub(value);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0xD7 => self.rst(bus, pc, 0x10),
            0xD8 => self.conditional_ret(bus, pc, self.registers.flag(FLAG_C)),
            0xDA => self.conditional_jp(bus, pc, self.registers.flag(FLAG_C)),
            0xDC => self.conditional_call(bus, pc, self.registers.flag(FLAG_C)),
            0xDF => self.rst(bus, pc, 0x18),
            0xE0 => {
                let offset = u16::from(self.read_immediate8(bus, pc));
                bus.write8(0xFF00 + offset, self.registers.a);
                self.registers.pc = pc.wrapping_add(2);
                3
            }
            0xE5 => {
                self.push16(bus, self.registers.hl());
                self.registers.pc = pc.wrapping_add(1);
                4
            }
            0xE6 => {
                let value = self.read_immediate8(bus, pc);
                self.and(value);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0xE7 => self.rst(bus, pc, 0x20),
            0xE9 => {
                self.registers.pc = self.registers.hl();
                1
            }
            0xEA => {
                let address = self.read_immediate16(bus, pc);
                bus.write8(address, self.registers.a);
                self.registers.pc = pc.wrapping_add(3);
                4
            }
            0xEE => {
                let value = self.read_immediate8(bus, pc);
                self.xor(value);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0xEF => self.rst(bus, pc, 0x28),
            0xF0 => {
                let offset = u16::from(self.read_immediate8(bus, pc));
                self.registers.a = bus.read8(0xFF00 + offset);
                self.registers.pc = pc.wrapping_add(2);
                3
            }
            0xF1 => {
                let value = self.pop16(bus);
                self.registers.set_af(value);
                self.registers.pc = pc.wrapping_add(1);
                3
            }
            0xF5 => {
                self.push16(bus, self.registers.af());
                self.registers.pc = pc.wrapping_add(1);
                4
            }
            0xF6 => {
                let value = self.read_immediate8(bus, pc);
                self.or(value);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0xF7 => self.rst(bus, pc, 0x30),
            0xFA => {
                let address = self.read_immediate16(bus, pc);
                self.registers.a = bus.read8(address);
                self.registers.pc = pc.wrapping_add(3);
                4
            }
            0xFE => {
                let value = self.read_immediate8(bus, pc);
                self.cp(value);
                self.registers.pc = pc.wrapping_add(2);
                2
            }
            0xFF => self.rst(bus, pc, 0x38),
            _ => return Err(CpuError::UnsupportedOpcode { opcode, pc }),
        };

        Ok(StepResult { machine_cycles })
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
        self.registers.pc = pc.wrapping_add(1);

        if destination == 6 || source == 6 {
            2
        } else {
            1
        }
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
            self.registers.pc = pc.wrapping_add(2);
            2
        }
    }

    fn conditional_jp(&mut self, bus: &Bus, pc: u16, condition: bool) -> u8 {
        let target = self.read_immediate16(bus, pc);
        if condition {
            self.registers.pc = target;
            4
        } else {
            self.registers.pc = pc.wrapping_add(3);
            3
        }
    }

    fn conditional_call(&mut self, bus: &mut Bus, pc: u16, condition: bool) -> u8 {
        let target = self.read_immediate16(bus, pc);
        if condition {
            self.push16(bus, pc.wrapping_add(3));
            self.registers.pc = target;
            6
        } else {
            self.registers.pc = pc.wrapping_add(3);
            3
        }
    }

    fn conditional_ret(&mut self, bus: &Bus, pc: u16, condition: bool) -> u8 {
        if condition {
            self.registers.pc = self.pop16(bus);
            5
        } else {
            self.registers.pc = pc.wrapping_add(1);
            2
        }
    }

    fn push16(&mut self, bus: &mut Bus, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        bus.write8(self.registers.sp, high);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        bus.write8(self.registers.sp, low);
    }

    fn pop16(&mut self, bus: &Bus) -> u16 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::Cartridge;

    fn bus_with_program(program: &[u8]) -> Bus {
        let mut rom = vec![0; 0x8000];
        let start = 0x0100;
        rom[start..start + program.len()].copy_from_slice(program);
        Bus::new(Cartridge::from_bytes(rom).expect("valid ROM-only cartridge"))
    }

    fn step(cpu: &mut Cpu, bus: &mut Bus) -> StepResult {
        cpu.step(bus).expect("implemented opcode should step")
    }

    #[test]
    fn dmg_post_boot_register_defaults() {
        let cpu = Cpu::new_dmg();

        assert_eq!(cpu.registers.pc, 0x0100);
        assert_eq!(cpu.registers.sp, 0xFFFE);
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.registers.f, 0xB0);
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.registers.c, 0x13);
        assert_eq!(cpu.registers.d, 0x00);
        assert_eq!(cpu.registers.e, 0xD8);
        assert_eq!(cpu.registers.h, 0x01);
        assert_eq!(cpu.registers.l, 0x4D);
        assert!(!cpu.halted);
        assert!(!cpu.stopped);
    }

    #[test]
    fn flag_register_masks_lower_nibble() {
        let mut registers = Registers::new_dmg();

        registers.set_f(0xFF);
        assert_eq!(registers.f, 0xF0);

        registers.set_af(0x123F);
        assert_eq!(registers.a, 0x12);
        assert_eq!(registers.f, 0x30);

        registers.set_flag(FLAG_Z, true);
        registers.set_flag(FLAG_C, true);
        assert_eq!(registers.f & 0x0F, 0);
    }

    #[test]
    fn unsupported_opcode_reports_opcode_and_pc_without_advancing() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0xD3]);

        let error = cpu.step(&mut bus).expect_err("0xD3 should be unsupported");

        assert_eq!(
            error,
            CpuError::UnsupportedOpcode {
                opcode: 0xD3,
                pc: 0x0100
            }
        );
        assert_eq!(cpu.registers.pc, 0x0100);
    }

    #[test]
    fn nop_advances_pc_and_reports_cycles() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0x00]);

        let result = step(&mut cpu, &mut bus);

        assert_eq!(result.machine_cycles, 1);
        assert_eq!(cpu.registers.pc, 0x0101);
    }

    #[test]
    fn halt_sets_placeholder_state() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0x76]);

        let result = step(&mut cpu, &mut bus);

        assert_eq!(result.machine_cycles, 1);
        assert!(cpu.halted);
        assert!(!cpu.stopped);
        assert_eq!(cpu.registers.pc, 0x0101);
    }

    #[test]
    fn stop_consumes_padding_byte_and_sets_placeholder_state() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0x10, 0x00]);

        let result = step(&mut cpu, &mut bus);

        assert_eq!(result.machine_cycles, 1);
        assert!(cpu.stopped);
        assert_eq!(cpu.registers.pc, 0x0102);
    }

    #[test]
    fn loads_immediate_registers() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[
            0x01, 0x34, 0x12, 0x11, 0x78, 0x56, 0x21, 0xBC, 0x9A, 0x31, 0xFC, 0xFF, 0x06, 0xAB,
            0x0E, 0xCD, 0x16, 0xEF, 0x1E, 0x01, 0x26, 0x02, 0x2E, 0x03, 0x3E, 0x99,
        ]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(cpu.registers.bc(), 0x1234);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(cpu.registers.de(), 0x5678);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(cpu.registers.hl(), 0x9ABC);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(cpu.registers.sp, 0xFFFC);

        for _ in 0..7 {
            assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        }

        assert_eq!(cpu.registers.b, 0xAB);
        assert_eq!(cpu.registers.c, 0xCD);
        assert_eq!(cpu.registers.d, 0xEF);
        assert_eq!(cpu.registers.e, 0x01);
        assert_eq!(cpu.registers.h, 0x02);
        assert_eq!(cpu.registers.l, 0x03);
        assert_eq!(cpu.registers.a, 0x99);
    }

    #[test]
    fn loads_between_registers_and_hl_memory() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.b = 0x42;
        let mut bus = bus_with_program(&[0x78]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 1);
        assert_eq!(cpu.registers.a, 0x42);

        let mut cpu = Cpu::new_dmg();
        cpu.registers.b = 0x77;
        cpu.registers.set_hl(0xC000);
        let mut bus = bus_with_program(&[0x70, 0x7E]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(bus.read8(0xC000), 0x77);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(cpu.registers.a, 0x77);
    }

    #[test]
    fn loads_accumulator_through_bus_addresses() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0x44;
        cpu.registers.set_bc(0xC000);
        cpu.registers.set_de(0xC001);
        cpu.registers.set_hl(0xC002);
        let mut bus = bus_with_program(&[0x02, 0x12, 0x22, 0x32, 0xE0, 0x10, 0xEA, 0x04, 0xC0]);

        step(&mut cpu, &mut bus);
        step(&mut cpu, &mut bus);
        step(&mut cpu, &mut bus);
        step(&mut cpu, &mut bus);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 4);

        assert_eq!(bus.read8(0xC000), 0x44);
        assert_eq!(bus.read8(0xC001), 0x44);
        assert_eq!(bus.read8(0xC002), 0x44);
        assert_eq!(bus.read8(0xC003), 0x44);
        assert_eq!(bus.read8(0xFF10), 0x44);
        assert_eq!(bus.read8(0xC004), 0x44);

        let mut cpu = Cpu::new_dmg();
        cpu.registers.set_bc(0xC010);
        cpu.registers.set_de(0xC011);
        cpu.registers.set_hl(0xC012);
        let mut bus = bus_with_program(&[0x0A, 0x1A, 0x2A, 0x3A, 0xF0, 0x10, 0xFA, 0x14, 0xC0]);
        bus.write8(0xC010, 0x10);
        bus.write8(0xC011, 0x11);
        bus.write8(0xC012, 0x12);
        bus.write8(0xC013, 0x13);
        bus.write8(0xFF10, 0xF0);
        bus.write8(0xC014, 0xFA);

        step(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0x10);
        step(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0x11);
        step(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0x12);
        assert_eq!(cpu.registers.hl(), 0xC013);
        step(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0x13);
        assert_eq!(cpu.registers.hl(), 0xC012);
        step(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0xF0);
        step(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0xFA);
    }

    #[test]
    fn unsupported_opcode_still_preserves_pc_after_loads_exist() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0xD3]);

        let error = cpu.step(&mut bus).expect_err("0xD3 should be unsupported");

        assert_eq!(
            error,
            CpuError::UnsupportedOpcode {
                opcode: 0xD3,
                pc: 0x0100
            }
        );
        assert_eq!(cpu.registers.pc, 0x0100);
    }

    #[test]
    fn inc_updates_zero_and_half_carry_flags() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.b = 0x0F;
        cpu.registers.set_flag(FLAG_C, true);
        let mut bus = bus_with_program(&[0x04]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.b, 0x10);
        assert!(!cpu.registers.flag(FLAG_Z));
        assert!(!cpu.registers.flag(FLAG_N));
        assert!(cpu.registers.flag(FLAG_H));
        assert!(cpu.registers.flag(FLAG_C));

        let mut cpu = Cpu::new_dmg();
        cpu.registers.b = 0xFF;
        let mut bus = bus_with_program(&[0x04]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.b, 0);
        assert!(cpu.registers.flag(FLAG_Z));
        assert!(cpu.registers.flag(FLAG_H));
    }

    #[test]
    fn dec_updates_subtract_and_half_borrow_flags() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.b = 0x10;
        cpu.registers.set_flag(FLAG_C, true);
        let mut bus = bus_with_program(&[0x05]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.b, 0x0F);
        assert!(!cpu.registers.flag(FLAG_Z));
        assert!(cpu.registers.flag(FLAG_N));
        assert!(cpu.registers.flag(FLAG_H));
        assert!(cpu.registers.flag(FLAG_C));

        let mut cpu = Cpu::new_dmg();
        cpu.registers.b = 0x01;
        let mut bus = bus_with_program(&[0x05]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.b, 0);
        assert!(cpu.registers.flag(FLAG_Z));
    }

    #[test]
    fn inc_dec_hl_use_bus_and_report_cycles() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.set_hl(0xC000);
        let mut bus = bus_with_program(&[0x34, 0x35]);
        bus.write8(0xC000, 0x0F);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(bus.read8(0xC000), 0x10);
        assert!(cpu.registers.flag(FLAG_H));

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(bus.read8(0xC000), 0x0F);
        assert!(cpu.registers.flag(FLAG_N));
        assert!(cpu.registers.flag(FLAG_H));
    }

    #[test]
    fn add_updates_carry_and_half_carry_flags() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0x8F;
        cpu.registers.b = 0x81;
        let mut bus = bus_with_program(&[0x80]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.a, 0x10);
        assert!(!cpu.registers.flag(FLAG_Z));
        assert!(!cpu.registers.flag(FLAG_N));
        assert!(cpu.registers.flag(FLAG_H));
        assert!(cpu.registers.flag(FLAG_C));
    }

    #[test]
    fn sub_updates_borrow_and_half_borrow_flags() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0x10;
        cpu.registers.b = 0x01;
        let mut bus = bus_with_program(&[0x90]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.a, 0x0F);
        assert!(!cpu.registers.flag(FLAG_Z));
        assert!(cpu.registers.flag(FLAG_N));
        assert!(cpu.registers.flag(FLAG_H));
        assert!(!cpu.registers.flag(FLAG_C));

        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0x00;
        cpu.registers.b = 0x01;
        let mut bus = bus_with_program(&[0x90]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.a, 0xFF);
        assert!(cpu.registers.flag(FLAG_C));
    }

    #[test]
    fn add_sub_memory_and_immediate_report_cycles() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0x10;
        cpu.registers.set_hl(0xC000);
        let mut bus = bus_with_program(&[0x86, 0xC6, 0x02, 0x96, 0xD6, 0x01]);
        bus.write8(0xC000, 0x01);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(cpu.registers.a, 0x13);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(cpu.registers.a, 0x12);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(cpu.registers.a, 0x11);
    }

    #[test]
    fn logical_ops_update_flags() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0b1010;
        cpu.registers.b = 0b1100;
        let mut bus = bus_with_program(&[0xA0]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.a, 0b1000);
        assert!(cpu.registers.flag(FLAG_H));
        assert!(!cpu.registers.flag(FLAG_N));
        assert!(!cpu.registers.flag(FLAG_C));

        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0;
        let mut bus = bus_with_program(&[0xB0, 0xAF]);

        step(&mut cpu, &mut bus);
        assert!(cpu.registers.flag(FLAG_Z));
        step(&mut cpu, &mut bus);
        assert!(cpu.registers.flag(FLAG_Z));
        assert_eq!(cpu.registers.a, 0);
    }

    #[test]
    fn cp_updates_flags_without_changing_a() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0x10;
        cpu.registers.b = 0x10;
        let mut bus = bus_with_program(&[0xB8]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.a, 0x10);
        assert!(cpu.registers.flag(FLAG_Z));
        assert!(cpu.registers.flag(FLAG_N));
        assert!(!cpu.registers.flag(FLAG_C));

        let mut cpu = Cpu::new_dmg();
        cpu.registers.a = 0x10;
        cpu.registers.b = 0x11;
        let mut bus = bus_with_program(&[0xB8]);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.a, 0x10);
        assert!(cpu.registers.flag(FLAG_C));
        assert!(cpu.registers.flag(FLAG_H));
    }

    #[test]
    fn arithmetic_opcode_families_report_cycles() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.set_hl(0xC000);
        let mut bus = bus_with_program(&[0x80, 0x86, 0xC6, 0x01, 0xA0, 0xA6, 0xE6, 0x01]);
        bus.write8(0xC000, 0x01);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 1);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 1);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 2);
    }

    #[test]
    fn push_writes_stack_little_endian_through_bus() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.set_bc(0x1234);
        let mut bus = bus_with_program(&[0xC5]);

        let result = step(&mut cpu, &mut bus);

        assert_eq!(result.machine_cycles, 4);
        assert_eq!(cpu.registers.sp, 0xFFFC);
        assert_eq!(bus.read8(0xFFFC), 0x34);
        assert_eq!(bus.read8(0xFFFD), 0x12);
    }

    #[test]
    fn pop_reads_stack_little_endian_through_bus() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.sp = 0xFFFC;
        let mut bus = bus_with_program(&[0xC1]);
        bus.write8(0xFFFC, 0x34);
        bus.write8(0xFFFD, 0x12);

        let result = step(&mut cpu, &mut bus);

        assert_eq!(result.machine_cycles, 3);
        assert_eq!(cpu.registers.bc(), 0x1234);
        assert_eq!(cpu.registers.sp, 0xFFFE);
    }

    #[test]
    fn pop_af_masks_flag_lower_nibble() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.sp = 0xFFFC;
        let mut bus = bus_with_program(&[0xF1]);
        bus.write8(0xFFFC, 0x3F);
        bus.write8(0xFFFD, 0x12);

        step(&mut cpu, &mut bus);

        assert_eq!(cpu.registers.a, 0x12);
        assert_eq!(cpu.registers.f, 0x30);
    }

    #[test]
    fn jp_sets_absolute_pc() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0xC3, 0x34, 0x12]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 4);
        assert_eq!(cpu.registers.pc, 0x1234);

        let mut cpu = Cpu::new_dmg();
        cpu.registers.set_hl(0xC000);
        let mut bus = bus_with_program(&[0xE9]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 1);
        assert_eq!(cpu.registers.pc, 0xC000);
    }

    #[test]
    fn conditional_jp_reports_taken_and_not_taken_cycles() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.set_flag(FLAG_Z, false);
        let mut bus = bus_with_program(&[0xC2, 0x34, 0x12]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 4);
        assert_eq!(cpu.registers.pc, 0x1234);

        let mut cpu = Cpu::new_dmg();
        cpu.registers.set_flag(FLAG_Z, false);
        let mut bus = bus_with_program(&[0xCA, 0x34, 0x12]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(cpu.registers.pc, 0x0103);
    }

    #[test]
    fn jr_applies_signed_relative_offsets() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0x18, 0x02]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(cpu.registers.pc, 0x0104);

        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0x18, 0xFE]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 3);
        assert_eq!(cpu.registers.pc, 0x0100);
    }

    #[test]
    fn call_pushes_return_address_and_jumps() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0xCD, 0x34, 0x12]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 6);
        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(cpu.registers.sp, 0xFFFC);
        assert_eq!(bus.read8(0xFFFC), 0x03);
        assert_eq!(bus.read8(0xFFFD), 0x01);
    }

    #[test]
    fn ret_pops_pc_from_stack() {
        let mut cpu = Cpu::new_dmg();
        cpu.registers.sp = 0xFFFC;
        let mut bus = bus_with_program(&[0xC9]);
        bus.write8(0xFFFC, 0x34);
        bus.write8(0xFFFD, 0x12);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 4);
        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(cpu.registers.sp, 0xFFFE);
    }

    #[test]
    fn rst_pushes_return_and_jumps_to_vector() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0xC7]);

        assert_eq!(step(&mut cpu, &mut bus).machine_cycles, 4);
        assert_eq!(cpu.registers.pc, 0x0000);
        assert_eq!(bus.read8(0xFFFC), 0x01);
        assert_eq!(bus.read8(0xFFFD), 0x01);
    }

    #[test]
    fn cb_prefix_reports_subopcode_and_original_pc_without_advancing() {
        let mut cpu = Cpu::new_dmg();
        let mut bus = bus_with_program(&[0xCB, 0x11]);

        let error = cpu
            .step(&mut bus)
            .expect_err("CB opcode should be deferred");

        assert_eq!(
            error,
            CpuError::UnsupportedCbOpcode {
                opcode: 0x11,
                pc: 0x0100
            }
        );
        assert_eq!(cpu.registers.pc, 0x0100);
    }

    #[test]
    fn implemented_opcode_families_report_machine_cycles() {
        fn cycle(program: &[u8], configure: impl FnOnce(&mut Cpu, &mut Bus)) -> u8 {
            let mut cpu = Cpu::new_dmg();
            let mut bus = bus_with_program(program);
            configure(&mut cpu, &mut bus);
            step(&mut cpu, &mut bus).machine_cycles
        }

        assert_eq!(cycle(&[0x00], |_, _| {}), 1);
        assert_eq!(cycle(&[0x06, 0x12], |_, _| {}), 2);
        assert_eq!(
            cycle(&[0x36, 0x12], |cpu, _| cpu.registers.set_hl(0xC000)),
            3
        );
        assert_eq!(cycle(&[0xEA, 0x00, 0xC0], |_, _| {}), 4);
        assert_eq!(cycle(&[0x80], |_, _| {}), 1);
        assert_eq!(
            cycle(&[0x86], |cpu, bus| {
                cpu.registers.set_hl(0xC000);
                bus.write8(0xC000, 1);
            }),
            2
        );
        assert_eq!(
            cycle(&[0x34], |cpu, bus| {
                cpu.registers.set_hl(0xC000);
                bus.write8(0xC000, 1);
            }),
            3
        );
        assert_eq!(cycle(&[0xC3, 0x00, 0xC0], |_, _| {}), 4);
        assert_eq!(cycle(&[0x18, 0x00], |_, _| {}), 3);
        assert_eq!(cycle(&[0xCD, 0x00, 0xC0], |_, _| {}), 6);
        assert_eq!(
            cycle(&[0xC9], |cpu, bus| {
                cpu.registers.sp = 0xFFFC;
                bus.write8(0xFFFC, 0x00);
                bus.write8(0xFFFD, 0xC0);
            }),
            4
        );
        assert_eq!(cycle(&[0xC5], |_, _| {}), 4);
        assert_eq!(
            cycle(&[0xC1], |cpu, bus| {
                cpu.registers.sp = 0xFFFC;
                bus.write8(0xFFFC, 0x34);
                bus.write8(0xFFFD, 0x12);
            }),
            3
        );
        assert_eq!(cycle(&[0xC7], |_, _| {}), 4);
    }
}
