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
- Rust 2021 edition - the CLI entry point lives in `src/main.rs`, with reusable emulator modules exported through `src/lib.rs`.
- Markdown - project documentation in `README.md`, `docs/hot_to_proceed.md`, `docs/gameboy_architecture_summary.md`, and ROM notes.
- Game Boy assembly - sample and upstream test ROM sources under `roms/**/source/` and `roms/hello-world/hello-world.asm`.
- YAML - GitHub Actions and repository automation under `.github/`.
## Runtime
- Native Rust binary built with Cargo.
- No explicit minimum Rust toolchain is pinned in Cargo metadata or repository toolchain configuration.
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
- Rust standard library - `std::env`, `std::path`, and `std::process` in `src/main.rs`, plus `std::fs` for ROM file reads in `src/cartridge.rs`.
- GitHub Actions - CI, labeling, first-interaction greeting, dependency review, auto-assignment, and clippy SARIF upload.
- No crates are listed in `[dependencies]` in `Cargo.toml`.
## Configuration
- No application environment variables are required.
- Runtime configuration is currently only the first CLI argument: a path ending in `.gb` or `.gbc`.
- `Cargo.toml` - Rust package metadata.
- `Cargo.lock` - dependency lockfile.
- `.editorconfig` - editor formatting baseline.
- GitHub workflow YAML files under `.github/workflows/` - CI automation.
## Platform Requirements
- Any platform with a compatible Rust toolchain and Cargo.
- No external database, service, or local daemon is required.
- The package metadata describes "A GameBoy emulator written in Rust"; the current executable is a CLI that validates and loads ROM bytes.
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- Rust files should use snake_case, matching standard Rust conventions.
- Current Rust source files are `src/main.rs`, `src/lib.rs`, `src/cartridge.rs`, and `src/bus.rs`.
- Emulator modules should use focused names such as cpu, timer, and ppu as new subsystems are added.
- Rust functions use snake_case.
- Current executable uses a single `main()` function.
- Rust local variables use snake_case, as seen with `args`, `program`, and `file_path` in `src/main.rs`.
- Compile-time constants use UPPER_SNAKE_CASE, as seen in cartridge header offsets and Bus memory sizes.
- Public project types include `Cartridge`, `CartridgeHeader`, `CartridgeType`, `CartridgeError`, and `Bus`.
- Structs and enums should use PascalCase, matching Rust convention.
## Code Style
- rustfmt is the expected formatter.
- CI runs `cargo fmt --all` in `.github/workflows/rust.yml`.
- Indentation and formatting should follow Rust defaults.
- Clippy is expected.
- CI runs `cargo clippy` in `.github/workflows/rust.yml`.
- A separate clippy SARIF workflow exists at `.github/workflows/rust-clippy.yml`.
## Import Organization
- Current code keeps imports narrow and module-local, such as CLI imports in `src/main.rs` and file/path imports in `src/cartridge.rs`.
- Keep imports simple and explicit.
- None.
## Error Handling
- Current code uses early return for missing arguments and invalid extensions.
- Cartridge file loading returns `Result` values and the CLI formats errors without panicking.
- Continue returning `Result` from parsing/loading helpers so CLI boundaries can format errors without panics.
- Current cartridge loading distinguishes unsupported extension, unreadable file, too-short header, unsupported cartridge type, unsupported ROM size code, and unsupported RAM size code.
## Logging
- `println!` and `eprintln!` only.
- Use `eprintln!` for usage and errors.
- Keep emulator core free of console output; return structured results/errors to the boundary instead.
## Comments
- Explain hardware quirks, timing assumptions, and spec/test-ROM discrepancies.
- Good candidates include DAA flag behavior, timer falling-edge behavior, delayed `ei`, HALT behavior, and PPU access restrictions.
- Avoid comments that merely restate straightforward Rust statements.
- Not used yet.
- Add Rustdoc for public emulator structs as the library API stabilizes.
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
- `src/lib.rs` exports reusable emulator core modules, currently `bus` and `cartridge`.
- Keep `src/main.rs` as a CLI shell around reusable emulator core modules.
- Keep CPU instruction execution independent from cartridge file I/O.
- Route memory-mapped reads/writes through a Bus abstraction.
- Keep rendering/audio backends separate from timing-accurate PPU/APU state.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- Cargo package with both a CLI binary and reusable library modules.
- Source is split across `src/main.rs`, `src/lib.rs`, `src/cartridge.rs`, and `src/bus.rs`.
- Cartridge loading/header parsing and Bus address routing modules are implemented.
- Documentation already describes the intended emulator architecture and implementation roadmap.
- ROM fixtures are present for later validation, but they are not wired into automated tests.
## Layers
- Purpose: Accept command-line input, validate ROM paths, delegate ROM loading, and report usage/errors to the user.
- Contains: argument collection, extension validation, cartridge loading delegation, exit-code handling, and console output.
- Location: `src/main.rs`.
- Depends on: Rust standard library only.
- Used by: direct binary execution through Cargo or compiled executable.
- Purpose: Load and validate the supplied ROM path.
- Contains: byte-oriented file reads, header parsing, ROM-only cartridge construction, and cartridge error types.
- Location: `src/cartridge.rs`.
- Depends on: local filesystem for file loading and Rust standard library parsing helpers.
- Used by: CLI boundary and Bus construction.
- Purpose: Model Game Boy hardware behavior behind reusable core modules.
- Contains: cartridge metadata/loading and Bus address routing today; CPU, timer, PPU, joypad, and APU are future subsystems.
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
- Examples: `Bus` in `src/bus.rs`.
- Pattern: boundary between CPU-visible memory access and devices.
- Purpose: Implement LR35902 registers, instruction decode, execution, flags, and timing.
- Examples: none implemented.
- Pattern: likely `Cpu` struct plus opcode execution helpers.
- Purpose: Hold ROM bytes, parse header metadata, and provide MBC behavior.
- Examples: `Cartridge`, `CartridgeHeader`, `CartridgeType`, and `CartridgeError` in `src/cartridge.rs`.
- Pattern: enum-backed cartridge type handling currently accepts ROM-only cartridges and explicitly rejects unsupported cartridge type codes.
## Entry Points
- Location: `src/main.rs`.
- Triggers: user runs the binary through Cargo or direct executable invocation.
- Responsibilities: validate arguments, load supplied ROM bytes through the cartridge module, print result, and return success/failure exit codes.
## Error Handling
- `eprintln!` for missing-argument usage.
- `eprintln!` for invalid extension and cartridge loading errors.
- Structured cartridge errors for read failures, short ROMs, unsupported type codes, and unsupported size codes.
## Cross-Cutting Concerns
- Console output only.
- No structured logging or log levels.
- Case-insensitive extension check for `.gb` and `.gbc`.
- Binary cartridge header validation is implemented in `src/cartridge.rs`.
- Source-level unit tests and CLI integration tests are present.
- Future emulator behavior should be validated against checked-in blargg and mooneye ROMs.
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, `.github/skills/`, or `.codex/skills/` with a skill index file.
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
