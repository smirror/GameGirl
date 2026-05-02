---
phase: 01-cartridge-and-bus-foundation
plan: "02"
subsystem: cartridge
tags: [rust, cartridge-header, rom-only, validation]
requires:
  - phase: 01-01
    provides: Binary ROM loading and cartridge error foundation
provides:
  - Parsed cartridge header metadata
  - ROM-only cartridge construction
  - Unsupported cartridge and size-code errors
  - ROM-only read and write policy hooks
affects: [phase-01, phase-02, phase-03, cartridge, bus]
tech-stack:
  added: []
  patterns:
    - Explicit header offset constants
    - Bounds-checked parsing before header slicing
    - In-memory cartridge fixtures for unit tests
key-files:
  created: []
  modified:
    - src/cartridge.rs
key-decisions:
  - "CartridgeHeader stores logo, checksum, CGB flag, and raw header bytes as metadata only."
  - "Cartridge::from_bytes accepts only ROM-only cartridge type 0x00 in Phase 1."
  - "ROM-only writes are routed to a cartridge policy hook and intentionally do not mutate ROM bytes."
patterns-established:
  - "Cartridge constructors validate size/type metadata before exposing a usable cartridge."
  - "ROM reads return 0xFF for missing fixture bytes instead of panicking."
requirements-completed: [CART-02, CART-03, CART-04, CART-05]
duration: 4 min
completed: 2026-05-02
---

# Phase 01 Plan 02: Cartridge Header and ROM-Only Summary

**Parsed Game Boy cartridge headers with ROM-only construction, explicit unsupported-type errors, and fixed ROM read hooks**

## Performance

- **Duration:** 4 min
- **Started:** 2026-05-02T02:57:15Z
- **Completed:** 2026-05-02T03:01:15Z
- **Tasks:** 3
- **Files modified:** 1

## Accomplishments

- Added `Cartridge`, `CartridgeHeader`, and `CartridgeType` with explicit constants for Game Boy header offsets.
- Parsed title, cartridge type, ROM/RAM size codes, entry point, logo bytes, CGB flag, header checksum, global checksum, and raw header bytes.
- Added `Cartridge::from_bytes` and `load_cartridge_file`, accepting ROM-only cartridges and rejecting unsupported MBC/size codes with clear errors.
- Added ROM-only `read_rom` and `write_rom` hooks for Bus integration.
- Expanded cartridge tests from 3 to 11 focused unit tests.

## Task Commits

Each task was committed atomically:

1. **Task 1: Define cartridge metadata and parser types** - `2810e8d` (feat)
2. **Task 2: Validate ROM-only type and size codes** - `cf7a27e` (feat)
3. **Task 3: Add ROM-only read behavior and parser tests** - `48a4549` (feat)

## Files Created/Modified

- `src/cartridge.rs` - Adds the parsed cartridge model, semantic validation, ROM-only reads/writes, and parser tests.

## Decisions Made

- Kept Nintendo logo and checksum values as exposed metadata, not hard validation gates.
- Kept CGB flag data as metadata only.
- Left broad MBC support deferred while still reporting unsupported type codes clearly.

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
- `cargo test` passed.
- `cargo test cartridge` passed with 11 cartridge tests.
- Grep for `UnsupportedCartridgeType`, `UnsupportedRomSize`, and `UnsupportedRamSize` found the expected validation paths in `src/cartridge.rs`.
- Cargo run against the hello-world ROM fixture loaded the ROM and printed `Loaded ROM: 32768 bytes`.

## Next Phase Readiness

Plan 01-03 can now construct a `Bus` around a ROM-only `Cartridge` and delegate ROM-range reads and writes to cartridge policy instead of duplicating cartridge behavior.

---
*Phase: 01-cartridge-and-bus-foundation*
*Completed: 2026-05-02*
