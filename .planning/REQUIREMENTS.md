# Requirements: GameGirl

**Defined:** 2026-05-02
**Milestone:** v1.1 Hardware-Accurate Core Architecture
**Core Value:** GameGirl must execute DMG Game Boy ROMs accurately enough that behavior is driven by hardware rules and verified by known test ROMs, not by ad hoc demo success.

## v1.1 Requirements

Requirements for hardening the emulator core around hardware-shaped cartridge, bus, MMIO, boot, mapper, timing, and validation boundaries.

### Cartridge Header Policy

- [ ] **CART-06**: User can load ROM files through one cartridge construction path that returns validated cartridge state and avoids duplicated CLI/core validation logic.
- [ ] **CART-07**: Emulator can compute and expose cartridge header checksum status, treating invalid header checksums as a strict/boot-policy failure while keeping global checksum as metadata.
- [ ] **CART-08**: Emulator can interpret the CGB flag at `0x0143`, allow DMG-compatible ROMs in DMG mode, and reject CGB-only ROMs with a clear error while CGB execution is out of scope.
- [ ] **CART-09**: Emulator can parse title/manufacturer/CGB header fields without treating the CGB flag byte as part of a fixed 16-byte title in newer cartridges.

### Mapper Layer

- [ ] **MAP-01**: Emulator exposes a mapper abstraction so CPU and Bus code access cartridge ROM/control and external RAM through mapper methods instead of cartridge-type conditionals.
- [ ] **MAP-02**: Emulator supports NoMbc cartridges through the mapper abstraction, including fixed ROM reads, optional external RAM window behavior, and ignored control writes.
- [ ] **MAP-03**: Emulator supports MBC1 ROM/RAM banking behavior, including RAM enable, ROM bank selection, RAM bank or upper-ROM-bank mode, and bank-zero remapping rules.
- [ ] **MAP-04**: Emulator supports MBC3 and MBC5 basic ROM/RAM banking behavior while explicitly deferring MBC3 RTC ticking and MBC5 rumble effects.

### Bus And Address Space

- [ ] **BUS-05**: Emulator routes every DMG CPU-visible address range through the Bus or Interconnect, including cartridge ROM/control, VRAM, external RAM, WRAM, Echo RAM, OAM, unusable memory, I/O, HRAM, and IE.
- [ ] **BUS-06**: Emulator separates execution reads/writes from side-effect-free `peek` reads so debugging or disassembly cannot trigger MMIO actions.
- [ ] **BUS-07**: Emulator can enforce VRAM and OAM CPU access restrictions through a PPU access-state hook, even before full pixel rendering exists.
- [ ] **BUS-08**: Emulator routes `A000-BFFF` external cartridge RAM and mapper-backed special windows through the mapper rather than returning a generic unmapped value.

### MMIO Devices

- [ ] **MMIO-01**: Emulator models `FF00` joypad row selection and active-low button reads through a Joypad device instead of plain I/O byte storage.
- [ ] **MMIO-02**: Emulator models `FF04-FF07` timer/divider registers through a Timer device, including DIV reset on write and a hook for TAC/DIV edge effects.
- [ ] **MMIO-03**: Emulator models `FF01/FF02` serial register behavior enough to capture test ROM output, clear transfer-start state, and request serial interrupts deterministically.
- [ ] **MMIO-04**: Emulator models `FF0F` interrupt request and `FFFF` interrupt enable through shared interrupt state used by CPU and devices.
- [ ] **MMIO-05**: Emulator models `FF46` OAM DMA start/progress state through a DMA controller and exposes bus restrictions during active DMA.

### Boot And Machine Timing

- [ ] **BOOT-02**: Emulator exposes explicit `SkipBootRom` and `UseBootRom` startup policies so post-boot defaults are never mixed with active boot ROM execution.
- [ ] **BOOT-03**: Emulator can route boot ROM reads over cartridge reads while mapped and unmap the boot ROM with the one-way `FF50` boot lock behavior.
- [ ] **MACH-01**: Emulator has a top-level Machine or equivalent owner for CPU, cartridge, RAM, and device state, with Bus/Interconnect acting as an address router.
- [ ] **MACH-02**: Emulator advances devices by machine cycles after each CPU instruction using `StepResult.machine_cycles`.
- [ ] **MACH-03**: Timer, serial, DMA, PPU access state, and interrupt hooks can receive M-cycle ticks without requiring CPU instruction code to know device internals.

### Validation

