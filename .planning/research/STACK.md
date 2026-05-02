# Stack Research

**Domain:** Rust DMG Game Boy emulator
**Researched:** 2026-05-02
**Confidence:** HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Rust stable | Current official docs show 1.95.0 | Emulator core and CLI | Existing crate is Rust 2021; Rust gives explicit ownership and bounds-checked data handling for untrusted ROM bytes |
| Cargo | Bundled with Rust toolchain | Build, test, lint entry point | Existing repo already uses `Cargo.toml` and `Cargo.lock`; keeps the project simple |
| Rust standard library | Bundled | CLI, filesystem, byte buffers | Enough for Phase 1 cartridge loading and core data structures; avoid dependencies until a real need appears |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| None for Phase 1 | n/a | Keep the emulator core dependency-free | Use for cartridge loading, bus, CPU basics, timer, and initial tests |
| clap | Defer | Rich CLI parsing | Consider only when CLI commands/options grow past one ROM path |
| pixels/winit | Defer | Window/display output | Consider after CPU/bus/timer/PPU mode foundations can run meaningful frames |
| cpal | Defer | Audio output | Consider after APU state is implemented and testable |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| rustfmt | Formatting | Existing CI already runs `cargo fmt --all` |
| Clippy | Linting | Existing CI runs `cargo clippy`; keep new code clippy-clean |
| Cargo tests | Unit and integration tests | Add unit tests early; add ROM integration harness once execution loop exists |
| blargg and mooneye ROMs | Hardware behavior validation | Already checked into `roms/`; wire them into tests in later phases |

## Installation

```bash
# Existing stack is already dependency-free.
cargo build
cargo test
cargo clippy
```

No new crates are recommended for the first milestone.

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| Standard library CLI parsing | clap | Use clap when there are multiple commands, flags, config files, or richer error messages |
| `fs::read` for ROM bytes | `fs::read_to_string` | Never for ROMs; ROMs are binary, not UTF-8 text |
| Dependency-free core | Framework-heavy emulator shell | Add host UI/audio dependencies only after core correctness has something to drive |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Text file reads for ROMs | `.gb` and `.gbc` are binary files; text reads can fail or misrepresent data | `std::fs::read`, which returns `Vec<u8>` |
| CGB-first architecture | Color-specific behavior multiplies memory, timing, and rendering variance | DMG-first modules with explicit extension points |
| UI/audio dependencies in Phase 1 | They distract from core correctness and make tests harder | Core crate modules plus Cargo tests |
| Unsafe indexing as a shortcut | ROM parsing is untrusted binary input | Bounds-checked slices and explicit parse errors |

## Stack Patterns by Variant

**If building the first milestone:**
- Use only Rust std, Cargo tests, and checked-in ROM fixtures.
- Because the main risk is correctness, not host integration.

**If adding a desktop shell later:**
- Keep the emulator core reusable and add host display/audio/input in separate modules.
- Because UI refresh and audio callbacks should not own hardware rules.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| Rust stable | Cargo, rustfmt, Clippy | No pinned version currently; add `rust-toolchain.toml` if reproducibility becomes important |
| Rust std `fs::read` | Rust 1.26.0+ | Official docs show it returns `Result<Vec<u8>>`, which matches ROM loading needs |

## Sources

- https://doc.rust-lang.org/std/fs/fn.read.html - verified byte-oriented file loading API
- https://gbdev.io/pandocs/Memory_Map.html - verified cartridge memory ranges and header location
- https://github.com/gbdev/pandocs - verified Pan Docs as a maintained public Game Boy technical reference
- `.planning/codebase/STACK.md` - verified current repo stack

---
*Stack research for: Rust DMG Game Boy emulator*
*Researched: 2026-05-02*
