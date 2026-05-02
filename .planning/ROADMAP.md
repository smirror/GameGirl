# Roadmap: GameGirl v1.1

## Overview

GameGirl v1.1 hardens the emulator core around hardware-shaped boundaries before
broader CPU/PPU compatibility work continues. The milestone turns cartridge
loading into a single mapper-ready path, expands the Bus into a full DMG
address-space router, replaces key I/O byte storage with device register
handlers, makes boot policy explicit, adds M-cycle machine stepping, and finishes
with staged MBC controller foundations plus a capability-gated ROM harness.

Detailed subsystem behavior is captured in `docs/gameboy_implementation_spec.md`.
This roadmap intentionally breaks the milestone into smaller implementation
slices so each plan can be reviewed, tested, and committed without carrying too
many hardware rules at once.

## Phases

**Phase Numbering:**
- Phase numbering continues from the previous milestone.
- v1.1 starts at Phase 6 because v1.0 used Phases 1-5.

- [ ] **Phase 6: Cartridge Header and Mapper Boundary** - Unify cartridge loading, enforce header/mode policy, and introduce a mapper-ready cartridge shape
- [ ] **Phase 7: Full Address Space and Peekable Interconnect** - Route every DMG CPU-visible range and separate execution reads from side-effect-free inspection
- [ ] **Phase 8: Side-Effect MMIO Device Skeletons** - Move key I/O registers behind joypad, timer, serial, interrupt, DMA, and PPU access-state devices
- [ ] **Phase 9: Boot Policy and M-Cycle Machine** - Add explicit boot modes and top-level machine stepping that advances devices from CPU machine cycles
- [ ] **Phase 10: MBC Controllers and ROM Harness** - Implement staged MBC1/MBC3/MBC5 basics and capability-gated ROM validation

## Plan Granularity Contract

Each plan should be small enough to satisfy these constraints:

1. It introduces one boundary or hardware rule cluster.
2. It has focused unit tests or a deterministic harness check.
3. It does not require broad CPU/PPU rendering compatibility to be considered done.
4. It leaves unsupported behavior explicit through typed errors, stubs, or documented deferrals.

## Phase Details

### Phase 6: Cartridge Header and Mapper Boundary

**Goal**: ROM loading produces one validated cartridge object with explicit header policy, CGB mode classification, size validation, and NoMbc mapper behavior.
**Depends on**: Phase 2 CPU Core Foundation
**Requirements**: [CART-06, CART-07, CART-08, CART-09, MAP-01, MAP-02]
**UI hint**: no

**Success Criteria**:
1. CLI and library loading both use the same cartridge construction path.
2. Header checksum status is computed and exposed, with strict/boot policy able to reject invalid headers.
3. CGB-only ROMs return a clear unsupported-mode error while DMG-compatible ROMs remain loadable.
4. Cartridge title/manufacturer/CGB metadata is parsed without fixed-title overlap mistakes.
5. Declared ROM/RAM size codes are checked against actual byte lengths and supported mapper capacity.
6. CPU and Bus interact with cartridge ROM/control and external RAM through a mapper abstraction, with NoMbc implemented first.

**Plans**: 6 plans

Plans:
- [ ] 06-01: Collapse ROM loading onto one cartridge construction path
  - Route CLI file loading and library loading through `Cartridge::from_bytes` or a single `load_cartridge_file` helper.
  - Remove or demote duplicate raw-byte validation that can drift from cartridge parsing.
  - Preserve user-facing CLI errors for unreadable, too-short, unsupported, or invalid files.
  - Verify with CLI integration tests and cartridge unit tests.
- [ ] 06-02: Add header checksum, logo, and global-checksum policy
  - Compute header checksum from the cartridge header bytes and expose expected/actual values.
  - Decide strict/default behavior: invalid header checksum can fail strict or boot-compatible mode; global checksum remains metadata.
  - Keep Nintendo logo validation available without making unofficial ROM development painful by default.
  - Verify with generated in-memory ROM fixtures.
- [ ] 06-03: Parse CGB flag, title variants, and manufacturer metadata
  - Interpret `0x0143` as the CGB flag instead of blindly including it in a fixed 16-byte title.
  - Parse old title layout, newer title/manufacturer split, CGB compatibility, SGB flag, and licensee bytes.
  - Add `UnsupportedMode::CgbOnly` or equivalent for `0xC0` while allowing `0x80` in DMG mode.
  - Verify title parsing for old-style and CGB-aware headers.
