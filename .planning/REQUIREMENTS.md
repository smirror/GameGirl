# Requirements: GameGirl

**Defined:** 2026-05-02
**Core Value:** GameGirl must execute DMG Game Boy ROMs accurately enough that behavior is driven by hardware rules and verified by known test ROMs, not by ad hoc demo success.

## v1 Requirements

Requirements for the initial emulator-core milestone. Each maps to roadmap phases.

### Cartridge

- [x] **CART-01**: User can provide a `.gb` or `.gbc` file path and the emulator reads the ROM as binary bytes.
- [x] **CART-02**: User receives a clear error when the ROM path is missing, unreadable, too short for a header, or invalid.
- [x] **CART-03**: Emulator can parse cartridge header fields needed for planning execution: title, cartridge type, ROM size, RAM size, and entry/header region.
- [x] **CART-04**: Emulator can construct a ROM-only cartridge representation that supports reads from fixed ROM address ranges.
- [x] **CART-05**: Non-ROM-only cartridge types are parsed far enough to report their type, then return `UnsupportedCartridgeType` until MBC support is added.

### Bus

- [x] **BUS-01**: Emulator exposes a Bus API that can read and write 8-bit values through the 16-bit DMG address space.
- [x] **BUS-02**: Bus routes reads for cartridge ROM, work RAM, high RAM, interrupt enable, and basic I/O register ranges.
- [x] **BUS-03**: Bus routes writes for writable memory and I/O registers without letting CPU instruction code bypass the Bus.
- [x] **BUS-04**: Bus behavior is covered by tests for representative address ranges and invalid/unusable ranges.

### Boot Model

- [ ] **BOOT-01**: v1.0 starts from documented post-boot DMG state, including `PC = 0x0100`, and does not emulate the Nintendo boot ROM.

### CPU

- [ ] **CPU-01**: Emulator can initialize DMG CPU registers and program counter/stack pointer to documented post-boot values.
- [ ] **CPU-02**: CPU fetches opcodes through the Bus and advances `PC` according to instruction length.
- [ ] **CPU-03**: CPU can execute initial load and control instructions needed for simple ROM startup.
- [ ] **CPU-04**: CPU can execute arithmetic/logical instruction helpers with correct `Z`, `N`, `H`, and `C` flag behavior.
- [ ] **CPU-05**: CPU can execute jump, call, return, and stack helpers needed for ROM control flow.
- [ ] **CPU-06**: Each implemented CPU instruction reports elapsed machine cycles for device timing.

### Timing

- [ ] **TIME-01**: Timer implements `DIV`, `TIMA`, `TMA`, and `TAC` using an internal system counter model.
- [ ] **TIME-02**: Timer increments `TIMA` from selected counter-bit falling edges and handles `DIV`/`TAC` write side effects.
- [ ] **TIME-03**: Timer handles `TIMA` overflow reload and timer interrupt request timing.
- [ ] **INT-01**: Emulator models `IE`, `IF`, and CPU-internal `IME`, including delayed `ei` behavior.
- [ ] **INT-02**: CPU can service enabled interrupts by clearing `IME`, clearing the requested `IF` bit, pushing `PC`, and jumping to the correct vector.

### Serial

- [ ] **SERIAL-01**: Emulator captures test ROM serial output through `SB` (`0xFF01`) and `SC` (`0xFF02`) by appending `SB` to a test buffer when `SC == 0x81`, without claiming full link-cable accuracy.

### Validation

- [ ] **TEST-01**: Cargo tests cover cartridge parsing, Bus address routing, CPU flags/instructions, timer behavior, and interrupt behavior introduced in v1.
- [ ] **TEST-02**: Emulator has a ROM test harness that can run at least one checked-in `.gb` fixture, custom or known-test, with a deterministic timeout.
- [ ] **TEST-03**: ROM harness can report pass/fail through a documented signal such as serial output or debug-break behavior.
- [ ] **TEST-04**: Expanded ROM validation can run selected blargg or mooneye ROMs matched to implemented CPU/timer/interrupt capability, with documented expectations.

