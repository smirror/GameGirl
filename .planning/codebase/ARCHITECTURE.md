# Architecture

**Analysis Date:** 2026-05-02

## Pattern Overview

**Overall:** Early-stage Rust CLI prototype for a Game Boy emulator.

**Key Characteristics:**
- Single binary crate.
- Single source file: `src/main.rs`.
- No emulator domain modules have been implemented yet.
- Documentation already describes the intended emulator architecture and implementation roadmap.
- ROM fixtures are present for later validation, but they are not wired into automated tests.

## Layers

**CLI Boundary:**
- Purpose: Accept command-line input and report usage/errors to the user.
- Contains: argument collection, extension validation, console output.
- Location: `src/main.rs`.
- Depends on: Rust standard library only.
- Used by: direct binary execution through Cargo or compiled executable.

**Prototype File Loading:**
- Purpose: Load the supplied ROM path.
- Contains: a call to `fs::read_to_string`.
- Location: `src/main.rs`.
- Depends on: local filesystem.
- Used by: CLI boundary after extension validation.

**Planned Emulator Core:**
- Purpose: Eventually model Game Boy hardware behavior.
- Contains: not implemented yet.
- Intended components from `docs/hot_to_proceed.md` and `docs/gameboy_architecture_summary.md`:
  - Bus and memory map.
  - CPU registers, fetch/decode/execute loop, and LR35902 instructions.
  - Timer and interrupt controller.
  - PPU mode timing, VRAM/OAM access restrictions, background/window/sprite rendering.
  - Joypad input.
  - Cartridge and MBC support.
  - APU audio channels.

## Data Flow

**Current CLI Execution:**

1. User runs the binary with a ROM path argument.
2. `src/main.rs` collects all command-line arguments with `env::args()`.
3. If no ROM path is supplied, the program prints usage to stderr and returns.
4. If the path does not end with `.gb` or `.gbc`, the program prints an error and returns.
5. The program reads the path as UTF-8 text using `fs::read_to_string`.
6. The loaded text is printed to stdout.

**Planned Emulator Execution:**

1. Load cartridge bytes from a `.gb` or `.gbc` file.
2. Parse cartridge header and determine ROM/MBC behavior.
3. Initialize DMG CPU registers and memory-mapped devices.
4. Step CPU instructions through a bus that coordinates memory, timer, interrupts, PPU, joypad, and cartridge access.
5. Use test ROMs to verify instruction correctness and timing.

**State Management:**
- Current application state is local to `main`.
- Planned emulator state should be explicit structs for CPU registers, bus, memory, cartridge, timer, PPU, joypad, and later APU.
- No persistence is implemented yet.

## Key Abstractions

**CLI Program:**
- Purpose: Minimal command runner.
- Example: `src/main.rs`.
- Pattern: single `main()` function.

**Bus (planned):**
- Purpose: Centralize 16-bit address / 8-bit data reads and writes across CPU-visible devices.
- Examples: none implemented.
- Pattern: should become the boundary between CPU and devices.

**CPU (planned):**
- Purpose: Implement LR35902 registers, instruction decode, execution, flags, and timing.
- Examples: none implemented.
- Pattern: likely `Cpu` struct plus opcode execution helpers.

**Cartridge (planned):**
- Purpose: Hold ROM bytes, parse header metadata, and provide MBC behavior.
- Examples: none implemented.
- Pattern: trait or enum-backed mapper implementations may fit future MBC0/MBC1/MBC3/MBC5 support.

## Entry Points

**CLI Entry:**
- Location: `src/main.rs`.
- Triggers: user runs the binary through Cargo or direct executable invocation.
- Responsibilities: validate arguments, read supplied file, print result.

## Error Handling

**Strategy:** Early returns for expected argument/extension failures, panic for read failures.

**Patterns:**
- `eprintln!` for missing-argument usage.
- `println!` for invalid extension.
- `expect("Something went wrong reading the file")` for read failures.

## Cross-Cutting Concerns

**Logging:**
- Console output only.
- No structured logging or log levels.

**Validation:**
- Simple filename suffix check for `.gb` and `.gbc`.
- No binary header validation yet.

**Testing:**
- No source-level tests currently exist.
- Future emulator behavior should be validated against checked-in blargg and mooneye ROMs.

---

*Architecture analysis: 2026-05-02*
*Update when major patterns change*
