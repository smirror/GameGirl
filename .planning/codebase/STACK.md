# Technology Stack

**Analysis Date:** 2026-05-02

## Languages

**Primary:**
- Rust 2021 edition - all executable application code currently lives in `src/main.rs`.

**Secondary:**
- Markdown - project documentation in `README.md`, `docs/hot_to_proceed.md`, `docs/gameboy_architecture_summary.md`, and ROM notes.
- Game Boy assembly - sample and upstream test ROM sources under `roms/**/source/` and `roms/hello-world/hello-world.asm`.
- YAML - GitHub Actions and repository automation under `.github/`.

## Runtime

**Environment:**
- Native Rust binary built with Cargo.
- No explicit minimum Rust toolchain is pinned in `Cargo.toml`, `rust-toolchain.toml`, or `.github/`.
- CI installs the stable Rust toolchain in `.github/workflows/rust-clippy.yml`.

**Package Manager:**
- Cargo - package metadata in `Cargo.toml`.
- Lockfile: `Cargo.lock` is present.

## Frameworks

**Core:**
- None. The current crate uses only Rust standard library APIs.

**Testing:**
- Cargo test harness - invoked by `.github/workflows/rust.yml`.
- External ROM fixtures - `roms/blargg-gb-tests/`, `roms/mooneye/`, and `roms/hello-world/` provide future emulator validation assets, but no automated harness consumes them yet.

**Build/Dev:**
- Cargo - build, format, lint, and test commands.
- rustfmt - run in `.github/workflows/rust.yml` through `cargo fmt --all`.
- Clippy - run in `.github/workflows/rust.yml` and SARIF-producing `.github/workflows/rust-clippy.yml`.

## Key Dependencies

**Critical:**
- Rust standard library - `std::env` for CLI argument collection and `std::fs` for file reading in `src/main.rs`.

**Infrastructure:**
- GitHub Actions - CI, labeling, first-interaction greeting, dependency review, auto-assignment, and clippy SARIF upload.
- No crates are listed in `[dependencies]` in `Cargo.toml`.

## Configuration

**Environment:**
- No application environment variables are required.
- Runtime configuration is currently only the first CLI argument: a path ending in `.gb` or `.gbc`.

**Build:**
- `Cargo.toml` - Rust package metadata.
- `Cargo.lock` - dependency lockfile.
- `.editorconfig` - editor formatting baseline.
- `.github/workflows/*.yml` - CI automation.

## Platform Requirements

**Development:**
- Any platform with a compatible Rust toolchain and Cargo.
- No external database, service, or local daemon is required.

**Production:**
- Not defined yet. The package metadata describes "A GameBoy emulator written in Rust", but the current executable is only a prototype CLI.

---

*Stack analysis: 2026-05-02*
*Update after major dependency or runtime changes*
