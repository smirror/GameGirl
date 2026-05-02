# GameGirl

## What This Is

GameGirl is a Rust Game Boy emulator project. The repository currently contains a minimal CLI scaffold, implementation notes, Game Boy hardware research, and a substantial local ROM corpus for future validation.

The immediate product direction is a DMG-first emulator core that can load real `.gb` ROM bytes, model the CPU/bus/devices accurately enough to run test ROMs, and grow toward rendering, input, cartridges, and audio in deliberate phases.

## Core Value

GameGirl must execute DMG Game Boy ROMs accurately enough that behavior is driven by hardware rules and verified by known test ROMs, not by ad hoc demo success.

## Requirements

### Validated

- ✓ Rust binary crate exists with Cargo metadata — existing
- ✓ CLI entry point accepts a ROM path argument and rejects paths without `.gb` or `.gbc` suffixes — existing
- ✓ Project includes local validation ROM suites for future emulator tests — existing
- ✓ Project includes DMG-focused implementation research and a subsystem roadmap — existing
- ✓ Load cartridge files as binary data and parse enough header metadata to identify the ROM and initial cartridge behavior — Phase 1
- ✓ Define a DMG memory bus with 16-bit addressing and 8-bit read/write access — Phase 1

### Active

- [ ] Start execution from documented post-boot DMG state rather than emulating the Nintendo boot ROM in v1.0
- [ ] Implement CPU register state, instruction fetch/decode/execute flow, flags, stack behavior, and staged core instruction groups
- [ ] Capture minimal serial output from `FF01`/`FF02` so ROM fixtures can report test results early
- [ ] Add timer and interrupt state using hardware-oriented timing rules, including delayed interrupt enable behavior
- [ ] Introduce PPU mode timing and VRAM/OAM access restrictions before full rendering
- [ ] Run automated checks against local blargg and mooneye ROM fixtures as emulator capabilities appear

### Out of Scope

- CGB-specific behavior — defer until a DMG core is stable
- Full audio output — defer until CPU, bus, timer, interrupts, PPU timing, and basic cartridge execution are grounded
- Polished desktop UI — defer until the emulator core can run meaningful ROMs
- Broad MBC coverage beyond the first cartridge path — start with the simplest supported cartridge behavior before expanding
- Nintendo boot ROM emulation in v1.0 — start from documented post-boot DMG state and make the assumption explicit
- Cycle-perfect edge cases everywhere in the first milestone — capture architecture that can support precision, then tighten with tests

## Context

- The codebase now has a thin CLI plus reusable `cartridge` and `bus` modules. Phase 2 should build CPU fetch/decode/execute through the Bus from documented post-boot state.
- `docs/hot_to_proceed.md` recommends a Bus-centered design with CPU, PPU, Timer, Joypad, and Cartridge decoupled behind memory access.
- `docs/gameboy_architecture_summary.md` narrows the initial target to DMG behavior, including the 16-bit memory map, LR35902/SM83 register model, interrupt priority, timer edge behavior, PPU modes, and VRAM/OAM access constraints.
- The repository includes blargg and mooneye ROM suites under `roms/`, which should become the backbone for regression checks.
- The local codebase map is in `.planning/codebase/` and should be used before planning implementation phases.

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
*Last updated: 2026-05-02 after Phase 1 completion*
