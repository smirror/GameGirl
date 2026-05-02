# Project Research Summary

**Project:** GameGirl
**Domain:** Rust DMG Game Boy emulator
**Researched:** 2026-05-02
**Confidence:** HIGH

## Executive Summary

GameGirl is best treated as a correctness-first emulator core project, not a UI project. The current repo is a Rust CLI scaffold with strong supporting materials: local Game Boy research, an implementation roadmap, and blargg/mooneye ROM fixtures. The first milestone should convert that scaffold into a byte-oriented cartridge loader, a Bus-centered hardware model, a CPU stepping foundation, and early validation tests.

The recommended stack is deliberately small: Rust stable, Cargo, rustfmt, Clippy, and standard library APIs for the core. The highest leverage architectural choice is to separate the CLI from a reusable emulator core, then route all CPU-visible memory through a Bus. The highest risk is timing correctness, especially timer/interrupt behavior; the roadmap should introduce cycle accounting early rather than trying to bolt it on after opcodes appear.

## Key Findings

### Recommended Stack

Use the existing Rust 2021 Cargo crate and keep Phase 1 dependency-free. The Rust standard library already provides byte-oriented ROM loading with `std::fs::read`, and Cargo gives enough testing/linting structure for the first emulator core phases.

**Core technologies:**
- Rust stable: emulator core and CLI - matches existing crate and supports safe binary parsing
- Cargo: build/test/lint - already present through `Cargo.toml` and `Cargo.lock`
- Rust std: filesystem and data structures - enough for cartridge, bus, CPU, and timer foundations

### Expected Features

**Must have (table stakes):**
- Binary ROM loading - users expect real `.gb` input to work
- Cartridge header parsing - required to identify ROM/mapper metadata
- DMG Bus/memory map - required for CPU-visible hardware behavior
- CPU register and instruction stepping - the core emulator loop
- Timer and interrupt foundations - needed for meaningful compatibility
- Test harness foundation - uses checked-in ROM suites for validation

**Should have (competitive):**
- Test-ROM-driven milestones - makes progress hard to fake
- Clean hardware module boundaries - makes timing fixes safer
- DMG-first scope discipline - prevents CGB complexity from derailing v1

**Defer (v2+):**
- CGB support - not essential until DMG is stable
- APU/audio output - high complexity and timing-sensitive
- Polished desktop UI - valuable later, not before core execution works

### Architecture Approach

Use a thin host boundary and reusable emulator core. `src/main.rs` should parse paths, load bytes, and report errors; `src/lib.rs` and focused modules should own cartridge, bus, CPU, timer, interrupts, PPU, joypad, and tests. CPU execution should go through Bus for all memory access and return elapsed cycles so timer/PPU/interrupt state can advance consistently.

**Major components:**
1. Cartridge - owns ROM bytes, parses header, and later handles MBC behavior
2. Bus - routes 16-bit reads/writes to cartridge, RAM, and I/O devices
3. CPU - owns registers, flags, instruction decode/execute, stack, and cycle results
4. Timer/Interrupts - model memory-mapped registers and CPU interrupt behavior
5. Test Harness - runs unit and ROM-level checks through Cargo

### Critical Pitfalls

1. **Treating ROMs as text** - replace `read_to_string` with `fs::read`
2. **Scattering memory map logic** - create Bus before broad opcode work
3. **Adding timing too late** - return cycles from CPU stepping from the start
4. **Incorrect timer model** - use internal system counter and edge behavior
5. **Test ROMs not harnessed** - wire fixtures into automated tests once execution can report pass/fail

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Cartridge and Bus Foundation
**Rationale:** Real binary input and a memory map must exist before CPU work can be meaningful.
**Delivers:** Byte ROM loading, cartridge header parsing, basic memory regions, Bus read/write API.
**Addresses:** ROM loading, cartridge parsing, memory map.
**Avoids:** Treating ROMs as text; scattered memory map logic.

### Phase 2: CPU Core Foundation
**Rationale:** CPU state and stepping are the next dependency for all ROM execution.
**Delivers:** Registers, flags, fetch/decode/execute loop, initial instruction families, cycle-returning step.
**Uses:** Bus API from Phase 1.
**Implements:** CPU component.

### Phase 3: Timer and Interrupt Foundations
**Rationale:** Many tests and games depend on timer/interrupt timing before graphics are useful.
**Delivers:** DIV/TIMA/TMA/TAC, IE/IF/IME, delayed `ei`, interrupt dispatch basics.
**Uses:** CPU cycles and Bus I/O registers.
**Implements:** Timer and interrupt components.

### Phase 4: Test ROM Harness
**Rationale:** The repo already has ROM fixtures; automation turns them into milestone gates.
**Delivers:** Cargo-based harness for a small initial blargg/mooneye subset with timeouts and pass/fail detection.
**Uses:** CPU, bus, timer, interrupt foundations.

### Phase 5: PPU Mode Skeleton and Access Rules
**Rationale:** PPU mode timing and VRAM/OAM restrictions should exist before pixel rendering.
**Delivers:** PPU modes, LY/dot stepping, VRAM/OAM access restrictions, VBlank/LCD interrupt hooks.
**Uses:** Bus and cycle advancement.

### Phase Ordering Rationale

- Binary ROM loading and Bus come first because every later subsystem depends on address routing.
- CPU comes before timer/interrupt harnessing because tests need execution.
- Timer/interrupts come before PPU rendering because timing correctness affects CPU tests and system behavior.
- ROM harness appears before visual output so compatibility remains measurable.
- PPU mode rules precede rendering to avoid building a display path on incorrect access timing.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** CPU instruction flags and cycle counts need careful source checks.
- **Phase 3:** Timer obscure behavior and interrupt timing need precise test-backed planning.
- **Phase 5:** PPU mode timing and VRAM/OAM restrictions need deeper source review.

Phases with standard patterns:
- **Phase 1:** Rust binary file loading, header parsing, and Bus scaffolding are straightforward.
- **Phase 4:** Harness shape is standard once pass/fail signal is chosen.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Existing Rust crate and official Rust docs support a minimal dependency-free core |
| Features | HIGH | Local docs, Pan Docs, and emulator test suites align on core needs |
| Architecture | HIGH | Bus-centered module split follows the hardware memory map and local docs |
| Pitfalls | HIGH | Current code already exhibits the binary/text loading pitfall; timer/interrupt pitfalls are documented |

**Overall confidence:** HIGH

### Gaps to Address

- CPU opcode implementation details: resolve during Phase 2 planning with instruction tables and targeted tests.
- ROM pass/fail protocol: choose exact detection strategy during Phase 4 planning.
- PPU rendering detail: defer until mode/access rules are in place.
- MBC scope: decide first mapper during cartridge planning; avoid broad mapper work too early.

## Sources

### Primary (HIGH confidence)

- https://doc.rust-lang.org/std/fs/fn.read.html - Rust byte file read API
- https://gbdev.io/pandocs/Memory_Map.html - memory map and cartridge header
- https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html - timer model and overflow behavior
- https://gbdev.io/pandocs/Interrupts.html - interrupt registers and `IME`
- https://github.com/Gekkio/mooneye-test-suite - test suite structure and pass/fail reporting

### Secondary (MEDIUM confidence)

- `docs/hot_to_proceed.md` - local implementation roadmap
- `docs/gameboy_architecture_summary.md` - local DMG implementation summary
- `.planning/codebase/*.md` - current brownfield codebase map

---
*Research completed: 2026-05-02*
*Ready for roadmap: yes*