- [ ] 06-04: Tighten ROM/RAM size code and length validation
  - Map supported ROM size codes to expected byte lengths and reject actual length mismatches.
  - Map RAM size codes to external RAM capacity while leaving ambiguous/unsupported codes explicit.
  - Keep nonstandard or unsupported size codes as structured errors rather than silent fallbacks.
  - Verify with minimum, undersized, oversized, and unsupported-code fixtures.
- [ ] 06-05: Introduce mapper trait and NoMbc implementation
  - Move runtime cartridge access behind `Mapper` methods for ROM reads, control writes, external RAM reads, and external RAM writes.
  - Implement NoMbc with fixed ROM reads, ignored control writes, and optional external RAM window behavior.
  - Keep Bus/CPU code independent from concrete cartridge type conditionals.
  - Verify mapper behavior through direct unit tests and Bus-backed reads.
- [ ] 06-06: Refresh cartridge docs and fixtures
  - Update public docs and inline Rustdoc where cartridge behavior is now stable.
  - Add reusable fixture builders for valid headers, invalid checksums, CGB flags, and mapper variants.
  - Ensure `docs/gameboy_implementation_spec.md` and planning artifacts describe the implemented policy.
  - Verify with `cargo test` and `cargo fmt --all -- --check`.

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

**Plans**: 6 plans

Plans:
- [ ] 07-01: Define memory storage types and address constants
  - Add focused storage for WRAM, HRAM, VRAM placeholder, OAM placeholder, and IE.
  - Centralize address constants for DMG memory ranges.
  - Add little-endian `read16`/`write16` helpers only if they route through `read8`/`write8`.
  - Verify boundary reads/writes at every range edge.
- [ ] 07-02: Route cartridge ROM/control and external RAM windows
  - Route `0000-7FFF` reads and writes through mapper ROM/control methods.
  - Route `A000-BFFF` through mapper external RAM methods.
  - Preserve correct behavior for NoMbc and unsupported mapper states.
  - Verify fixed ROM, control-write, and ERAM access tests.
- [ ] 07-03: Implement WRAM, Echo RAM, HRAM, IE, and unusable ranges
  - Store `C000-DFFF` as WRAM and mirror `E000-FDFF` to `C000-DDFF`.
  - Store `FF80-FFFE` as HRAM and `FFFF` as interrupt enable.
  - Treat `FEA0-FEFF` as an explicit unusable range rather than ordinary RAM.
  - Verify mirror writes, mirror reads, edge addresses, and ignored writes.
- [ ] 07-04: Add VRAM/OAM placeholders and PPU access-state hook
  - Introduce a minimal PPU memory/access facade for `8000-9FFF` and `FE00-FE9F`.
  - Support an access-state query that lets Bus decide whether CPU access is allowed.
  - Return deterministic blocked-read values and ignore blocked writes.
  - Verify Mode 0/1 allowed, Mode 2 OAM-blocked, and Mode 3 VRAM/OAM-blocked cases.
- [ ] 07-05: Split execution reads from side-effect-free peek
  - Add `peek8` for debugger, logger, disassembler, and tests.
  - Ensure `peek8` can inspect MMIO-facing ranges without starting DMA, changing serial state, clearing flags, or mutating counters.
  - Keep `read8`/`write8` as the execution path with side effects.
  - Verify by comparing read and peek behavior on representative MMIO stubs.
- [ ] 07-06: Build the full memory-map test matrix
  - Add parameterized tests for range starts, middles, ends, and cross-range boundaries.
  - Cover cartridge ROM, mapper ERAM, VRAM, WRAM, Echo RAM, OAM, unusable, I/O, HRAM, and IE.
  - Document expected default values for temporarily stubbed I/O ranges.
  - Verify with `cargo test`.

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

**Plans**: 7 plans

Plans:
- [ ] 08-01: Add shared Interrupts register state
  - Introduce an `Interrupts` type for `IE`, `IF`, request/set/clear helpers, and priority queries.
  - Route `FF0F` and `FFFF` through this state.
  - Keep CPU IME separate from mapped registers.
  - Verify read/write masking, request helpers, and shared state through Bus.
