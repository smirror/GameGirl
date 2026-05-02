# v1.1 Stack Research: Hardware-Accurate Core Architecture

## Context

GameGirl remains a Rust 2021 Cargo crate with no runtime dependencies. v1.1 should harden core architecture rather than add UI, audio output, or a large external framework.

## Stack Additions

No new Rust dependencies are recommended for this milestone.

Use existing project tools:

- Cargo unit and integration tests.
- Existing checked-in ROM corpus under `roms/`.
- Shell verification script pattern from `scripts/verify_rom_loading.sh`.
- Internal Rust modules for hardware devices.

## New Internal Modules

Recommended module split:

- `src/machine.rs` — top-level machine/device owner and step orchestration.
- `src/interconnect.rs` or evolved `src/bus.rs` — address router that exposes execution reads/writes and side-effect-free peeks.
- `src/mapper.rs` or `src/cartridge/mapper.rs` — mapper trait plus NoMbc/MBC implementations.
- `src/timer.rs` — DIV/TIMA/TMA/TAC state and write side effects.
- `src/serial.rs` — SB/SC register behavior and test-output capture hook.
- `src/joypad.rs` — active-low matrix register behavior.
- `src/dma.rs` — OAM DMA request/progress state.
- `src/ppu.rs` — minimal mode/access-state surface for VRAM/OAM restrictions.
- `src/boot.rs` — boot mode and boot ROM mapping policy.

## Why No Dependencies

The highest-risk work is hardware semantics, not parsing, UI, async, or data modeling. Standard library types are enough for:

- `Box<dyn Mapper>` or enum-backed mappers.
- Fixed-size arrays for WRAM, VRAM, OAM, HRAM.
- Explicit `Result`-based load errors.
- Deterministic test harness loops.

## Primary References Checked

- Pan Docs memory map and I/O ranges: https://gbdev.io/pandocs/Memory_Map.html
- Pan Docs cartridge header: https://gbdev.io/pandocs/The_Cartridge_Header.html
- Pan Docs MBC overview: https://gbdev.io/pandocs/MBCs.html
- Pan Docs timer behavior: https://gbdev.io/pandocs/Timer_and_Divider_Registers.html and https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html
- Pan Docs OAM DMA and VRAM/OAM access: https://gbdev.io/pandocs/OAM_DMA_Transfer.html and https://gbdev.io/pandocs/Accessing_VRAM_and_OAM.html
- Gekkio GB Complete Technical Reference: https://gekkio.fi/files/gb-docs/gbctr.pdf
- mooneye-test-suite: https://github.com/Gekkio/mooneye-test-suite
- blargg test ROM collection mirror: https://github.com/retrio/gb-test-roms

## What Not To Add

- No GUI crate.
- No audio backend.
- No dynamic plugin system.
- No generated opcode table dependency.
- No broad test runner framework unless the local harness becomes unwieldy.
