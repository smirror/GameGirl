<!-- GSD:project-start source:PROJECT.md -->
## Project

**GameGirl**

GameGirl is a Rust Game Boy emulator project. The repository currently contains a minimal CLI scaffold, implementation notes, Game Boy hardware research, and a substantial local ROM corpus for future validation.

The immediate product direction is a DMG-first emulator core that can load real `.gb` ROM bytes, model the CPU/bus/devices accurately enough to run test ROMs, and grow toward rendering, input, cartridges, and audio in deliberate phases.

**Core Value:** GameGirl must execute DMG Game Boy ROMs accurately enough that behavior is driven by hardware rules and verified by known test ROMs, not by ad hoc demo success.

### Constraints

- **Tech stack**: Rust 2021 with Cargo — match the existing crate and avoid adding dependencies unless they remove real implementation risk.
- **Compatibility**: DMG-first — keep Color Game Boy behavior out of the initial critical path.
- **Correctness**: Use test ROMs as a decision aid when documentation and intuition disagree.
- **Architecture**: Keep CPU, bus, cartridge, timer, PPU, joypad, and eventual APU responsibilities separated so hardware timing rules can be tested in isolation.
- **Input data**: ROMs are untrusted binary files — cartridge parsing must be bounds-checked and avoid unsafe Rust by default.
- **Validation**: Prefer small unit tests for pure CPU/device behavior and ROM-driven integration tests for compatibility.
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- Rust 2021 edition - all executable application code currently lives in `src/main.rs`.
- Markdown - project documentation in `README.md`, `docs/hot_to_proceed.md`, `docs/gameboy_architecture_summary.md`, and ROM notes.
- Game Boy assembly - sample and upstream test ROM sources under `roms/**/source/` and `roms/hello-world/hello-world.asm`.
- YAML - GitHub Actions and repository automation under `.github/`.
## Runtime
- Native Rust binary built with Cargo.
- No explicit minimum Rust toolchain is pinned in `Cargo.toml`, `rust-toolchain.toml`, or `.github/`.
- CI installs the stable Rust toolchain in `.github/workflows/rust-clippy.yml`.
- Cargo - package metadata in `Cargo.toml`.
- Lockfile: `Cargo.lock` is present.
## Frameworks
- None. The current crate uses only Rust standard library APIs.
- Cargo test harness - invoked by `.github/workflows/rust.yml`.
- External ROM fixtures - `roms/blargg-gb-tests/`, `roms/mooneye/`, and `roms/hello-world/` provide future emulator validation assets, but no automated harness consumes them yet.
- Cargo - build, format, lint, and test commands.
- rustfmt - run in `.github/workflows/rust.yml` through `cargo fmt --all`.
- Clippy - run in `.github/workflows/rust.yml` and SARIF-producing `.github/workflows/rust-clippy.yml`.
## Key Dependencies
- Rust standard library - `std::env` for CLI argument collection and `std::fs` for file reading in `src/main.rs`.
- GitHub Actions - CI, labeling, first-interaction greeting, dependency review, auto-assignment, and clippy SARIF upload.
- No crates are listed in `[dependencies]` in `Cargo.toml`.
## Configuration
- No application environment variables are required.
- Runtime configuration is currently only the first CLI argument: a path ending in `.gb` or `.gbc`.
- `Cargo.toml` - Rust package metadata.
- `Cargo.lock` - dependency lockfile.
- `.editorconfig` - editor formatting baseline.
- `.github/workflows/*.yml` - CI automation.
## Platform Requirements
- Any platform with a compatible Rust toolchain and Cargo.
- No external database, service, or local daemon is required.
- Not defined yet. The package metadata describes "A GameBoy emulator written in Rust", but the current executable is only a prototype CLI.
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- Rust files should use snake_case, matching standard Rust conventions.
- Current source file is `src/main.rs`.
- Future emulator modules should use focused names such as `cpu.rs`, `bus.rs`, `cartridge.rs`, `timer.rs`, and `ppu.rs`.
- Rust functions use snake_case.
- Current executable uses a single `main()` function.
- Rust local variables use snake_case, as seen with `args`, `file_path`, and `contents` in `src/main.rs`.
- Constants are not present yet; use UPPER_SNAKE_CASE for future compile-time constants.
- No project types are defined yet.
- Future structs/enums should use PascalCase, matching Rust convention.
## Code Style
- rustfmt is the expected formatter.
- CI runs `cargo fmt --all` in `.github/workflows/rust.yml`.
- Indentation and formatting should follow Rust defaults.
- Clippy is expected.
- CI runs `cargo clippy` in `.github/workflows/rust.yml`.
- A separate clippy SARIF workflow exists at `.github/workflows/rust-clippy.yml`.
## Import Organization
- Current code groups `std::env` and `std::fs` together at the top of `src/main.rs`.
- Keep imports simple and explicit.
- None.
## Error Handling
- Current code uses early return for missing arguments and invalid extensions.
- Current code panics on file read failure through `expect`.
- As emulator code grows, prefer returning `Result` from parsing/loading helpers so CLI boundaries can format errors without panics.
- None defined yet.
- Future cartridge loading should distinguish invalid path, unsupported extension, unreadable file, invalid header, and unsupported MBC.
## Logging
- `println!` and `eprintln!` only.
- Use `eprintln!` for usage and errors.
- Keep emulator core free of console output; return structured results/errors to the boundary instead.
## Comments
- Explain hardware quirks, timing assumptions, and spec/test-ROM discrepancies.
- Good candidates include DAA flag behavior, timer falling-edge behavior, delayed `ei`, HALT behavior, and PPU access restrictions.
- Avoid comments that merely restate straightforward Rust statements.
- Not used yet.
- Add Rustdoc for public emulator structs, especially if a `lib.rs` is introduced.
- No project TODO pattern is established.
- Prefer tracking planned work in `.planning/` requirements and phase plans once initialization is complete.
## Function Design
- `main()` is currently short.
- Future instruction and device logic should be split into small helpers because emulator correctness depends on many edge cases.
- Prefer typed structs over long parameter lists for hardware state.
- Keep CPU/device state ownership explicit.
- Prefer `Result<T, E>` for fallible loading/parsing.
- Avoid hidden panics in emulator core code.
## Module Design
- No module export pattern exists yet.
- If the project grows past a single binary, introduce `src/lib.rs` for reusable emulator core and keep `src/main.rs` as a CLI shell.
- Keep CPU instruction execution independent from cartridge file I/O.
- Route memory-mapped reads/writes through a Bus abstraction.
- Keep rendering/audio backends separate from timing-accurate PPU/APU state.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- Single binary crate.
- Single source file: `src/main.rs`.
- No emulator domain modules have been implemented yet.
- Documentation already describes the intended emulator architecture and implementation roadmap.
- ROM fixtures are present for later validation, but they are not wired into automated tests.
## Layers
- Purpose: Accept command-line input and report usage/errors to the user.
- Contains: argument collection, extension validation, console output.
- Location: `src/main.rs`.
- Depends on: Rust standard library only.
- Used by: direct binary execution through Cargo or compiled executable.
- Purpose: Load the supplied ROM path.
- Contains: a call to `fs::read_to_string`.
- Location: `src/main.rs`.
- Depends on: local filesystem.
- Used by: CLI boundary after extension validation.
- Purpose: Eventually model Game Boy hardware behavior.
- Contains: not implemented yet.
- Intended components from `docs/hot_to_proceed.md` and `docs/gameboy_architecture_summary.md`:
## Data Flow
- Current application state is local to `main`.
- Planned emulator state should be explicit structs for CPU registers, bus, memory, cartridge, timer, PPU, joypad, and later APU.
- No persistence is implemented yet.
## Key Abstractions
- Purpose: Minimal command runner.
- Example: `src/main.rs`.
- Pattern: single `main()` function.
- Purpose: Centralize 16-bit address / 8-bit data reads and writes across CPU-visible devices.
- Examples: none implemented.
- Pattern: should become the boundary between CPU and devices.
- Purpose: Implement LR35902 registers, instruction decode, execution, flags, and timing.
- Examples: none implemented.
- Pattern: likely `Cpu` struct plus opcode execution helpers.
- Purpose: Hold ROM bytes, parse header metadata, and provide MBC behavior.
- Examples: none implemented.
- Pattern: trait or enum-backed mapper implementations may fit future MBC0/MBC1/MBC3/MBC5 support.
## Entry Points
- Location: `src/main.rs`.
- Triggers: user runs the binary through Cargo or direct executable invocation.
- Responsibilities: validate arguments, read supplied file, print result.
## Error Handling
- `eprintln!` for missing-argument usage.
- `println!` for invalid extension.
- `expect("Something went wrong reading the file")` for read failures.
## Cross-Cutting Concerns
- Console output only.
- No structured logging or log levels.
- Simple filename suffix check for `.gb` and `.gbc`.
- No binary header validation yet.
- No source-level tests currently exist.
- Future emulator behavior should be validated against checked-in blargg and mooneye ROMs.
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, `.github/skills/`, or `.codex/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
