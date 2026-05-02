# Coding Conventions

**Analysis Date:** 2026-05-02

## Naming Patterns

**Files:**
- Rust files should use snake_case, matching standard Rust conventions.
- Current source file is `src/main.rs`.
- Future emulator modules should use focused names such as `cpu.rs`, `bus.rs`, `cartridge.rs`, `timer.rs`, and `ppu.rs`.

**Functions:**
- Rust functions use snake_case.
- Current executable uses a single `main()` function.

**Variables:**
- Rust local variables use snake_case, as seen with `args`, `file_path`, and `contents` in `src/main.rs`.
- Constants are not present yet; use UPPER_SNAKE_CASE for future compile-time constants.

**Types:**
- No project types are defined yet.
- Future structs/enums should use PascalCase, matching Rust convention.

## Code Style

**Formatting:**
- rustfmt is the expected formatter.
- CI runs `cargo fmt --all` in `.github/workflows/rust.yml`.
- Indentation and formatting should follow Rust defaults.

**Linting:**
- Clippy is expected.
- CI runs `cargo clippy` in `.github/workflows/rust.yml`.
- A separate clippy SARIF workflow exists at `.github/workflows/rust-clippy.yml`.

## Import Organization

**Order:**
1. Standard library imports.
2. External crate imports, once dependencies are added.
3. Internal module imports.

**Grouping:**
- Current code groups `std::env` and `std::fs` together at the top of `src/main.rs`.
- Keep imports simple and explicit.

**Path Aliases:**
- None.

## Error Handling

**Patterns:**
- Current code uses early return for missing arguments and invalid extensions.
- Current code panics on file read failure through `expect`.
- As emulator code grows, prefer returning `Result` from parsing/loading helpers so CLI boundaries can format errors without panics.

**Error Types:**
- None defined yet.
- Future cartridge loading should distinguish invalid path, unsupported extension, unreadable file, invalid header, and unsupported MBC.

## Logging

**Framework:**
- `println!` and `eprintln!` only.

**Patterns:**
- Use `eprintln!` for usage and errors.
- Keep emulator core free of console output; return structured results/errors to the boundary instead.

## Comments

**When to Comment:**
- Explain hardware quirks, timing assumptions, and spec/test-ROM discrepancies.
- Good candidates include DAA flag behavior, timer falling-edge behavior, delayed `ei`, HALT behavior, and PPU access restrictions.
- Avoid comments that merely restate straightforward Rust statements.

**Rustdoc:**
- Not used yet.
- Add Rustdoc for public emulator structs, especially if a `lib.rs` is introduced.

**TODO Comments:**
- No project TODO pattern is established.
- Prefer tracking planned work in `.planning/` requirements and phase plans once initialization is complete.

## Function Design

**Size:**
- `main()` is currently short.
- Future instruction and device logic should be split into small helpers because emulator correctness depends on many edge cases.

**Parameters:**
- Prefer typed structs over long parameter lists for hardware state.
- Keep CPU/device state ownership explicit.

**Return Values:**
- Prefer `Result<T, E>` for fallible loading/parsing.
- Avoid hidden panics in emulator core code.

## Module Design

**Exports:**
- No module export pattern exists yet.
- If the project grows past a single binary, introduce `src/lib.rs` for reusable emulator core and keep `src/main.rs` as a CLI shell.

**Boundaries:**
- Keep CPU instruction execution independent from cartridge file I/O.
- Route memory-mapped reads/writes through a Bus abstraction.
- Keep rendering/audio backends separate from timing-accurate PPU/APU state.

---

*Convention analysis: 2026-05-02*
*Update when patterns change*
