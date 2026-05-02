<!-- generated-by: gsd-doc-writer -->
# Development

GameGirl is a Rust 2021 Cargo project. Development currently focuses on small, testable emulator-core modules and a thin CLI wrapper.

## Local Setup

1. Fork or clone the repository.
2. Build the crate:

   ```bash
   cargo build
   ```

3. Run tests before changing behavior:

   ```bash
   cargo test
   ```

4. Run formatting and lint checks before opening a pull request:

   ```bash
   cargo fmt --all
   cargo clippy --all-targets -- -D warnings
   ```

No `.env` file, local service, database, or generated source step is required.

## Build Commands

| Command | Description |
|---------|-------------|
| `cargo build` | Compiles the binary and library crate. |
| `cargo run -- roms/hello-world/hello-world.gb` | Runs the CLI against the checked-in hello-world ROM fixture. |
| `cargo test` | Runs unit tests, integration tests, and doc tests. |
| `cargo test cartridge` | Runs tests whose names or module paths include `cartridge`. |
| `cargo test bus` | Runs tests whose names or module paths include `bus`. |
| `cargo test --test cli` | Runs the CLI integration tests in `tests/cli.rs`. |
| `cargo fmt --all` | Formats all Rust code with rustfmt. |
| `cargo fmt --all -- --check` | Checks Rust formatting without rewriting files. |
| `cargo clippy --all-targets -- -D warnings` | Runs clippy for all targets and treats warnings as errors. |

## Code Style

- Rust code follows standard rustfmt formatting. The repository uses `.editorconfig` for editor defaults and `.github/workflows/rust.yml` runs `cargo fmt --all`.
- Clippy is used for linting. `.github/workflows/rust.yml` runs `cargo clippy`, and `.github/workflows/rust-clippy.yml` runs a SARIF-producing clippy analysis workflow.
- Keep CLI behavior in `src/main.rs`; put reusable emulator logic in library modules exported by `src/lib.rs`.
- Prefer small unit tests for pure emulator behavior and integration tests for compiled CLI behavior.
- Avoid adding dependencies unless they remove meaningful implementation risk; `Cargo.toml` currently has an empty `[dependencies]` section.

## Branch Conventions

No formal branch naming convention is documented in the repository. Use short descriptive names that identify the work area, such as `feature/add-cpu` for feature work or `fix/rom-loading-error` for bug fixes.

## PR Process

- Keep each pull request focused on one subsystem or behavior change.
- Run `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` locally before requesting review.
- Include tests for new cartridge, bus, CPU, timer, PPU, or CLI behavior.
- Describe which ROM fixtures or focused tests were used for validation.
- Watch CI results from `.github/workflows/rust.yml`, `.github/workflows/rust-clippy.yml`, and `.github/workflows/dependency-review.yml`.
