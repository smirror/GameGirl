<!-- generated-by: gsd-doc-writer -->
# Getting Started

This guide gets a local GameGirl checkout to the point where it builds, runs, and loads a ROM file through the current CLI.

## Prerequisites

- Rust and Cargo are required.
- The repository does not pin an exact Rust version. `Cargo.toml` does not declare a `rust-version`, and no Rust toolchain pin file is present.
- CI installs the stable Rust toolchain for clippy analysis in `.github/workflows/rust-clippy.yml`.
- No database, service, emulator frontend, or environment-variable setup is required.

## Installation Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/smirror/GameGirl.git
   ```

2. Enter the project directory:

   ```bash
   cd GameGirl
   ```

3. Build the Rust crate:

   ```bash
   cargo build
   ```

4. Run the full test suite:

   ```bash
   cargo test
   ```

## First Run

Use the checked-in hello-world ROM fixture:

```bash
cargo run -- roms/hello-world/hello-world.gb
```

The current CLI prints the number of bytes loaded from the ROM:

```text
Loaded ROM: 32768 bytes
```

## Common Setup Issues

| Symptom | Cause | Fix |
|---------|-------|-----|
| `Usage: ... <rom.gb\|rom.gbc>` | No ROM path was provided. | Pass a `.gb` or `.gbc` path after `--`. |
| `File must be a .gb or .gbc file` | The path extension is not supported. | Use a file ending in `.gb` or `.gbc`; uppercase variants such as `.GBC` are accepted. |
| `could not read ROM ...` | The path does not exist or cannot be read. | Check the file path and filesystem permissions. |
| `ROM is too short ... expected at least 336 bytes` | The file is smaller than the Game Boy cartridge header region. | Use a real Game Boy ROM or a test fixture with at least `0x150` bytes. |
| `unsupported cartridge type ...` | The ROM uses a cartridge type that is not implemented yet. | Use a ROM-only cartridge for the current emulator foundation. |

## Next Steps

- Read `docs/ARCHITECTURE.md` for the module layout and data flow.
- Read `docs/DEVELOPMENT.md` before changing code.
- Read `docs/TESTING.md` for focused test commands and CI behavior.
- Read `docs/CONFIGURATION.md` for supported runtime inputs and repository configuration files.
