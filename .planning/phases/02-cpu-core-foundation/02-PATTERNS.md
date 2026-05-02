# Phase 2: CPU Core Foundation - Pattern Map

**Mapped:** 2026-05-02
**Status:** Ready for planning

## Files Expected

| File | Role | Closest Existing Analog | Notes |
|------|------|-------------------------|-------|
| `src/cpu.rs` | New CPU state, decode, execution, and unit tests | `src/bus.rs`, `src/cartridge.rs` | Follow focused module style with private helpers and inline unit tests. |
| `src/lib.rs` | Library export for CPU module | `src/lib.rs` | Add `pub mod cpu;` beside existing modules. |

## Existing Patterns to Reuse

### Focused Module With Inline Unit Tests

Existing modules define public structs and keep helper functions private. Unit tests live in the same file and build small in-memory fixtures.

Relevant examples:
- `src/bus.rs` defines `Bus`, private index helpers, and `#[cfg(test)] mod tests`.
- `src/cartridge.rs` defines `Cartridge`, `CartridgeHeader`, error types, private parse helpers, and focused unit tests.

Phase 2 should put CPU unit tests in `src/cpu.rs` so private flag/fetch helpers can be tested where appropriate.

### Structured Unsupported Behavior

`src/cartridge.rs` returns explicit errors for unsupported cartridge types and size codes instead of panicking or silently accepting unsupported inputs. CPU unsupported opcodes should follow the same pattern.

Target pattern:
- Add `CpuError::UnsupportedOpcode { opcode: u8, pc: u16 }`.
- Add `CpuError::UnsupportedCbOpcode { opcode: u8, pc: u16 }` when the CB boundary lands.
- Tests should assert exact opcode and PC context.

### Bus as CPU-Visible Boundary

`src/bus.rs` owns address routing through `read8` and `write8`. CPU plans should route all opcode fetch, immediate operand fetch, `(HL)` memory access, high-memory access, absolute address access, stack push, and stack pop through the Bus API.

Relevant existing API:
```rust
pub fn read8(&self, addr: u16) -> u8
pub fn write8(&mut self, addr: u16, value: u8)
```

### Dependency-Free Rust

No dependencies are listed in `Cargo.toml`; current core behavior uses the Rust standard library. Phase 2 should keep this unless a later plan finds a strong correctness reason.

## Integration Guidance

- Export the CPU module from `src/lib.rs`.
- Do not add CPU execution to `src/main.rs` during Phase 2.
- Build CPU tests with in-memory `Vec<u8>` ROM fixtures, `Cartridge::from_bytes`, and `Bus::new`.
- Preserve Phase 1 behavior by running broad checks after each plan: `cargo fmt --all -- --check`, `cargo test`, and `cargo clippy -- -D warnings`.

## Plan Dependency Shape

All Phase 2 plan slices touch `src/cpu.rs`, so they should execute sequentially:

| Plan | Depends On | Reason |
|------|------------|--------|
| 02-01 | none | Creates CPU module, state, errors, step/fetch skeleton. |
| 02-02 | 02-01 | Implements first opcodes on the CPU API. |
| 02-03 | 02-02 | Reuses register/load helpers and adds flag-sensitive arithmetic. |
| 02-04 | 02-03 | Reuses fetch, register-pair, and Bus helpers for control flow and stack. |
| 02-05 | 02-04 | Finalizes CB boundary and cycle-returning behavior across implemented opcodes. |

---

*Phase: 02-cpu-core-foundation*
*Pattern map created: 2026-05-02*
