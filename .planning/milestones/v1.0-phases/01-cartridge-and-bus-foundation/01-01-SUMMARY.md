---
phase: 01-cartridge-and-bus-foundation
plan: "01"
subsystem: cartridge
tags: [rust, cartridge, cli, binary-loading]
requires: []
provides:
  - Reusable Rust library module boundary
  - Byte-oriented ROM file loading
  - Cartridge loading error type with Display output
  - CLI delegation to the cartridge loader
affects: [phase-01, phase-02, cartridge, cli]
tech-stack:
  added: []
  patterns:
    - Dependency-free Rust core modules
    - Thin CLI boundary delegating to library APIs
key-files:
  created:
    - src/lib.rs
    - src/cartridge.rs
  modified:
    - src/main.rs
    - src/cartridge.rs
key-decisions:
  - "Introduced src/lib.rs as the reusable emulator core entry point."
  - "Kept src/main.rs focused on argument handling, extension checks, user-facing output, and exit codes."
  - "Validated ROM bytes with a 0x150 minimum length before future header parsing."
patterns-established:
  - "Core loading APIs return Result values and leave presentation to the CLI boundary."
  - "Cartridge tests use in-memory byte fixtures rather than checked-in ROM execution."
requirements-completed: [CART-01, CART-02]
duration: 3 min
completed: 2026-05-02
---

# Phase 01 Plan 01: Binary Cartridge Loading Summary

**Dependency-free binary ROM loading with a reusable cartridge module and non-panicking CLI error reporting**

## Performance

- **Duration:** 3 min
- **Started:** 2026-05-02T02:53:35Z
- **Completed:** 2026-05-02T02:56:06Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments

- Added `src/lib.rs` and exported `pub mod cartridge;` as the first reusable core boundary.
- Added `src/cartridge.rs` with byte-oriented ROM loading through `std::fs::read`, minimum header length validation, and a custom `CartridgeError` with `Display`.
- Refactored `src/main.rs` so the CLI delegates to `game_girl::cartridge::load_rom_file`, reports clear errors, and exits non-zero for normal user failures.
- Added fast in-memory unit tests for too-short ROM rejection, valid header-length acceptance, and human-readable error display.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add the core module boundary and cartridge loading error type** - `a557de5` (feat)
2. **Task 2: Refactor the CLI to delegate binary ROM loading** - `990134a` (feat)
3. **Task 3: Add focused binary loading tests** - `828b1d6` (test)

## Files Created/Modified

- `src/lib.rs` - Exposes the cartridge module for the reusable emulator core.
- `src/cartridge.rs` - Defines the ROM loading guard, cartridge loading errors, and unit tests.
- `src/main.rs` - Delegates ROM loading to the library and reports user errors without panics.

## Decisions Made

- Kept file loading as `Vec<u8>` for this plan so full header parsing can land cleanly in Plan 01-02.
- Used `std::process::ExitCode` in the CLI so missing paths, invalid suffixes, unreadable files, and too-short ROMs are observable failures.
- Kept tests in `src/cartridge.rs` and avoided checked-in ROM suite execution.

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
- `cargo test cartridge` passed.
- `cargo run -- missing.gb` printed a clear error and exited non-zero.
- `rg "read_to_string|expect\\(" src` found no matches.

## Next Phase Readiness

Plan 01-02 can build directly on `src/cartridge.rs` by replacing raw byte validation with a parsed `Cartridge` and `CartridgeHeader` while keeping the CLI boundary stable.

---
*Phase: 01-cartridge-and-bus-foundation*
*Completed: 2026-05-02*
