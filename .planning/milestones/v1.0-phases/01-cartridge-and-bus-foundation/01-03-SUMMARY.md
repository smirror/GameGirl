---
phase: 01-cartridge-and-bus-foundation
plan: "03"
subsystem: bus
tags: [rust, bus, memory-map, dmg]
requires:
  - phase: 01-02
    provides: ROM-only cartridge construction and read/write policy hooks
provides:
  - Bus module exported from the reusable core
  - 8-bit read/write API over the 16-bit address space
  - Phase 1 routing for cartridge ROM, WRAM, Echo RAM, OAM, I/O, HRAM, and IE
  - Representative Bus unit tests
affects: [phase-01, phase-02, phase-03, bus, cpu]
tech-stack:
  added: []
  patterns:
    - Fixed-size arrays for simple memory regions
    - Bus delegates cartridge policy to Cartridge
    - In-memory ROM-only fixture for Bus tests
key-files:
  created:
    - src/bus.rs
  modified:
    - src/lib.rs
    - src/bus.rs
key-decisions:
  - "Bus owns the Cartridge and delegates ROM reads/writes to the cartridge layer."
  - "Unusable memory range FEA0-FEFF reads as 0xFF and ignores writes."
  - "read16/write16, CPU fetch helpers, timer side effects, PPU access restrictions, and interrupt semantics remain deferred."
patterns-established:
  - "Future CPU work should access memory through Bus::read8 and Bus::write8."
  - "Phase 1 memory ranges are tested with exact representative addresses."
requirements-completed: [CART-04, BUS-01, BUS-02, BUS-03, BUS-04]
duration: 4 min
completed: 2026-05-02
---

# Phase 01 Plan 03: Bus Address Routing Summary

**DMG Bus skeleton with cartridge delegation, writable core memory ranges, unusable range behavior, and focused address tests**

## Performance

- **Duration:** 4 min
- **Started:** 2026-05-02T03:01:15Z
- **Completed:** 2026-05-02T03:05:23Z
- **Tasks:** 3
- **Files modified:** 2

## Accomplishments

- Added `src/bus.rs` and exported it through `src/lib.rs`.
- Implemented `Bus::new`, `read8`, and `write8`.
- Routed cartridge ROM, WRAM, Echo RAM, OAM placeholder storage, unusable range, I/O register storage, HRAM, and IE.
- Added 6 Bus tests covering exact representative addresses and ROM write delegation behavior.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add Bus struct and module export** - `9367d89` (feat)
2. **Task 2: Route Phase 1 memory ranges** - `d3c789f` (feat)
3. **Task 3: Add representative Bus routing tests** - `54ec3f0` (test)

## Files Created/Modified

- `src/bus.rs` - Implements Bus ownership, address routing, and Bus unit tests.
- `src/lib.rs` - Exports the Bus module from the reusable core.

## Decisions Made

- Backed OAM with simple placeholder storage so later PPU access rules can attach cleanly.
- Returned `0xFF` for out-of-scope or unusable reads and ignored unsupported writes in Phase 1.
- Kept Bus byte-only for now; no word helpers or CPU fetch helpers were added.

## Deviations from Plan

None - plan executed exactly as written.

---

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope changes.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Verification

- `cargo fmt --all` passed.
- `cargo test` passed with 17 tests.
- `cargo test bus` passed with 6 Bus tests.
- Grep confirmed no `read16`, `write16`, or `fetch` helpers exist in `src/bus.rs`.
- Cargo run against the hello-world ROM fixture loaded the ROM and printed `Loaded ROM: 32768 bytes`.

## Next Phase Readiness

Phase 2 can build CPU state and instruction fetch around `Bus::read8` and `Bus::write8`, with cartridge ROM and core memory ranges already routed through a central boundary.

---
*Phase: 01-cartridge-and-bus-foundation*
*Completed: 2026-05-02*
