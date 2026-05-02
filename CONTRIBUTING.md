<!-- generated-by: gsd-doc-writer -->
# Contributing

Thank you for helping improve GameGirl. The project is early, so small, well-tested changes are easier to review than broad subsystem rewrites.

## Development Setup

See `docs/GETTING-STARTED.md` for prerequisites and first-run instructions. See `docs/DEVELOPMENT.md` for local development commands, style expectations, and pull-request preparation.

## Coding Standards

- Use Rust 2021 and Cargo.
- Format Rust code with `cargo fmt --all`.
- Run `cargo clippy --all-targets -- -D warnings` before opening a pull request.
- Keep reusable emulator logic in library modules under `src/`; keep command-line argument handling in `src/main.rs`.
- Add tests for new emulator behavior. Prefer unit tests near pure module logic and integration tests under `tests/` for binary behavior.

## PR Guidelines

- Keep the pull request focused on one behavior, subsystem, or documentation topic.
- Explain what changed and why.
- List the commands you ran, especially `cargo test`, `cargo fmt --all -- --check`, and `cargo clippy --all-targets -- -D warnings`.
- Include new or updated tests when behavior changes.
- Avoid adding third-party dependencies unless they clearly reduce risk or complexity.
- For emulator behavior, note any ROM fixtures or hardware references used to validate the change.

## Issue Reporting

Use GitHub Issues for bugs and feature requests.

For bugs, include:

- Steps to reproduce.
- Expected behavior.
- Actual behavior.
- ROM path or fixture used, if applicable.
- Output from the CLI or test command.
- Your Rust toolchain version.

For feature requests, include:

- The emulator subsystem involved.
- The user-visible behavior or compatibility goal.
- Any known test ROMs, references, or existing implementations that clarify the expected behavior.
