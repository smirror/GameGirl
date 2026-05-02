# Roadmap: GameGirl

## Overview

GameGirl v1.0 turns the current Rust CLI scaffold into a testable DMG emulator core foundation. The roadmap starts with binary cartridge loading and a Bus-centered memory map, then adds a deliberately staged CPU core, inserts minimal serial/ROM validation before timer work, expands timing/interrupt validation, and finally adds PPU mode/access foundations before any full rendering, audio, CGB, or UI polish.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Cartridge and Bus Foundation** - Replace placeholder ROM text loading with binary cartridge parsing and a tested DMG Bus skeleton
- [ ] **Phase 2: CPU Core Foundation** - Add DMG CPU state, Bus-backed fetch/decode/execute flow, staged initial instruction groups, and cycle reporting
- [ ] **Phase 2.1: Minimal Serial and ROM Test Harness INSERTED** - Run small ROM fixtures under deterministic limits and capture serial test output
- [ ] **Phase 3: Timer and Interrupt Foundations** - Model timer registers, interrupt state, delayed `ei`, and interrupt service behavior
- [ ] **Phase 4: Automated Validation Harness Expansion** - Expand the early ROM harness into reproducible Cargo-based validation against capability-appropriate test ROMs
- [ ] **Phase 5: PPU Mode Skeleton and Access Rules** - Add cycle-driven PPU mode state, VRAM/OAM access restrictions, and interrupt hooks

## Phase Details

### Phase 1: Cartridge and Bus Foundation
**Goal**: User can load ROM bytes into a cartridge representation and exercise a Bus API for core DMG memory ranges.
**Depends on**: Nothing (first phase)
**Requirements**: [CART-01, CART-02, CART-03, CART-04, CART-05, BUS-01, BUS-02, BUS-03, BUS-04]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. User can run the CLI with a `.gb` path and the program reads binary bytes without UTF-8 assumptions.
  2. User gets clear errors for missing, unreadable, too-short, or invalid ROM inputs.
  3. Cartridge header parsing exposes title, type, ROM size, RAM size, and entry/header region data in tests.
  4. Non-ROM-only cartridge types are parsed far enough to report their type, then fail with `UnsupportedCartridgeType` until MBC support is added.
  5. Bus read/write tests cover cartridge ROM, WRAM, HRAM, interrupt enable, I/O, and representative unusable ranges.
**Plans**: 3 plans

Plans:
- [ ] 01-01: Replace text ROM loading with binary cartridge loading and CLI errors
- [ ] 01-02: Parse cartridge headers and model ROM-only cartridge reads
- [ ] 01-03: Introduce Bus address routing with focused tests

### Phase 2: CPU Core Foundation
**Goal**: Emulator has a DMG CPU core that fetches through the Bus, starts from documented post-boot state, executes initial instruction groups in staged slices, updates flags, and reports cycles.
**Depends on**: Phase 1
**Requirements**: [BOOT-01, CPU-01, CPU-02, CPU-03, CPU-04, CPU-05, CPU-06]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. v1.0 starts from documented post-boot DMG state and does not emulate the Nintendo boot ROM.
  2. CPU fetches opcodes through Bus and advances `PC` by instruction length.
  3. NOP, HALT/STOP placeholders, basic 8-bit loads, arithmetic/logical helpers, and control-flow/stack helpers are implemented through small plan slices with targeted unit tests.
  4. Each implemented instruction reports elapsed machine cycles for downstream device timing.
  5. CB-prefixed opcodes are either represented by a clear skeleton or explicitly deferred with deterministic unsupported behavior.
**Plans**: 5 plans

Plans:
- [ ] 02-01: Add CPU registers, flags, boot defaults, and fetch skeleton
- [ ] 02-02: Implement NOP, HALT/STOP placeholders, and basic 8-bit loads
- [ ] 02-03: Implement INC/DEC/ADD/SUB/AND/OR/XOR/CP with flag tests
- [ ] 02-04: Implement JP/JR/CALL/RET/RST and stack helpers
- [ ] 02-05: Add CB-prefix skeleton or explicit CB deferral plus cycle-returning step behavior

