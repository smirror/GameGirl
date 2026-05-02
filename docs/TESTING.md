<!-- generated-by: gsd-doc-writer -->
# Testing

GameGirl uses Cargo's built-in Rust test harness. The current test suite covers cartridge parsing and ROM-only behavior, Bus address routing, and compiled CLI behavior.

## Test Framework And Setup

- Test framework: Cargo's built-in Rust test harness.
- Unit tests live beside implementation code in `src/cartridge.rs` and `src/bus.rs`.
- CLI integration tests live in `tests/cli.rs`.
- No third-party test framework is configured in `Cargo.toml`.
- No global test setup file is required.

## Running Tests

Run everything:

```bash
cargo test
```

Run cartridge-focused tests:

```bash
cargo test cartridge
```

Run Bus-focused tests:

```bash
cargo test bus
```

Run only the CLI integration test target:

```bash
cargo test --test cli
```

Run formatting and lint checks used during local verification:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

Verify that all checked-in ROM files can be loaded and parsed by the CLI:

```bash
scripts/verify_rom_loading.sh
```

## Writing New Tests

- Put pure module tests inside the same source file under `#[cfg(test)] mod tests`.
- Put compiled binary behavior tests under `tests/`.
- Use descriptive snake_case test names such as `rejects_rom_shorter_than_header` or `reports_missing_rom_file`.
- Keep emulator fixtures small where possible. `tests/cli.rs` creates temporary ROM bytes in the test instead of depending on a large external fixture.
- Use checked-in ROM suites under `roms/` only when the emulator behavior being tested is implemented enough to make the result meaningful.

Current examples:

| Area | File | Coverage |
|------|------|----------|
| Cartridge validation and metadata | `src/cartridge.rs` | Header length, title parsing, ROM/RAM size codes, unsupported type codes, ROM-only reads/writes. |
| Bus routing | `src/bus.rs` | ROM delegation, WRAM/Echo RAM mirroring, OAM, I/O, HRAM, IE, unusable range, unmapped range behavior. |
| CLI behavior | `tests/cli.rs` | Valid ROM load, `.GBC` extension handling, missing argument, invalid suffix, missing file, too-short ROM. |

## Coverage Requirements

No coverage threshold is configured. There is no `tarpaulin`, `llvm-cov`, `grcov`, or coverage configuration file in the repository.

## CI Integration

| Workflow | Trigger | Test-related behavior |
|----------|---------|-----------------------|
| `.github/workflows/rust.yml` | Push or pull request to `main` when Rust source, tests, scripts, workflow config, Cargo metadata, or checked-in ROMs change | Runs `cargo fmt --all`, `cargo clippy`, `cargo test --verbose`, `cargo build`, and `scripts/verify_rom_loading.sh`. |
| `.github/workflows/rust-clippy.yml` | Push, pull request, and a monthly schedule | Installs the stable Rust toolchain with clippy and uploads SARIF results. |
| `.github/workflows/dependency-review.yml` | Pull request to `main` | Runs `actions/dependency-review-action@v4`. |

The checked-in ROM directories `roms/blargg-gb-tests/`, `roms/mooneye/`, and `roms/hello-world/` are verified for loadability in CI. Behavioral pass/fail validation against those ROMs is still future emulator work.
