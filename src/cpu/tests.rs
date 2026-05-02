use super::*;
use crate::cartridge::Cartridge;
use crate::memory_map::{CARTRIDGE_ROM_SIZE, DMG_ENTRY_POINT, DMG_STACK_POINTER};

fn bus_with_program(program: &[u8]) -> Bus {
    let mut rom = vec![0; CARTRIDGE_ROM_SIZE];
    let start = usize::from(DMG_ENTRY_POINT);
    rom[start..start + program.len()].copy_from_slice(program);
    Bus::new(Cartridge::from_bytes(rom).expect("valid ROM-only cartridge"))
}

fn step(cpu: &mut Cpu, bus: &mut Bus) -> StepResult {
    cpu.step(bus).expect("implemented opcode should step")
}

#[test]
fn dmg_post_boot_register_defaults() {
    let cpu = Cpu::new_dmg();

    assert_eq!(cpu.registers.pc, DMG_ENTRY_POINT);
    assert_eq!(cpu.registers.sp, DMG_STACK_POINTER);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.f(), 0xB0);
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
    assert_eq!(registers.f(), 0xF0);

    registers.set_af(0x123F);
    assert_eq!(registers.a, 0x12);
    assert_eq!(registers.f(), 0x30);

    registers.set_flag(FLAG_Z, true);
    registers.set_flag(FLAG_C, true);
    assert_eq!(registers.f() & 0x0F, 0);
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
        0x01, 0x34, 0x12, 0x11, 0x78, 0x56, 0x21, 0xBC, 0x9A, 0x31, 0xFC, 0xFF, 0x06, 0xAB, 0x0E,
        0xCD, 0x16, 0xEF, 0x1E, 0x01, 0x26, 0x02, 0x2E, 0x03, 0x3E, 0x99,
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
    assert_eq!(cpu.registers.f(), 0x30);
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
