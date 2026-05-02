# v1.1 Research Summary: Hardware-Accurate Core Architecture

## Core Finding

GameGirl's next milestone should harden cartridge, mapper, bus/MMIO, boot, and M-cycle boundaries before adding broader CPU/PPU behavior. The existing CPU foundation is useful, but many future "CPU bugs" will actually originate from address-space and device side-effect modeling if the Bus remains a plain memory container.

## Stack Additions

None. Rust 2021, Cargo tests, fixed-size arrays, enums/traits, and checked-in ROM fixtures are enough for v1.1.

## Table Stakes

- One cartridge loading path that validates and constructs cartridge state.
- Header policy for CGB flag, header checksum, global checksum metadata, and CGB-only rejection.
- Mapper abstraction with NoMbc first, then MBC1/MBC3/MBC5.
- Full DMG address-space routing, including VRAM and external cartridge RAM.
- Device-backed MMIO for joypad, serial, timer, interrupts, DMA, and boot ROM lock.
- Execution reads/writes separated from side-effect-free peeks.
- Explicit `BootMode`.
- Machine-level M-cycle advancement.
- Capability-gated ROM harness.

## Recommended Requirements Categories

- Cartridge Header Policy
- Mapper Layer
- Bus And Address Space
- MMIO Devices
- Boot And Machine Timing
- Validation Harness

## Recommended Phase Order

1. Cartridge header/mode policy and mapper trait.
2. Full address-space routing and peek/read/write split.
3. Side-effect-aware MMIO device skeletons.
4. Boot mode plus M-cycle machine stepping.
5. MBC controller foundations and capability-gated ROM harness.

## Watch Out For

- Do not use file extension as hardware mode.
- Do not let MBC-specific details leak into CPU instruction code.
- Do not run broad ROM suites without capability classification.
- Do not mix post-boot skip defaults with boot ROM execution.
- Do not postpone all timing until after PPU/timer work; create the M-cycle surface now.

## Sources

- Pan Docs Memory Map: https://gbdev.io/pandocs/Memory_Map.html
- Pan Docs Cartridge Header: https://gbdev.io/pandocs/The_Cartridge_Header.html
- Pan Docs MBCs: https://gbdev.io/pandocs/MBCs.html
- Pan Docs Timer Registers and Obscure Behaviour: https://gbdev.io/pandocs/Timer_and_Divider_Registers.html, https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html
- Pan Docs Joypad, Serial, OAM DMA, VRAM/OAM Access: https://gbdev.io/pandocs/Joypad_Input.html, https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html, https://gbdev.io/pandocs/OAM_DMA_Transfer.html, https://gbdev.io/pandocs/Accessing_VRAM_and_OAM.html
- Pan Docs Power-Up Sequence: https://gbdev.io/pandocs/Power_Up_Sequence.html
- Game Boy: Complete Technical Reference: https://gekkio.fi/files/gb-docs/gbctr.pdf
- mooneye-test-suite: https://github.com/Gekkio/mooneye-test-suite
- blargg test ROM collection mirror: https://github.com/retrio/gb-test-roms
