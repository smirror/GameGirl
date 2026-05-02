<!-- generated-by: gsd-doc-writer -->
# Configuration

GameGirl currently has almost no runtime configuration. The emulator foundation is configured by Cargo metadata, repository tooling files, GitHub Actions workflow files, and the ROM path passed to the CLI.

## Environment Variables

No application environment variables are read by `src/main.rs`, `src/cartridge.rs`, or `src/bus.rs`.

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| None | No | None | The current Rust code does not reference environment variables. |

## Config File Format

| File | Format | Purpose |
|------|--------|---------|
| `Cargo.toml` | TOML | Defines the Rust package name, version, edition, license metadata, repository, readme, keywords, and dependencies. |
| `Cargo.lock` | TOML-like Cargo lockfile | Records the resolved dependency graph. The current dependency graph has no third-party Rust crates. |
| `.editorconfig` | EditorConfig | Sets UTF-8, LF endings, final newlines, four-space indentation by default, two-space YAML indentation, and Markdown trailing-space behavior. |
| `renovate.json` | JSON | Configures Renovate. |
| `.github/labeler.yml` | YAML | Maps changed files to repository labels. |
| `.github/dependabot.yml` | YAML | Configures weekly GitHub Actions and Go module dependency update checks. |
| `.github/auto_assign.yml` | YAML | Configures pull-request auto-assignment behavior. |
| `.github/workflows/rust.yml` | YAML | Runs formatting, clippy, and tests for Rust source changes on `main`. |
| `.github/workflows/rust-clippy.yml` | YAML | Runs a scheduled and PR/push clippy SARIF analysis workflow. |
| `.github/workflows/dependency-review.yml` | YAML | Runs dependency review on pull requests. |

`Cargo.toml` is the main project configuration file:

```toml
[package]
name = "game_girl"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "A GameBoy emulator written in Rust"

[dependencies]
```

## Required vs Optional Settings

The CLI requires exactly one user-provided runtime setting: a ROM path argument.

| Setting | Required | Validation | Failure behavior |
|---------|----------|------------|------------------|
| ROM path argument | Yes | Must be present as the first positional argument. | Prints `Usage: {program} <rom.gb|rom.gbc>` and exits with failure. |
| ROM path extension | Yes | Extension must be `.gb` or `.gbc`, case-insensitive. | Prints `File must be a .gb or .gbc file` and exits with failure. |
| ROM file readability | Yes | `std::fs::read` must successfully read the file. | Returns a `CartridgeError::Io` message. |
| ROM header length | Yes | ROM must be at least `MIN_CARTRIDGE_HEADER_LEN` bytes. | Returns a `CartridgeError::TooShort` message. |
| Cartridge type | Yes | Known Game Boy cartridge type codes are recognized for metadata loading. | Unknown type codes return `CartridgeError::UnsupportedCartridgeType`. |
| ROM size code | Yes | Codes `0x00` through `0x08` are supported. | Other codes return `CartridgeError::UnsupportedRomSize`. |
| RAM size code | Yes | Codes `0x00` through `0x05` are supported. | Other codes return `CartridgeError::UnsupportedRamSize`. |

## Defaults

| Area | Default | Location |
|------|---------|----------|
| Cargo edition | Rust 2021 | `Cargo.toml` |
| Runtime dependencies | No third-party Rust dependencies | `Cargo.toml` |
| Bus WRAM | Zero-filled `[u8; 0x2000]` | `src/bus.rs` |
| Bus OAM | Zero-filled `[u8; 0xA0]` | `src/bus.rs` |
| Bus I/O registers | Zero-filled `[u8; 0x80]` | `src/bus.rs` |
| Bus HRAM | Zero-filled `[u8; 0x7F]` | `src/bus.rs` |
| Interrupt enable byte | `0` | `src/bus.rs` |
| Unusable and unmapped reads | `0xFF` | `src/bus.rs` |
| ROM and MBC register writes | Ignored until bank-controller behavior is implemented | `src/cartridge.rs` |

## Per-Environment Overrides

No development, staging, production, or test-specific application configuration files are present. The repository does not include `.env`, `.env.example`, `.env.development`, `.env.production`, Docker, or deployment-platform configuration files.

CI behavior is controlled separately by files under `.github/workflows/`.