- [ ] 08-02: Add Joypad device and `FF00` routing
  - Model d-pad/action row select bits and active-low button reads.
  - Add a button-state API for frontend/test code.
  - Request Joypad interrupt on selected high-to-low transitions.
  - Verify row selection, pressed/released values, and interrupt request behavior.
- [ ] 08-03: Add Timer register device with M-cycle-ready internals
  - Model `DIV`, `TIMA`, `TMA`, and `TAC` behind a Timer type.
  - Reset DIV on write and expose hooks for TAC/DIV falling-edge behavior.
  - Keep overflow/reload state machine shaped for later mooneye-level precision.
  - Verify register reads/writes, basic frequency behavior, overflow, and IF request.
- [ ] 08-04: Add Serial register behavior for test ROM output
  - Model `SB` and `SC`.
  - Capture a byte when transfer-start/internal-clock behavior is requested.
  - Clear transfer-start state and request Serial interrupt deterministically.
  - Verify serial output buffer contents and interrupt request behavior.
- [ ] 08-05: Add OAM DMA controller skeleton
  - Route `FF46` writes into a DMA controller state rather than an inert byte.
  - Track source high byte, active/inactive state, elapsed M-cycles, and last written value.
  - Add bus restriction hooks for active DMA, even if the first copy implementation is still simple.
  - Verify start state, progress ticking, completion, and blocked access policy.
- [ ] 08-06: Add minimal PPU register/access facade
  - Route `FF40-FF4B` through a PPU register type where practical.
  - Keep `LY`, `STAT` mode bits, and access-state queries coherent enough for Bus tests.
  - Avoid full rendering in this phase.
  - Verify LCD/STAT register defaults, write masks, and access-state transitions.
- [ ] 08-07: Consolidate MMIO integration tests
  - Add tests that exercise device interactions through Bus rather than direct struct-only tests.
  - Cover joypad, timer, serial, interrupts, DMA, boot-lock placeholder, and PPU access hooks.
  - Document any intentionally stubbed MMIO behavior.
  - Verify with `cargo test` and `cargo clippy`.

### Phase 9: Boot Policy and M-Cycle Machine

**Goal**: Emulator startup and device advancement are coordinated by an explicit Machine or GameBoy layer.
**Depends on**: Phase 8
**Requirements**: [BOOT-02, BOOT-03, MACH-01, MACH-02, MACH-03]
**UI hint**: no

**Success Criteria**:
1. `SkipBootRom` initializes documented post-boot CPU/device defaults and starts execution at `0x0100`.
2. `UseBootRom` starts at `0x0000`, maps boot ROM reads over cartridge reads, and unmaps through the one-way `FF50` lock.
3. Machine owns CPU, cartridge, RAM, and device state while Interconnect routes CPU-visible access.
4. Each CPU instruction's `StepResult.machine_cycles` is consumed through a machine-level M-cycle tick loop.
5. Timer, serial, DMA, PPU access state, and interrupt hooks can advance without CPU opcode code knowing device internals.

**Plans**: 6 plans

Plans:
- [ ] 09-01: Introduce GameBoy/Machine ownership and options
  - Add a top-level owner for CPU, cartridge, RAM, devices, boot policy, and model/mode.
  - Keep CLI construction thin and library-friendly.
  - Add `EmulatorOptions` or equivalent for boot mode, strict header policy, and headless behavior.
  - Verify construction failures and default options.
- [ ] 09-02: Implement `SkipBootRom` initialization
  - Centralize documented DMG post-boot CPU defaults and supported MMIO defaults.
  - Start execution at `0x0100` with boot ROM disabled.
  - Avoid mixing skip-boot defaults into boot-ROM execution mode.
  - Verify CPU registers, boot lock state, and selected MMIO defaults.
- [ ] 09-03: Implement `UseBootRom` overlay and `FF50` lock
  - Accept a boot ROM byte buffer or path without bundling copyrighted bytes.
  - Route low-address reads through boot ROM while mapped.
  - Unmap permanently on `FF50` write and route subsequent reads to cartridge.
  - Verify overlay reads, cartridge fallback, and one-way lock behavior.