### PPU

- [ ] **PPU-01**: Emulator has a PPU state skeleton that advances LY/dot/mode state from elapsed cycles.
- [ ] **PPU-02**: Bus enforces VRAM and OAM access restrictions based on current PPU mode.
- [ ] **PPU-03**: PPU mode transitions can request VBlank and LCD-related interrupts through the shared interrupt path.

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Rendering

- **REND-01**: Emulator can render DMG background tiles into a 160x144 frame buffer.
- **REND-02**: Emulator can render Window layer behavior.
- **REND-03**: Emulator can render Sprite/OAM behavior with scanline limits and priority rules.

### Input

- **INPT-01**: User can control Joypad state for D-pad, A, B, Select, and Start.
- **INPT-02**: Emulator can request Joypad interrupts on relevant input changes.

### Cartridge Expansion

- **MBC-01**: Emulator can run MBC1 ROMs.
- **MBC-02**: Emulator can run MBC3 ROMs with RAM behavior.
- **MBC-03**: Emulator can run MBC5 ROMs.
- **SAVE-01**: Emulator can persist and reload battery-backed external RAM.

### Audio and Host

- **APU-01**: Emulator can model DMG pulse, wave, and noise channel state.
- **APU-02**: Emulator can output audio through a host audio backend.
- **HOST-01**: User can run ROMs in an interactive desktop window.

### Compatibility

- **CGB-01**: Emulator can select and execute Color Game Boy behavior.

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| CGB support in v1 | DMG correctness is already a large hardware target; CGB adds banked VRAM/WRAM, palettes, and speed behavior |
| Full pixel rendering in v1 | PPU mode/access correctness should land before visual output |
| Audio output in v1 | APU is timing-sensitive and should follow CPU/bus/timer stability |
| Desktop UI in v1 | Core correctness and automated validation matter before host polish |
| Broad MBC support in v1 | Start with ROM-only cartridge behavior, then expand once Bus and cartridge boundaries are proven |
| Cycle-perfect completion for every subsystem | v1 establishes architecture and selected verified behavior; later phases tighten compatibility |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| CART-01 | Phase 1 | Complete |
| CART-02 | Phase 1 | Complete |
| CART-03 | Phase 1 | Complete |
| CART-04 | Phase 1 | Complete |
| CART-05 | Phase 1 | Complete |
| BUS-01 | Phase 1 | Complete |
| BUS-02 | Phase 1 | Complete |
| BUS-03 | Phase 1 | Complete |
| BUS-04 | Phase 1 | Complete |
| BOOT-01 | Phase 2 | Pending |
| CPU-01 | Phase 2 | Pending |
| CPU-02 | Phase 2 | Pending |
| CPU-03 | Phase 2 | Pending |
| CPU-04 | Phase 2 | Pending |
| CPU-05 | Phase 2 | Pending |
| CPU-06 | Phase 2 | Pending |
| SERIAL-01 | Phase 2.1 | Pending |
| TIME-01 | Phase 3 | Pending |
| TIME-02 | Phase 3 | Pending |
| TIME-03 | Phase 3 | Pending |
| INT-01 | Phase 3 | Pending |
| INT-02 | Phase 3 | Pending |
| TEST-01 | Phase 4 | Pending |
| TEST-02 | Phase 2.1 | Pending |
| TEST-03 | Phase 2.1 | Pending |
| TEST-04 | Phase 4 | Pending |
| PPU-01 | Phase 5 | Pending |
| PPU-02 | Phase 5 | Pending |
| PPU-03 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 29 total
- Mapped to phases: 29
- Unmapped: 0

---
*Requirements defined: 2026-05-02*
*Last updated: 2026-05-02 after Phase 1 completion*
