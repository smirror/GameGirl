# Roadmap: GameGirl

## Overview

GameGirl v1.0 turns the current Rust CLI scaffold into a testable DMG emulator core foundation. The roadmap starts with binary cartridge loading and a Bus-centered memory map, then adds CPU stepping, timing/interrupt behavior, automated ROM validation, and finally PPU mode/access foundations before any full rendering, audio, CGB, or UI polish.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Cartridge and Bus Foundation** - Replace placeholder ROM text loading with binary cartridge parsing and a tested DMG Bus skeleton
- [ ] **Phase 2: CPU Core Foundation** - Add DMG CPU state, fetch/decode/execute flow, initial instruction families, and cycle reporting
- [ ] **Phase 3: Timer and Interrupt Foundations** - Model timer registers, interrupt state, delayed `ei`, and interrupt service behavior
- [ ] **Phase 4: Automated Validation Harness** - Turn local test ROM assets into reproducible Cargo-based validation
- [ ] **Phase 5: PPU Mode Skeleton and Access Rules** - Add cycle-driven PPU mode state, VRAM/OAM access restrictions, and interrupt hooks

## Phase Details

### Phase 1: Cartridge and Bus Foundation
**Goal**: User can load ROM bytes into a cartridge representation and exercise a Bus API for core DMG memory ranges.
**Depends on**: Nothing (first phase)
**Requirements**: [CART-01, CART-02, CART-03, CART-04, BUS-01, BUS-02, BUS-03, BUS-04]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. User can run the CLI with a `.gb` path and the program reads binary bytes without UTF-8 assumptions.
  2. User gets clear errors for missing, unreadable, too-short, or invalid ROM inputs.
  3. Cartridge header parsing exposes title, type, ROM size, RAM size, and entry/header region data in tests.
  4. Bus read/write tests cover cartridge ROM, WRAM, HRAM, interrupt enable, I/O, and representative unusable ranges.
**Plans**: 3 plans

Plans:
- [ ] 01-01: Replace text ROM loading with binary cartridge loading and CLI errors
- [ ] 01-02: Parse cartridge headers and model ROM-only cartridge reads
- [ ] 01-03: Introduce Bus address routing with focused tests

### Phase 2: CPU Core Foundation
**Goal**: Emulator has a DMG CPU core that fetches through the Bus, executes initial instruction groups, updates flags, and reports cycles.
**Depends on**: Phase 1
**Requirements**: [CPU-01, CPU-02, CPU-03, CPU-04, CPU-05, CPU-06]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. CPU initializes DMG register, `PC`, and `SP` state to documented post-boot defaults.
  2. CPU fetches opcodes through Bus and advances `PC` by instruction length.
  3. Load/control, arithmetic/logical, jump/call/return, and stack helpers have targeted unit tests.
  4. Each implemented instruction reports elapsed machine cycles for downstream device timing.
**Plans**: 3 plans

Plans:
- [ ] 02-01: Add CPU registers, flags, boot defaults, and fetch skeleton
- [ ] 02-02: Implement initial load/control and arithmetic/logical helpers with flag tests
- [ ] 02-03: Implement control-flow/stack helpers and cycle-returning step behavior

### Phase 3: Timer and Interrupt Foundations
**Goal**: Emulator can advance timer state from CPU cycles and service enabled DMG interrupts through shared `IE`/`IF`/`IME` behavior.
**Depends on**: Phase 2
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

### Phase 4: Automated Validation Harness
**Goal**: User can run reproducible Cargo tests that exercise emulator behavior and at least one checked-in ROM fixture with deterministic pass/fail reporting.
**Depends on**: Phase 3
**Requirements**: [TEST-01, TEST-02, TEST-03]
**UI hint**: no
**Success Criteria** (what must be TRUE):
  1. Cargo tests cover cartridge parsing, Bus routing, CPU flags/instructions, timer behavior, and interrupt behavior introduced so far.
  2. A ROM harness can load and run at least one checked-in blargg or mooneye ROM with a deterministic timeout.
  3. The harness documents and asserts its pass/fail signal, such as serial output or debug-break behavior.
**Plans**: 2 plans

Plans:
- [ ] 04-01: Consolidate unit/integration coverage for v1 core behavior
- [ ] 04-02: Build the initial ROM test harness with pass/fail reporting

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
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Cartridge and Bus Foundation | 0/3 | Not started | - |
| 2. CPU Core Foundation | 0/3 | Not started | - |
| 3. Timer and Interrupt Foundations | 0/3 | Not started | - |
| 4. Automated Validation Harness | 0/2 | Not started | - |
| 5. PPU Mode Skeleton and Access Rules | 0/3 | Not started | - |