- [ ] 09-04: Extract Interconnect borrowing and routing surface
  - Provide a CPU-facing routing object that can borrow Machine-owned devices safely.
  - Keep device ownership outside CPU.
  - Preserve `read8`, `write8`, and `peek8` semantics.
  - Verify CPU fetch/stack helpers still operate through the new routing surface.
- [ ] 09-05: Consume CPU machine cycles through device ticks
  - Update `step_instruction` to tick devices once per reported M-cycle.
  - Tick Timer, Serial, DMA, PPU access state, and interrupt hooks in a deterministic order.
  - Keep exact obscure timing hooks available for future refinement.
  - Verify per-instruction tick counts with fake or instrumented devices.
- [ ] 09-06: Add machine-level run controls
  - Add bounded stepping helpers such as instruction limit, frame limit, or until-condition loops.
  - Return structured `StepReport`/`RunReport` values for harness and CLI use.
  - Ensure HALT/STOP placeholder behavior does not deadlock tests.
  - Verify deterministic stop reasons: limit reached, frame ready, serial pass/fail, or error.

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

**Plans**: 7 plans

Plans:
- [ ] 10-01: Build mapper fixture and test infrastructure
  - Add helpers for synthetic ROM banks, RAM banks, headers, and mapper construction.
  - Make bank identity easy to assert without commercial ROM dependencies.
  - Add shared mapper behavior tests for ROM reads, RAM enable, RAM reads/writes, and ignored writes.
  - Verify NoMbc still passes through the shared fixture layer.
- [ ] 10-02: Implement and test MBC1
  - Implement RAM enable, lower five ROM bank bits, upper bank bits, banking mode, and external RAM bank selection.
  - Handle MBC1 bank-zero remapping rules.
  - Keep unsupported edge cases explicit if they require later accuracy work.
  - Verify with focused bank-switching and RAM-mode tests.
- [ ] 10-03: Implement and test MBC3 basic banking
  - Implement MBC3 ROM bank select, RAM bank select, and RAM enable.
  - Stub RTC register selection/latching with explicit deferred behavior.
  - Ensure RTC deferral cannot be mistaken for ordinary RAM success.
  - Verify ROM/RAM bank behavior and RTC-deferred errors or placeholder reads.
- [ ] 10-04: Implement and test MBC5 basic banking
  - Implement 9-bit ROM bank selection and RAM bank selection.
  - Defer rumble effects explicitly.
  - Ensure bank 0 selection follows MBC5 rules rather than MBC1 remapping.
  - Verify high-bank reads, RAM bank routing, and rumble-bit handling.
- [ ] 10-05: Add battery/external RAM lifecycle hooks without persistence
  - Track external RAM capacity, dirty state, and optional battery metadata.
  - Expose save/load hooks or accessors without committing to filesystem persistence in v1.1.
  - Keep `.sav` writing out of scope unless a later plan explicitly pulls it in.
  - Verify dirty flag behavior and RAM capacity enforcement.
- [ ] 10-06: Build capability-gated ROM harness runner
  - Load ROM fixtures through normal cartridge and machine construction paths.
  - Run with instruction/frame/time limits and deterministic stop reasons.
  - Capture serial output for blargg-style pass/fail signals.
  - Verify timeout, pass, fail, unsupported capability, and error reporting paths.
- [ ] 10-07: Wire first mapper/MMIO-capable validation fixtures
  - Add a manifest or code-level registry that names each fixture's required capabilities.
  - Enable only fixtures compatible with implemented CPU/MMIO/mapper behavior.
  - Document skipped fixtures so unsupported hardware does not look like a regression.
  - Verify harness output in Cargo tests or an explicitly skipped integration suite.

## Deferred Roadmap

v1.1 intentionally prepares hardware-shaped boundaries. Later milestones should
use those boundaries to complete CPU opcodes, timer/interrupt precision, PPU
rendering, APU behavior, save persistence, CGB execution, and specialty
cartridge compatibility.

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 6. Cartridge Header and Mapper Boundary | 0/6 | Ready to plan | - |
| 7. Full Address Space and Peekable Interconnect | 0/6 | Not started | - |
| 8. Side-Effect MMIO Device Skeletons | 0/7 | Not started | - |
| 9. Boot Policy and M-Cycle Machine | 0/6 | Not started | - |
| 10. MBC Controllers and ROM Harness | 0/7 | Not started | - |
