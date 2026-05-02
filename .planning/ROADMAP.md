# Roadmap: GameGirl v1.1

## Overview

GameGirl v1.1 hardens the emulator core around hardware-shaped boundaries before broader CPU/PPU compatibility work continues. The milestone turns cartridge loading into a single mapper-ready path, expands the Bus into a full DMG address-space router, replaces key I/O byte storage with device register handlers, makes boot policy explicit, adds M-cycle machine stepping, and finishes with staged MBC controller foundations plus a capability-gated ROM harness.

## Phases

**Phase Numbering:**
- Phase numbering continues from the previous milestone.
- v1.1 starts at Phase 6 because v1.0 used Phases 1-5.

- [ ] **Phase 6: Cartridge Header and Mapper Boundary** - Unify cartridge loading, enforce header/mode policy, and introduce a mapper-ready cartridge shape
- [ ] **Phase 7: Full Address Space and Peekable Interconnect** - Route every DMG CPU-visible range and separate execution reads from side-effect-free inspection
- [ ] **Phase 8: Side-Effect MMIO Device Skeletons** - Move key I/O registers behind joypad, timer, serial, interrupt, DMA, and PPU access-state devices
- [ ] **Phase 9: Boot Policy and M-Cycle Machine** - Add explicit boot modes and top-level machine stepping that advances devices from CPU machine cycles
- [ ] **Phase 10: MBC Controllers and ROM Harness** - Implement staged MBC1/MBC3/MBC5 basics and capability-gated ROM validation

## Phase Details

### Phase 6: Cartridge Header and Mapper Boundary

**Goal**: ROM loading produces one validated cartridge object with explicit header policy, CGB mode classification, and NoMbc mapper behavior.
**Depends on**: Phase 2 CPU Core Foundation
**Requirements**: [CART-06, CART-07, CART-08, CART-09, MAP-01, MAP-02]
**UI hint**: no

**Success Criteria**:
1. CLI and library loading both use the same cartridge construction path.
2. Header checksum status is computed and exposed, with strict/boot policy able to reject invalid headers.
3. CGB-only ROMs return a clear unsupported-mode error while DMG-compatible ROMs remain loadable.
4. Cartridge title/manufacturer/CGB metadata is parsed without fixed-title overlap mistakes.
5. CPU and Bus interact with cartridge ROM/control and external RAM through a mapper abstraction, with NoMbc implemented first.

**Plans**: 3 plans

Plans:
- [ ] 06-01: Unify cartridge loading and header checksum policy
- [ ] 06-02: Add CGB flag mode classification and title/manufacturer parsing
- [ ] 06-03: Introduce mapper trait with NoMbc implementation

### Phase 7: Full Address Space and Peekable Interconnect

**Goal**: Bus/Interconnect covers the full DMG CPU address space and exposes side-effect-free inspection reads.
**Depends on**: Phase 6
**Requirements**: [BUS-05, BUS-06, BUS-07, BUS-08, TEST-05]
**UI hint**: no

**Success Criteria**:
1. `0000-FFFF` routes through explicit cartridge, VRAM, external RAM, WRAM/Echo RAM, OAM, unusable, I/O, HRAM, and IE behavior.
2. `A000-BFFF` reads/writes route to mapper external RAM or mapper-backed windows.
3. `peek8` can inspect addresses without triggering execution-side effects.
4. VRAM/OAM access can be allowed or blocked through a PPU access-state hook.
5. Cargo tests cover the full memory map, Echo RAM mirroring, unusable behavior, mapper ERAM routing, and peek semantics.

**Plans**: 3 plans

Plans:
- [ ] 07-01: Expand Bus/Interconnect to full DMG address map
- [ ] 07-02: Add side-effect-free peek path
- [ ] 07-03: Add PPU access-state hooks and memory-map tests

### Phase 8: Side-Effect MMIO Device Skeletons

**Goal**: Key I/O registers become device-backed register handlers instead of anonymous byte-array storage.
**Depends on**: Phase 7
**Requirements**: [MMIO-01, MMIO-02, MMIO-03, MMIO-04, MMIO-05, TEST-06]
**UI hint**: no

