# Phase 1 Pattern Map: Cartridge and Bus Foundation

**Phase:** 01-cartridge-and-bus-foundation
**Mapped:** 2026-05-02

## Existing Patterns To Preserve

- Dependency-free Rust crate: keep implementation in Rust standard library and avoid adding dependencies.
- Thin CLI boundary: keep user argument handling and display in `src/main.rs`.
- Cargo-native tests: add ordinary Rust unit tests and verify with `cargo test`.
- Bus-centered architecture: follow project docs that route CPU-visible memory through a central Bus.

## File Pattern Mapping

| New or Modified File | Closest Existing Pattern | Planning Guidance |
|----------------------|--------------------------|-------------------|
| `src/main.rs` | Current CLI file | Keep it small. It should parse arguments, reject unsupported path suffixes, call core cartridge loading, print a concise success message or error, and avoid hardware logic. |
| `src/lib.rs` | Standard Rust library module root | Add module exports only. Do not place implementation logic here. |
| `src/cartridge.rs` | New core module | Use explicit constants for header offsets, custom error enums with `Display`, and inline unit tests with in-memory ROM byte fixtures. |
| `src/bus.rs` | New core module | Use a `Bus` struct with fixed-size arrays for simple memory regions and tests for each required address range. |

## Anti-Patterns To Avoid

- Do not keep using `read_to_string` for ROM data.
- Do not add CPU, timer, interrupt, PPU, APU, or UI behavior in Phase 1.
- Do not silently accept unknown cartridge type or size codes.
- Do not make the Bus own ROM write policy; route ROM writes to the cartridge layer.
- Do not build broad checked-in ROM execution tests before the execution harness phase.

## Execution Dependencies

Plan ordering should be sequential:

- `01-01` creates the library/cartridge loading foundation and CLI handoff.
- `01-02` builds on cartridge loading to parse headers and expose ROM-only reads.
- `01-03` builds on the cartridge representation to add Bus routing and memory range tests.

