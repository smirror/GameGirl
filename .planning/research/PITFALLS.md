# v1.1 Pitfalls Research: Hardware-Accurate Core Architecture

## Pitfall 1: Treating I/O As Plain RAM

Warning sign: `FF00-FF7F` stays as a single byte array and devices later "interpret" it after the fact.

Why it hurts: writes to `FF04`, `FF07`, `FF46`, and `FF50` are actions, not just stored values. Reads from `FF00` depend on selected joypad rows and button state. Serial completion changes `SC` and requests an interrupt.

Prevention: introduce device register handlers before adding more ROM-driven behavior.

## Pitfall 2: Letting Cartridge Type Leak Into CPU/Bus Logic

Warning sign: CPU or Bus branches on `CartridgeType::Mbc1` directly.

Why it hurts: MBC1, MBC3, MBC5, and NoMbc differ in control registers, RAM windows, bank-zero behavior, and optional RTC/rumble behavior. Hard-coded type checks spread controller-specific behavior across the emulator.

Prevention: use a mapper boundary and keep controller quirks inside mapper implementations.

## Pitfall 3: Boot Skip And Boot Execution Sharing Initialization

Warning sign: `Cpu::new_dmg()` post-boot defaults are used even when boot ROM execution is enabled.

Why it hurts: real boot starts at `0x0000` with boot ROM mapped over cartridge ROM, then unmaps through `FF50` before entering cartridge code at `0x0100`.

Prevention: make `BootMode` explicit and test both mapping policy and skip-boot defaults.

## Pitfall 4: Instruction-Level Timing Only

Warning sign: CPU returns cycles, but timers/DMA/PPU are only updated once per instruction with coarse totals.

Why it hurts: timer obscure behavior, DMA restrictions, serial transfer progress, and PPU mode access are M-cycle-sensitive.

Prevention: create a machine-level `tick_mcycle` path now, even if individual devices start with coarse or partial behavior.

## Pitfall 5: Test ROMs Without Capability Gates

Warning sign: CI runs a large ROM suite and treats every failure as equivalent.

Why it hurts: many checked-in ROMs require timer, interrupts, PPU, audio, OAM bug, CGB, or MBC behavior that may be intentionally out of scope.

Prevention: classify ROM fixtures by required capability and run only the subset supported by the current milestone.

## Pitfall 6: File Extension As Hardware Mode

Warning sign: `.gbc` selects CGB behavior and `.gb` selects DMG behavior.

Why it hurts: mode compatibility is encoded in the cartridge header CGB flag, not reliably in host file suffixes.

Prevention: parse `0x0143`, reject CGB-only ROMs while DMG-only, and document compatibility decisions.
