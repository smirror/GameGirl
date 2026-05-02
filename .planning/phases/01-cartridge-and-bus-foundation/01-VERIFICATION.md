---
phase: 01-cartridge-and-bus-foundation
status: passed
verified: 2026-05-02
requirements_verified: [CART-01, CART-02, CART-03, CART-04, CART-05, BUS-01, BUS-02, BUS-03, BUS-04]
must_haves_verified: 21
must_haves_total: 21
human_verification: []
gaps: []
---

# Phase 01 Verification: Cartridge and Bus Foundation

## Verdict

Passed. Phase 1 achieves its goal: the user can load ROM bytes into a cartridge path, construct/read ROM-only cartridge data, and exercise a Bus API for the core DMG memory ranges required by the roadmap.

## Automated Checks

- `cargo fmt --all -- --check` passed.
- `cargo test` passed with 23 tests: 17 core unit tests and 6 CLI integration tests.
- `cargo test cartridge` passed with 11 cartridge tests.
- `cargo test bus` passed with 6 Bus tests.
- `cargo clippy -- -D warnings` passed.
- CLI integration tests cover valid ROM loading, case-insensitive `.gbc` handling, missing argument usage, invalid suffix, missing file, and too-short ROM behavior.

## Requirement Verification

| Requirement | Status | Evidence |
|-------------|--------|----------|
| CART-01 | Passed | `src/main.rs` delegates to `game_girl::cartridge::load_rom_file`, which reads bytes through `std::fs::read`; CLI integration tests assert valid ROM output. |
| CART-02 | Passed | CLI integration tests assert non-zero errors for missing arguments, missing files, invalid suffixes, and too-short ROMs; cartridge errors implement `Display`. |
| CART-03 | Passed | `CartridgeHeader` exposes title, cartridge type/code, ROM/RAM size/code, entry point, logo/header bytes, CGB flag, header checksum, and global checksum. |
| CART-04 | Passed | `Cartridge::from_bytes` constructs ROM-only cartridges and `read_rom` covers fixed ROM range reads. |
| CART-05 | Passed | Non-ROM-only type codes are parsed and returned as `UnsupportedCartridgeType(code)`. |
| BUS-01 | Passed | `Bus::read8(addr: u16)` and `Bus::write8(addr: u16, value: u8)` are implemented. |
| BUS-02 | Passed | Bus routes cartridge ROM, WRAM, HRAM, IE, and I/O reads. |
| BUS-03 | Passed | Bus routes writable memory/I/O writes and delegates ROM-range writes to `Cartridge::write_rom`. |
| BUS-04 | Passed | Bus tests cover representative valid, mirrored, unusable, and unmapped ranges. |

## Success Criteria

1. CLI with a `.gb` path reads binary bytes without UTF-8 assumptions: Passed with integration coverage.
2. Clear errors for missing, unreadable, too-short, or invalid ROM inputs: Passed with integration coverage.
3. Cartridge header parsing exposes title, type, ROM size, RAM size, and entry/header data in tests: Passed.
4. Non-ROM-only cartridge types report `UnsupportedCartridgeType`: Passed.
5. Bus tests cover cartridge ROM, WRAM, HRAM, IE, I/O, and representative unusable ranges: Passed.

## Must-Haves

All plan `must_haves.truths` were verified against code and tests:

- Reusable core modules exist in `src/lib.rs`, `src/cartridge.rs`, and `src/bus.rs`.
- CLI logic stays in `src/main.rs` and delegates loading to the core.
- No new dependencies were added.
- ROM loading is byte-oriented and no `read_to_string`/ROM-loading `expect` path remains.
- Header parsing is bounds-checked before slicing.
- ROM-only cartridges construct and read; unsupported type and size codes fail explicitly.
- Nintendo logo, checksum, and CGB flag are metadata only.
- Bus routes Phase 1 address ranges through `read8`/`write8`.
- ROM writes route to the cartridge layer.
- Out-of-scope helpers and ROM execution harness behavior remain deferred.

## Non-Blocking Notes

- Codebase drift gate reported stale codebase-map context for `.gitattributes`, `.gitignore`, and `AGENTS.md`. This is advisory and unrelated to Phase 1 source correctness.
- Security enforcement is enabled and no phase security audit has been run yet. Run `$gsd-secure-phase 1` before advancing if you want the formal security gate artifact.

## Gaps

None.

---
*Verified: 2026-05-02*