### Phase 2.1: Minimal Serial and ROM Test Harness INSERTED
**Goal**: Emulator can run small ROM fixtures under a deterministic step limit and collect serial test output.
**Depends on**: Phase 2
**Requirements**: [SERIAL-01, TEST-02, TEST-03]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. Harness can load a checked-in `.gb` fixture through the cartridge and Bus path.
  2. Emulator can run until timeout or an explicit pass/fail signal, and timeout is reported as failure.
  3. Serial output via `SB` (`0xFF01`) and `SC` (`0xFF02`) is captured into a test buffer when a test transfer is requested.
  4. At least one tiny custom ROM or capability-appropriate known test ROM fixture runs deterministically.
**Plans**: 2 plans

Plans:
- [ ] 02.1-01: Add minimal serial transfer register handling for ROM test output
- [ ] 02.1-02: Build deterministic ROM step harness and first fixture assertion

### Phase 3: Timer and Interrupt Foundations
**Goal**: Emulator can advance timer state from CPU cycles and service enabled DMG interrupts through shared `IE`/`IF`/`IME` behavior.
**Depends on**: Phase 2.1
**Requirements**: [TIME-01, TIME-02, TIME-03, INT-01, INT-02]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. Timer tests show `DIV`, `TIMA`, `TMA`, and `TAC` behavior driven by an internal system counter.
  2. `DIV` and `TAC` writes can trigger documented timer edge effects.
  3. `TIMA` overflow reload and timer interrupt request timing are covered by tests.
  4. CPU interrupt tests cover `IE`, `IF`, `IME`, delayed `ei`, vector jumps, `PC` push, and request clearing.
**Plans**: 3 plans

Plans:
- [ ] 03-01: Implement timer registers with internal counter and edge behavior
- [ ] 03-02: Implement interrupt registers, delayed `ei`, and interrupt service flow
- [ ] 03-03: Integrate CPU cycles with timer/interrupt progression and tests

### Phase 4: Automated Validation Harness Expansion
**Goal**: User can run reproducible Cargo tests that expand the early ROM harness into capability-appropriate blargg/mooneye validation with deterministic pass/fail reporting.
**Depends on**: Phase 3
**Requirements**: [TEST-01, TEST-03, TEST-04]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. Cargo tests cover cartridge parsing, Bus routing, CPU flags/instructions, timer behavior, and interrupt behavior introduced so far.
  2. The ROM harness can load and run at least one checked-in blargg or mooneye ROM matched to the implemented CPU/timer/interrupt capability.
  3. The harness documents and asserts its pass/fail signal, such as serial output or debug-break behavior.
  4. Test ROM selection is documented so unsupported subsystems do not masquerade as emulator failures.
**Plans**: 2 plans

Plans:
- [ ] 04-01: Consolidate unit/integration coverage for v1 core behavior
- [ ] 04-02: Expand ROM harness with capability-gated blargg/mooneye pass/fail reporting

### Phase 5: PPU Mode Skeleton and Access Rules
**Goal**: Emulator has enough PPU timing state for Bus access restrictions and interrupt hooks, without committing to full pixel rendering yet.
**Depends on**: Phase 4
**Requirements**: [PPU-01, PPU-02, PPU-03]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. PPU state advances dot, LY, and mode values from elapsed CPU cycles.
  2. Bus read/write tests enforce VRAM and OAM access restrictions based on PPU mode.
  3. PPU mode transitions can request VBlank and LCD-related interrupts through the shared interrupt path.
  4. Rendering, audio, CGB, and host UI remain explicitly deferred.
**Plans**: 3 plans

Plans:
- [ ] 05-01: Add PPU mode/dot/LY state machine skeleton
- [ ] 05-02: Enforce VRAM/OAM access restrictions through Bus tests
- [ ] 05-03: Connect PPU mode transitions to interrupt request hooks

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 2.1 -> 3 -> 4 -> 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Cartridge and Bus Foundation | 0/3 | Not started | - |
| 2. CPU Core Foundation | 0/5 | Not started | - |
| 2.1 Minimal Serial and ROM Test Harness INSERTED | 0/2 | Not started | - |
| 3. Timer and Interrupt Foundations | 0/3 | Not started | - |
| 4. Automated Validation Harness Expansion | 0/2 | Not started | - |
| 5. PPU Mode Skeleton and Access Rules | 0/3 | Not started | - |
