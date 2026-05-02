# Phase 1 Research: Cartridge and Bus Foundation

**Phase:** 01-cartridge-and-bus-foundation
**Researched:** 2026-05-02
**Status:** RESEARCH COMPLETE

## Scope

Phase 1 is a foundation slice. It should replace text-based ROM loading with binary cartridge loading, parse the core Game Boy cartridge header fields, support ROM-only cartridge reads, and introduce a Bus API for CPU-visible 8-bit reads and writes.

This phase should not execute CPU instructions, run blargg/mooneye ROMs, implement timer or interrupt side effects, render pixels, support broad MBC behavior, or add a UI/audio host.

## Current Code Facts

- `src/main.rs` is the only Rust source file.
- `src/main.rs` currently uses `std::fs::read_to_string`, which is wrong for arbitrary `.gb` or `.gbc` ROM bytes.
- The crate has no runtime dependencies beyond Rust standard library usage.
- There are no source-level tests yet.
- ROM fixtures exist under `roms/`, but Phase 1 should mostly use in-memory byte fixtures so tests stay fast and focused.

## Implementation Shape

Recommended files:

- `src/lib.rs`: expose reusable core modules.
- `src/cartridge.rs`: own ROM byte loading, cartridge metadata, validation, and ROM-only reads.
- `src/bus.rs`: own DMG address routing and simple backing storage for Phase 1 memory ranges.
- `src/main.rs`: remain a thin CLI boundary that validates path shape, calls cartridge APIs, and prints clear errors.

Recommended module exports:

- `pub mod cartridge;`
- `pub mod bus;`

The CLI should import from the library crate rather than embedding cartridge or Bus logic in `main`.

## Cartridge Header Notes

Minimum useful ROM length for Phase 1 is `0x150` bytes, because the cartridge header ends at `0x014F`.

Header fields to parse:

- Entry point bytes: `0x0100..=0x0103`
- Nintendo logo bytes, optional metadata only: `0x0104..=0x0133`
- Title bytes: `0x0134..=0x0143`
- CGB flag metadata: `0x0143`
- Cartridge type: `0x0147`
- ROM size code: `0x0148`
- RAM size code: `0x0149`
- Header checksum metadata, optional: `0x014D`
- Global checksum metadata, optional: `0x014E..=0x014F`

Phase 1 should accept cartridge type `0x00` as ROM-only and return a clear unsupported-cartridge error for other types. Later MBC support belongs outside this phase.

Useful ROM size code mapping for Phase 1:

- `0x00`: 32 KiB
- `0x01`: 64 KiB
- `0x02`: 128 KiB
- `0x03`: 256 KiB
- `0x04`: 512 KiB
- `0x05`: 1 MiB
- `0x06`: 2 MiB
- `0x07`: 4 MiB
- `0x08`: 8 MiB

Useful RAM size code mapping for Phase 1:

- `0x00`: no RAM
- `0x01`: 2 KiB
- `0x02`: 8 KiB
- `0x03`: 32 KiB
- `0x04`: 128 KiB
- `0x05`: 64 KiB

Unknown or unsupported size codes should return clear errors instead of silently guessing.

## Cartridge API Notes

Recommended types:

- `Cartridge`
- `CartridgeHeader`
- `CartridgeType`
- `CartridgeError`

Recommended constructors and helpers:

- `Cartridge::from_bytes(bytes: Vec<u8>) -> Result<Self, CartridgeError>`
- `Cartridge::read_rom(&self, address: u16) -> u8`
- `Cartridge::write_rom(&mut self, address: u16, value: u8)` as a ROM-only no-op or explicit policy hook
- `load_cartridge_file(path: impl AsRef<Path>) -> Result<Cartridge, CartridgeError>`

`CartridgeError` should implement `std::fmt::Display` and should be useful to CLI users. It can wrap `std::io::Error` for filesystem failures.

## Bus Memory Map Notes

Phase 1 Bus should expose:

- `Bus::new(cartridge: Cartridge) -> Self`
- `read8(&self, addr: u16) -> u8`
- `write8(&mut self, addr: u16, value: u8)`

Recommended backing storage:

- WRAM: `[u8; 0x2000]` for `0xC000..=0xDFFF`
- OAM placeholder storage: `[u8; 0xA0]` for `0xFE00..=0xFE9F`
- I/O storage or stubs: `[u8; 0x80]` for `0xFF00..=0xFF7F`
- HRAM: `[u8; 0x7F]` for `0xFF80..=0xFFFE`
- IE: `u8` for `0xFFFF`

Address routing:

- `0x0000..=0x7FFF`: cartridge ROM reads; writes route to cartridge policy.
- `0xC000..=0xDFFF`: WRAM.
- `0xE000..=0xFDFF`: Echo RAM mirror of WRAM.
- `0xFE00..=0xFE9F`: OAM placeholder storage.
- `0xFEA0..=0xFEFF`: unusable; reads return `0xFF`, writes ignored.
- `0xFF00..=0xFF7F`: basic I/O storage or explicit stubs.
- `0xFF80..=0xFFFE`: HRAM.
- `0xFFFF`: interrupt enable register.

VRAM and external RAM can be left as prepared storage or default behavior only if needed, but the required success criteria focus on cartridge ROM, WRAM, HRAM, IE, I/O, and representative unusable ranges.

## Test Strategy

Use focused unit tests inside `src/cartridge.rs` and `src/bus.rs`.

Minimum tests:

- Loading/parsing rejects ROMs shorter than `0x150`.
- Header parsing exposes title, type, ROM size, RAM size, and entry bytes.
- Unsupported cartridge type returns a clear error.
- Unsupported ROM/RAM size code returns a clear error.
- ROM-only cartridge reads return bytes from `0x0000..=0x7FFF`.
- Bus reads cartridge ROM through the cartridge layer.
- WRAM writes can be read back.
- Echo RAM mirrors WRAM.
- OAM placeholder range stores values or behaves consistently with the chosen placeholder policy.
- Unusable range reads `0xFF` and ignores writes.
- I/O, HRAM, and IE read/write behavior is covered.

Do not build a ROM execution harness in Phase 1. Save minimal serial pass/fail signaling and deterministic ROM timeouts for inserted Phase 2.1, and broader blargg/mooneye validation expansion for Phase 4.

## Verification Commands

Run after each plan execution:

```bash
cargo fmt --all
cargo test
```

Useful optional smoke check after Plan 01 or Plan 02:

```bash
cargo run -- roms/hello-world/hello-world.gb
```

## Threat Model

ROM files are untrusted binary inputs. Implementation should avoid text decoding, unchecked indexing, panics on expected user failures, and silent interpretation of unknown header codes.

Mitigations:

- Use `std::fs::read` for bytes.
- Check `bytes.len() >= 0x150` before header indexing.
- Use safe slice access and explicit constants for offsets.
- Return custom errors for too-short ROMs, unsupported types, and unsupported size codes.
- Keep CLI errors human-readable and avoid `expect` for ordinary file problems.

Residual risk:

- Phase 1 does not validate Nintendo logo or header checksum as hard gates.
- Phase 1 does not prove broad ROM compatibility because CPU execution and ROM harnessing are deferred.
