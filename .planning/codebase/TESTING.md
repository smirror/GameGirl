# Testing Patterns

**Analysis Date:** 2026-05-02

## Test Framework

**Runner:**
- Cargo test harness.
- No additional Rust testing crates are configured.

**Assertion Library:**
- Rust built-in test assertions, when tests are added.
- No custom matchers or snapshot tools are present.

**Run Commands:**

```bash
cargo test --verbose     # Run all tests, matching CI
cargo test               # Run all tests locally
cargo fmt --all          # Format Rust code
cargo clippy             # Run clippy locally
cargo build              # Build the binary
```

## Test File Organization

**Location:**
- No Rust tests are currently present.
- Future unit tests can live in `#[cfg(test)]` modules inside source files.
- Future integration tests can live under `tests/`.

**Naming:**
- Use Rust defaults: module unit tests inline, integration test files as `tests/<feature>.rs`.
- For ROM harness tests, use descriptive names such as `tests/blargg_cpu_instrs.rs` or `tests/mooneye_timer.rs`.

**Structure:**

```text
src/
  main.rs
tests/
  blargg_cpu_instrs.rs        # future integration harness
  mooneye_acceptance.rs       # future integration harness
roms/
  blargg-gb-tests/
  mooneye/
```

## Test Structure

**Suite Organization:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_expected_behavior() {
        // arrange
        // act
        // assert
    }
}
```

**Patterns:**
- Use small unit tests for pure CPU flag/register helpers.
- Use ROM-driven integration tests for instruction behavior, timing, and hardware interactions.
- Keep fixtures in `roms/` rather than embedding binary blobs into test source.

## Mocking

**Framework:**
- None configured.

**Patterns:**
- Prefer deterministic fake devices or in-memory cartridge data over mocking libraries.
- Build tests around explicit CPU/bus/device state.

**What to Mock:**
- Future display/audio output adapters.
- Future host input sources.
- File I/O at the CLI boundary.

**What NOT to Mock:**
- CPU instruction execution.
- Timer edge behavior.
- Bus memory mapping.
- Cartridge mapper behavior once implemented.

## Fixtures and Factories

**Test Data:**
- `roms/hello-world/hello-world.gb` - simple sample ROM.
- `roms/blargg-gb-tests/cpu_instrs/` - CPU instruction validation.
- `roms/blargg-gb-tests/instr_timing/` - instruction timing validation.
- `roms/blargg-gb-tests/mem_timing/` and `mem_timing-2/` - memory timing validation.
- `roms/mooneye/acceptance/` - acceptance tests for CPU, timer, interrupt, PPU, DMA, and boot behavior.
- `roms/mooneye/emulator-only/mbc*/` - MBC behavior tests.

**Location:**
- Shared ROM fixtures are already checked into `roms/`.
- Future factory helpers can live in `tests/common/` or module-local test helpers.

## Coverage

**Requirements:**
- No coverage target is defined.
- CI currently blocks on `cargo test` failures but does not enforce coverage.

**Configuration:**
- No coverage tooling configured.

**View Coverage:**
- Not available until a coverage tool such as cargo-llvm-cov is added.

## Test Types

**Unit Tests:**
- Scope: CPU register operations, flag helpers, instruction implementations, timer counters, memory map behavior.
- Mocking: prefer direct state setup.
- Speed: should be fast and deterministic.

**Integration Tests:**
- Scope: executing test ROMs until pass/fail output or known completion states.
- Fixtures: blargg and mooneye ROMs under `roms/`.
- Setup: future harness must define timeouts and pass/fail detection.

**E2E Tests:**
- None yet.
- Future emulator UI/audio workflows would require separate host-level tests.

## Common Patterns

**Error Testing:**

```rust
#[test]
fn rejects_invalid_cartridge_header() {
    let rom = vec![0; 0x150];
    let result = Cartridge::from_bytes(rom);
    assert!(result.is_err());
}
```

**ROM Testing:**

```rust
#[test]
fn blargg_cpu_instrs_passes() {
    // Load `roms/blargg-gb-tests/cpu_instrs/cpu_instrs.gb`.
    // Run emulator until serial pass/fail output or timeout.
    // Assert pass marker.
}
```

---

*Testing analysis: 2026-05-02*
*Update when test patterns change*
