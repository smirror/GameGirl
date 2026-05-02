# GameGirl

## What This Is

GameGirl is a Rust Game Boy emulator project. The repository currently contains a minimal CLI scaffold, implementation notes, Game Boy hardware research, and a substantial local ROM corpus for future validation.

The immediate product direction is a DMG-first emulator core that can load real `.gb` ROM bytes, model the CPU/bus/devices accurately enough to run test ROMs, and grow toward rendering, input, cartridges, and audio in deliberate phases.

## Core Value

GameGirl must execute DMG Game Boy ROMs accurately enough that behavior is driven by hardware rules and verified by known test ROMs, not by ad hoc demo success.

## Current Milestone: v1.1 Hardware-Accurate Core Architecture

**Goal:** Harden the emulator core around hardware-shaped cartridge, bus, MMIO, boot, and timing boundaries before adding broader CPU/PPU behavior.

**Target features:**
- Single cartridge loading path with header policy, CGB flag handling, and mapper-ready construction.
- Full DMG CPU address-space routing with side-effect-aware MMIO devices and a separate side-effect-free peek path.
- Mapper abstraction with NoMbc plus staged MBC1, MBC3, and MBC5 foundations.
- Boot mode policy and `FF50` mapping behavior that distinguish skip-boot and boot-ROM execution.
- M-cycle machine/interconnect stepping that can advance timer, serial, DMA, PPU, and interrupt hooks from CPU cycles.

## Requirements

### Validated

- ✓ Rust binary crate exists with Cargo metadata — existing
- ✓ CLI entry point accepts a ROM path argument and rejects paths without `.gb` or `.gbc` suffixes — existing
- ✓ Project includes local validation ROM suites for future emulator tests — existing
- ✓ Project includes DMG-focused implementation research and a subsystem roadmap — existing
- ✓ Load cartridge files as binary data and parse enough header metadata to identify the ROM and initial cartridge behavior — Phase 1
- ✓ Define a DMG memory bus with 16-bit addressing and 8-bit read/write access — Phase 1
- ✓ Initialize DMG CPU registers and execute the Phase 2 load/ALU/control-flow CPU foundation through the Bus — Phase 2

### Active

- [ ] Unify ROM loading so CLI and core paths both construct validated cartridge objects through one policy.
- [ ] Interpret cartridge header mode metadata, including CGB flag handling, without using file extension as hardware mode.
- [ ] Introduce a mapper abstraction that supports NoMbc immediately and can host MBC1, MBC3, and MBC5 without changing CPU code.
- [ ] Route the full DMG CPU address space through hardware-shaped devices, including VRAM, external RAM, WRAM/Echo RAM, OAM, unusable memory, I/O, HRAM, and IE.
- [ ] Replace plain byte-array I/O behavior for key registers with side-effect-aware device register handlers.
- [ ] Define boot mode and M-cycle stepping boundaries so timers, serial, DMA, PPU, and interrupts can advance from CPU execution.
- [ ] Add capability-gated tests that verify address-space routing, MMIO side effects, mapper behavior, boot policy, and deterministic ROM harness output.

### Out of Scope

- Full CGB execution — v1.1 may classify or reject CGB-only ROMs, but does not implement CGB VRAM/WRAM banking, palettes, or double-speed behavior.
- Full PPU rendering — v1.1 prepares VRAM/OAM and mode access boundaries but does not render frames.
- Full APU/audio output — serial, timer, DMA, and PPU timing boundaries come first.
- Complete boot ROM implementation or bundled Nintendo boot ROM images — v1.1 defines the policy and mapping hooks, then can execute a user-supplied boot ROM later.
- Exotic cartridge hardware beyond staged MBC1/MBC3/MBC5 foundations — specialty cartridges remain compatibility-expansion work.
- Perfect every timing edge case — v1.1 introduces an M-cycle architecture and targeted tests; later milestones can tighten obscure hardware behavior.

## Context

- The codebase now has a thin CLI plus reusable `cartridge`, `bus`, and `cpu` modules. Phase 2 added post-boot CPU defaults, Bus-backed stepping, initial load/ALU/control-flow instructions, cycle reporting, and deterministic unsupported opcode errors.
- `docs/hot_to_proceed.md` recommends a Bus-centered design with CPU, PPU, Timer, Joypad, and Cartridge decoupled behind memory access.
- `docs/gameboy_architecture_summary.md` narrows the initial target to DMG behavior, including the 16-bit memory map, LR35902/SM83 register model, interrupt priority, timer edge behavior, PPU modes, and VRAM/OAM access constraints.
- The repository includes blargg and mooneye ROM suites under `roms/`, which should become the backbone for regression checks.
- The local codebase map is in `.planning/codebase/` and should be used before planning implementation phases.
- A May 2026 implementation review identified cartridge and bus/MMIO boundaries as the highest-leverage design risk: CPU bugs will be hard to diagnose if memory-mapped side effects, mapper writes, boot mapping, and device timing stay hidden behind plain byte arrays.
- Primary references for v1.1 are Pan Docs, Game Boy: Complete Technical Reference, mooneye-test-suite, and checked-in blargg/mooneye ROM fixtures.

## Constraints

- **Tech stack**: Rust 2021 with Cargo — match the existing crate and avoid adding dependencies unless they remove real implementation risk.
- **Compatibility**: DMG-first — keep Color Game Boy behavior out of the initial critical path.
- **Correctness**: Use test ROMs as a decision aid when documentation and intuition disagree.
- **Architecture**: Keep CPU, bus, cartridge, timer, PPU, joypad, and eventual APU responsibilities separated so hardware timing rules can be tested in isolation.
- **Input data**: ROMs are untrusted binary files — cartridge parsing must be bounds-checked and avoid unsafe Rust by default.
- **Validation**: Prefer small unit tests for pure CPU/device behavior and ROM-driven integration tests for compatibility.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Map the brownfield codebase before initialization | Existing code and docs should define validated context instead of treating the repo as blank | ✓ Good |
| Target DMG behavior first | The docs and available tests support a focused first emulator target, while CGB multiplies hardware variance | — Pending |
| Build around a Bus abstraction | CPU-visible memory behavior spans cartridge, RAM, timer, PPU, joypad, interrupts, and later audio | ✓ Phase 1 |
| Replace text ROM loading with byte loading before emulator work | Game Boy ROMs are binary files and the current UTF-8 path is only a placeholder | ✓ Phase 1 |
| Use blargg/mooneye ROMs for milestone validation | Emulator correctness depends on edge cases that ordinary demos can miss | — Pending |
| Insert a minimal serial ROM harness before timer/interrupt work | CPU changes should encounter ROM-style failures early instead of waiting for a broad validation phase | — Pending |
| Skip Nintendo boot ROM emulation in v1.0 | Starting at documented post-boot DMG state keeps the first milestone focused and aligns CPU defaults around `PC = 0x0100` | — Pending |
| Treat Bus as an address router, not the long-term owner of every device byte | MMIO, cartridge, boot ROM mapping, DMA, timer, serial, and PPU access all have side effects that plain arrays cannot express | — Pending |
| Continue phase numbering into v1.1 | Existing v1.0 phase history remains useful context; v1.1 should start at Phase 6 rather than reusing Phase 1 labels | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check - still the right priority?
3. Audit Out of Scope - reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-05-02 after v1.1 milestone initialization*
