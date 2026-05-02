# Phase 1: Cartridge and Bus Foundation - Context

**Gathered:** 2026-05-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 1 replaces the current placeholder ROM text-loading CLI with binary cartridge loading, basic cartridge header parsing, ROM-only cartridge reads, and a tested DMG Bus skeleton for core memory ranges.

This phase does **not** execute CPU instructions, run ROM test suites, implement timers/interrupts, render pixels, implement broad MBC support, add CGB behavior, or build a UI/audio host. Those are later phases.

</domain>

<decisions>
## Implementation Decisions

### Module Boundaries
- **D-01:** Create a reusable emulator core now instead of continuing the single-file prototype. Add `src/lib.rs` and focused modules such as `src/cartridge.rs` and `src/bus.rs`.
- **D-02:** Keep `src/main.rs` as a thin CLI boundary: parse the ROM path, call core loading APIs, print clear human-facing errors, and avoid embedding emulator hardware logic in the CLI.
- **D-03:** Do not add new dependencies in Phase 1. Use Rust standard library APIs and custom `Result`/error types that implement `Display`.

### Cartridge Loading and Validation
- **D-04:** Replace `fs::read_to_string` with byte-oriented ROM loading via `std::fs::read`.
- **D-05:** Treat ROM input as untrusted binary data. Hard-fail missing paths, unreadable files, files shorter than the cartridge header range, unsupported/unknown cartridge type for the Phase 1 implementation, and unsupported/invalid size codes.
- **D-06:** Parse cartridge title, cartridge type, ROM size, RAM size, and relevant header/entry fields needed by downstream phases.
- **D-07:** Support ROM-only cartridge behavior first. Cartridge type `0x00` should be constructible and readable through the ROM address ranges. Other MBC types should return a clear unsupported-cartridge error for now.
- **D-08:** Do not make full Nintendo logo or header checksum validation a hard gate in Phase 1 unless the planner finds it trivial and low-risk. It is acceptable to parse or expose these values for later validation.
- **D-09:** CGB-related header data is metadata only in Phase 1. It must not imply CGB execution support.

### Bus Scope and Memory Behavior
- **D-10:** Introduce a `Bus` API centered on `read8(addr: u16)` and `write8(addr: u16, value: u8)`.
- **D-11:** Route CPU-visible memory through the Bus from the start. Future CPU code should not bypass Bus for cartridge, RAM, I/O, or interrupt-enable access.
- **D-12:** Phase 1 Bus coverage should include cartridge ROM ranges `0000-7FFF`, WRAM `C000-DFFF`, Echo RAM `E000-FDFF` as WRAM mirror behavior, OAM `FE00-FE9F` as storage or explicitly prepared storage, unusable `FEA0-FEFF` as read `0xFF` and ignored writes, I/O `FF00-FF7F` as basic register storage/stubs, HRAM `FF80-FFFE`, and IE `FFFF`.
- **D-13:** Writes to cartridge ROM address ranges should route to the cartridge layer. For ROM-only cartridges this can be a no-op or clear unsupported operation, but the Bus should not silently own that policy.
- **D-14:** Defer `read16`/`write16`, CPU fetch helpers, timer side effects, PPU access restrictions, and interrupt semantics unless they are needed as small helpers for tests. Their full behavior belongs to later phases.

### Testing Boundary
- **D-15:** Add tests during Phase 1. Minimum coverage should include cartridge byte loading/parsing, unsupported and invalid input errors, ROM-only reads, Bus read/write behavior for representative ranges, Echo RAM mirror behavior, unusable range behavior, HRAM, I/O storage, and IE.
- **D-16:** Prefer in-memory ROM byte fixtures for unit tests. Use checked-in ROM files only for lightweight smoke tests if they help, not as a full ROM execution harness.
- **D-17:** Defer blargg/mooneye execution, serial pass/fail detection, and deterministic ROM timeouts to Phase 4.

### the agent's Discretion
- Exact Rust module names and file splits inside the locked boundary.
- Exact custom error enum names and wording, provided CLI errors are clear and tests cover important cases.
- Whether OAM is backed by storage in Phase 1 or represented as a prepared placeholder, provided later PPU access rules can attach cleanly.
- Whether Nintendo logo/header checksum values are parsed now or left for a later validation pass.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope
- `.planning/ROADMAP.md` — Phase 1 goal, requirements, success criteria, and three planned work slices.
- `.planning/REQUIREMENTS.md` — `CART-01` through `CART-04` and `BUS-01` through `BUS-04`.
- `.planning/PROJECT.md` — DMG-first, Bus-centered, correctness-first project constraints.
- `.planning/research/SUMMARY.md` — Research rationale for binary loading, Bus first, and dependency-free Rust core.

### Codebase Context
- `.planning/codebase/ARCHITECTURE.md` — Current single-file CLI and planned emulator core boundaries.
- `.planning/codebase/CONCERNS.md` — Current text-ROM-loading bug, extension-only validation, and Bus/module gaps.
- `.planning/codebase/TESTING.md` — Cargo test conventions and future ROM fixture strategy.
- `src/main.rs` — Current CLI behavior to replace/refactor.

### Project Notes
- `docs/hot_to_proceed.md` — Local implementation direction: Bus first, CPU/device separation, test ROM validation philosophy.
- `docs/gameboy_architecture_summary.md` — DMG memory map, cartridge header location, and subsystem constraints relevant to Bus planning.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/main.rs`: Existing CLI argument and suffix-check flow can be kept as the starting boundary, but the file read must become binary and error handling should stop panicking for normal user failures.
- `roms/hello-world/hello-world.gb`: Useful as an optional smoke fixture for byte loading only.
- `roms/blargg-gb-tests/` and `roms/mooneye/`: Important future validation assets, but Phase 1 should not build full execution around them.

### Established Patterns
- The crate is dependency-free Rust 2021. Keep Phase 1 within `std` unless planning finds a truly compelling reason.
- CI already expects `cargo fmt`, `cargo clippy`, and `cargo test`; Phase 1 should add tests that fit normal Cargo patterns.
- Project docs already prefer Bus-centered device separation, so new code should make that boundary explicit.

### Integration Points
- `src/main.rs` should call into core APIs rather than own cartridge/Bus behavior.
- New core modules should be exposed through `src/lib.rs` so unit/integration tests and future hosts can use them.
- Bus should connect first to cartridge and simple memory/register storage; CPU, Timer, PPU, Joypad, and APU attach in later phases.

</code_context>

<specifics>
## Specific Ideas

- Use a pragmatic validation ladder: binary read and header length first, header metadata parsing second, ROM-only support third.
- Keep the first cartridge implementation honest by returning explicit unsupported errors for MBC types not yet implemented.
- Treat `FEA0-FEFF` conservatively as unusable: reads return `0xFF`, writes ignored, unless later research/tests require nuance.
- The planning agent should keep Phase 1 boring in the best way: small APIs, clear tests, no speculative UI/audio/rendering work.

</specifics>

<deferred>
## Deferred Ideas

- CPU fetch/decode/execute and cycle reporting — Phase 2.
- Timer and interrupt semantics — Phase 3.
- ROM execution harness and pass/fail detection — Phase 4.
- PPU mode access restrictions and VBlank/LCD interrupt hooks — Phase 5.
- Background/window/sprite rendering, Joypad, broader MBC support, save RAM, APU, desktop UI, and CGB behavior — v2 requirements or later phases.

</deferred>

---

*Phase: 01-cartridge-and-bus-foundation*
*Context gathered: 2026-05-02*