- [ ] **TEST-05**: Cargo tests cover full address-space routing, Echo RAM mirroring, unusable memory behavior, VRAM/OAM access gating, external RAM routing, and side-effect-free peek behavior.
- [ ] **TEST-06**: Cargo tests cover key MMIO side effects for joypad, timer/divider writes, serial transfer completion, interrupt request/enable state, DMA start, and boot ROM unmap.
- [ ] **TEST-07**: Cargo tests cover NoMbc, MBC1, MBC3, and MBC5 mapper behavior with focused ROM/RAM bank fixtures.
- [ ] **TEST-08**: ROM-driven validation harness can run capability-gated fixtures with deterministic pass/fail/timeout reporting and serial-output capture.

## Future Requirements

Deferred to later milestones. Tracked but not in the v1.1 roadmap.

### CPU Completion

- **CPU-07**: Emulator can execute remaining SM83 base opcodes and CB-prefixed opcodes with tested flag/timing behavior.
- **CPU-08**: Emulator can execute interrupt control instructions such as `EI`, `DI`, `RETI`, and HALT edge cases with hardware-compatible timing.

### Timing And Interrupt Precision

- **TIME-04**: Timer obscure behavior is tightened against mooneye/blargg timing ROMs after the M-cycle architecture exists.
- **INT-03**: CPU interrupt service timing is validated against ROM-driven interrupt tests.

### Rendering

- **PPU-04**: Emulator can render DMG background/window/sprite pixels into a frame buffer.
- **PPU-05**: Emulator can pass capability-appropriate PPU mode and OAM bug tests.

### Cartridge Expansion

- **MBC-06**: Emulator supports MBC2 512x4-bit RAM semantics.
- **MBC-07**: Emulator supports MBC3 RTC ticking, latching, persistence, and edge cases.
- **MBC-08**: Emulator supports specialty cartridge families such as MBC6, MBC7, Pocket Camera, Bandai Tama5, HuC3, and HuC1.
- **SAVE-02**: Emulator can persist and reload battery-backed external RAM.

### Audio And Host

- **APU-03**: Emulator can model DMG APU channel state.
- **APU-04**: Emulator can output audio through a host backend.
- **HOST-02**: User can run ROMs in an interactive desktop window.

### Compatibility

- **CGB-02**: Emulator can execute CGB mode, including CGB-specific VRAM/WRAM banking, palettes, DMA, speed switching, and register behavior.

## Out of Scope

Explicitly excluded from v1.1. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Full CGB execution | v1.1 may classify or reject CGB-only ROMs, but implementing CGB mode would expand VRAM/WRAM banking, palettes, DMA, and speed behavior. |
| Full pixel rendering | v1.1 only needs PPU access-state hooks for bus correctness; rendering can follow once timing and memory access rules are stable. |
| Audio output | APU work depends on stable CPU/device timing and is not needed for the architecture hardening milestone. |
| Bundled Nintendo boot ROM images | v1.1 can support a boot ROM path/policy but should not ship copyrighted boot ROM bytes. |
| Full MBC3 RTC behavior | MBC3 banking can land first; RTC ticking/latching/persistence should be a focused later compatibility slice. |
| Save persistence | External RAM semantics should be correct before adding filesystem persistence. |
| Broad ROM-suite pass claims | v1.1 harness must be capability-gated; unsupported hardware failures should not masquerade as emulator regressions. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| CART-06 | Phase 6 | Pending |
| CART-07 | Phase 6 | Pending |
| CART-08 | Phase 6 | Pending |
| CART-09 | Phase 6 | Pending |
| MAP-01 | Phase 6 | Pending |
| MAP-02 | Phase 6 | Pending |
| MAP-03 | Phase 10 | Pending |
| MAP-04 | Phase 10 | Pending |
| BUS-05 | Phase 7 | Pending |
| BUS-06 | Phase 7 | Pending |
| BUS-07 | Phase 7 | Pending |
| BUS-08 | Phase 7 | Pending |
| MMIO-01 | Phase 8 | Pending |
| MMIO-02 | Phase 8 | Pending |
| MMIO-03 | Phase 8 | Pending |
| MMIO-04 | Phase 8 | Pending |
| MMIO-05 | Phase 8 | Pending |
| BOOT-02 | Phase 9 | Pending |
| BOOT-03 | Phase 9 | Pending |
| MACH-01 | Phase 9 | Pending |
| MACH-02 | Phase 9 | Pending |
| MACH-03 | Phase 9 | Pending |
| TEST-05 | Phase 7 | Pending |
| TEST-06 | Phase 8 | Pending |
| TEST-07 | Phase 10 | Pending |
| TEST-08 | Phase 10 | Pending |

**Coverage:**
- v1.1 requirements: 26 total
- Mapped to phases: 26
- Unmapped: 0

---
*Requirements defined: 2026-05-02*
*Last updated: 2026-05-02 after v1.1 roadmap detail expansion*
