# Codebase Concerns

**Analysis Date:** 2026-05-02

## Tech Debt

**Single-file prototype:**
- Issue: All runtime logic is in `src/main.rs`.
- Why: The project is at the scaffold/prototype stage.
- Impact: No natural place yet for emulator concepts such as CPU, Bus, Cartridge, Timer, PPU, Joypad, or APU.
- Fix approach: Introduce small Rust modules as each emulator subsystem is implemented.

**ROM loading reads binary files as text:**
- Issue: `src/main.rs` uses `fs::read_to_string(file_path)`.
- Why: Placeholder CLI behavior.
- Impact: Real `.gb` and `.gbc` files are binary and may fail UTF-8 parsing or be corrupted conceptually by text handling.
- Fix approach: Replace with `fs::read(file_path)` and pass raw bytes into a cartridge loader.

**Extension check only:**
- Issue: Valid input is checked only by `.gb` or `.gbc` suffix.
- Why: Minimal placeholder validation.
- Impact: Invalid files with matching suffix pass the boundary; valid uppercase or unusual paths may be rejected.
- Fix approach: Validate readable bytes, minimum header length, Nintendo logo/header fields where appropriate, and cartridge type.

## Known Bugs

**Binary ROM read failure:**
- Symptoms: Many ROMs will panic with "Something went wrong reading the file" if their bytes are not valid UTF-8.
- Trigger: Run the current binary against a normal binary ROM, for example a file under `roms/blargg-gb-tests/`.
- Workaround: None in current code.
- Root cause: `fs::read_to_string` instead of `fs::read`.

**CI format workflow references missing step id:**
- Symptoms: `.github/workflows/rust.yml` has `if: steps.git-check.outputs.modified == 'true'`, but no `git-check` step is defined.
- Trigger: GitHub Actions evaluates the "push changes" step.
- Workaround: The condition likely evaluates false or the workflow fails depending on Actions expression handling.
- Root cause: Missing git diff/check step before attempting to push formatting changes.

## Security Considerations

**Untrusted ROM input:**
- Risk: Emulator will eventually parse arbitrary binary data.
- Current mitigation: Only a suffix check exists.
- Recommendations: Treat ROM files as untrusted input, avoid unsafe Rust unless absolutely necessary, check bounds before indexing, and fuzz parsers once cartridge loading exists.

**CI token usage in formatting workflow:**
- Risk: `.github/workflows/rust.yml` rewrites origin with `secrets.GITHUB_TOKEN` in a shell command to push formatting changes.
- Current mitigation: GitHub masks secrets in logs, but the step is currently gated by a missing step id.
- Recommendations: Prefer a dedicated formatting check that fails on unformatted code, or use a well-maintained formatting action with least privilege.

## Performance Bottlenecks

**No emulator loop yet:**
- Problem: There are no measured performance bottlenecks because core emulation is not implemented.
- Measurement: Not available.
- Cause: Project is still scaffold-level.
- Improvement path: Once CPU stepping exists, add benchmarks around instruction execution and frame timing before optimizing.

**Future ROM loading:**
- Problem: Current text loading would be incorrect and inefficient for binary ROM data.
- Measurement: Not measured.
- Cause: Placeholder file read path.
- Improvement path: Use byte loading and avoid repeated copies when introducing cartridge parsing.

## Fragile Areas

**Future CPU flags and timing:**
- Why fragile: Game Boy behavior depends on exact `Z/N/H/C` flags, delayed `ei`, HALT behavior, interrupt timing, and instruction cycles.
- Common failures: Passing simple demos while failing blargg/mooneye edge cases.
- Safe modification: Implement instructions in small groups with unit tests and ROM tests.
- Test coverage: No emulator tests exist yet.

**Future Timer implementation:**
- Why fragile: `DIV`, `TIMA`, `TMA`, and `TAC` depend on internal counter bits and falling-edge behavior.
- Common failures: Incorrect reload timing and missed timer interrupt edge cases.
- Safe modification: Model an internal system counter from the start.
- Test coverage: ROM fixtures exist under `roms/mooneye/acceptance/timer/`, but no harness exists.

**Future PPU access restrictions:**
- Why fragile: VRAM/OAM access depends on PPU mode timing.
- Common failures: Rendering works in simple cases but fails timing tests or sprite/OAM edge cases.
- Safe modification: Introduce PPU mode state before full rendering so bus access rules have a home.
- Test coverage: PPU acceptance ROMs exist under `roms/mooneye/acceptance/ppu/`, but no harness exists.

## Scaling Limits

**Repository size and ROM fixtures:**
- Current capacity: Many binary fixtures are checked into `roms/`.
- Limit: Repository clone size and CI checkout time may grow if more ROM suites are added.
- Symptoms at limit: Slow checkouts and storage churn.
- Scaling path: Keep only legally safe, high-value fixtures in git; document optional external suites if needed.

## Dependencies at Risk

**actions-rs GitHub Actions:**
- Risk: `.github/workflows/rust.yml` and `.github/workflows/rust-clippy.yml` use `actions-rs` actions, which have seen limited maintenance.
- Impact: CI may break as GitHub runner/toolchain behavior changes.
- Migration plan: Use `dtolnay/rust-toolchain` or direct `rustup`/`cargo` commands.

**Dependabot Go module configuration:**
- Risk: `.github/dependabot.yml` includes `package-ecosystem: "gomod"` at `/`, but the repo currently has no `go.mod`.
- Impact: No useful Go updates; potential confusion in dependency automation.
- Migration plan: Remove the gomod entry unless a Go component is introduced.

## Missing Critical Features

**Cartridge byte loader:**
- Problem: No binary ROM loader or cartridge header parser exists.
- Current workaround: None.
- Blocks: Any real emulator behavior.
- Implementation complexity: Low for MBC0/header basics, higher for MBC variants.

**CPU core:**
- Problem: No registers, fetch/decode/execute loop, instruction set, or timing model exists.
- Current workaround: None.
- Blocks: All emulator execution.
- Implementation complexity: High but can be phased by instruction groups.

**Bus and memory map:**
- Problem: No address map or read/write interface exists.
- Current workaround: None.
- Blocks: CPU, cartridge, timer, PPU, joypad, and interrupt integration.
- Implementation complexity: Medium.

**Automated ROM test harness:**
- Problem: Validation ROMs are present but not executable through tests.
- Current workaround: Manual future use.
- Blocks: Confidence in CPU/timing/hardware compatibility.
- Implementation complexity: Medium.

## Test Coverage Gaps

**All emulator behavior:**
- What's not tested: CPU, bus, cartridge, timer, interrupts, PPU, joypad, APU.
- Risk: Core correctness regressions will go unnoticed once implementation begins.
- Priority: High.
- Difficulty to test: Requires emulator harness design and pass/fail detection for ROM suites.

**CLI input behavior:**
- What's not tested: missing argument, invalid extension, unreadable file, binary ROM loading.
- Risk: Regressions at the only current runtime boundary.
- Priority: Medium.
- Difficulty to test: Low.

---

*Concerns audit: 2026-05-02*
*Update as issues are fixed or new ones discovered*