**Success Criteria**:
1. `FF00` joypad reads reflect selected rows and active-low button state.
2. `FF04-FF07` timer/divider register reads/writes route through a Timer device with DIV reset and TAC/DIV edge hooks.
3. `FF01/FF02` serial behavior captures test output, clears transfer-start state, and can request serial interrupts.
4. `FF0F` and `FFFF` are shared interrupt state rather than unrelated bytes.
5. `FF46` starts DMA controller state and exposes active-DMA bus restrictions.
6. Cargo tests cover each MMIO side effect introduced in this phase.

**Plans**: 4 plans

Plans:
- [ ] 08-01: Add Joypad and Interrupts register devices
- [ ] 08-02: Add Timer register device with write side-effect hooks
- [ ] 08-03: Add Serial register behavior for ROM test output
- [ ] 08-04: Add OAM DMA controller skeleton and MMIO tests

### Phase 9: Boot Policy and M-Cycle Machine

**Goal**: Emulator startup and device advancement are coordinated by an explicit Machine layer.
**Depends on**: Phase 8
**Requirements**: [BOOT-02, BOOT-03, MACH-01, MACH-02, MACH-03]
**UI hint**: no

**Success Criteria**:
1. `SkipBootRom` initializes documented post-boot CPU/device defaults and starts execution at `0x0100`.
2. `UseBootRom` starts at `0x0000`, maps boot ROM reads over cartridge reads, and unmaps through the one-way `FF50` lock.
3. Machine owns CPU, cartridge, RAM, and device state while Interconnect routes CPU-visible access.
4. Each CPU instruction's `StepResult.machine_cycles` is consumed through a machine-level M-cycle tick loop.
5. Timer, serial, DMA, PPU access state, and interrupt hooks can advance without CPU opcode code knowing device internals.

**Plans**: 3 plans

Plans:
- [ ] 09-01: Add Machine ownership and Interconnect construction
- [ ] 09-02: Add BootMode policy and FF50 boot ROM mapping
- [ ] 09-03: Consume CPU machine cycles through device tick hooks

### Phase 10: MBC Controllers and ROM Harness

**Goal**: Mapper behavior expands beyond NoMbc and ROM-driven validation becomes capability-gated.
**Depends on**: Phase 9
**Requirements**: [MAP-03, MAP-04, TEST-07, TEST-08]
**UI hint**: no

**Success Criteria**:
1. MBC1 tests cover RAM enable, ROM bank selection, banking mode, upper bank bits, and bank-zero remapping.
2. MBC3 and MBC5 tests cover basic ROM/RAM banking while RTC and rumble effects remain explicit deferrals.
3. Mapper tests use focused fixtures rather than depending on broad commercial ROM behavior.
4. ROM harness can run selected capability-compatible fixtures with deterministic pass/fail/timeout results and serial-output capture.
5. Unsupported ROM-suite fixtures are classified by missing capability rather than counted as emulator regressions.

**Plans**: 4 plans

Plans:
- [ ] 10-01: Implement and test MBC1 banking behavior
- [ ] 10-02: Implement and test MBC3/MBC5 basic banking behavior
- [ ] 10-03: Build capability-gated ROM harness runner
- [ ] 10-04: Wire first mapper/MMIO-capable ROM validation fixtures

## Deferred Roadmap

v1.1 intentionally prepares hardware-shaped boundaries. Later milestones should use those boundaries to complete CPU opcodes, timer/interrupt precision, PPU rendering, APU behavior, save persistence, CGB execution, and specialty cartridge compatibility.

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 6. Cartridge Header and Mapper Boundary | 0/3 | Ready to plan | - |
| 7. Full Address Space and Peekable Interconnect | 0/3 | Not started | - |
| 8. Side-Effect MMIO Device Skeletons | 0/4 | Not started | - |
| 9. Boot Policy and M-Cycle Machine | 0/3 | Not started | - |
| 10. MBC Controllers and ROM Harness | 0/4 | Not started | - |
