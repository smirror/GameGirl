# Phase 2: CPU Core Foundation - Context

**Gathered:** 2026-05-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 2 adds a reusable DMG CPU core foundation. It should model CPU registers and flags, initialize from documented post-boot DMG state, fetch/decode/execute opcodes through the existing `Bus`, implement the staged instruction groups listed in the roadmap, and report elapsed machine cycles for downstream device timing.

This phase does **not** emulate the Nintendo boot ROM, run a ROM validation harness, capture serial output, implement timer/interrupt behavior, model PPU access rules, add rendering, add broad MBC support, add audio, add host UI, or add CGB behavior. Those remain later-phase or v2 work.

</domain>

<decisions>
## Implementation Decisions

### CPU State and Step Surface
- **D-01:** Add a focused reusable CPU module, expected to be `src/cpu.rs` or an equivalent module exported from `src/lib.rs`. Keep CPU logic out of `src/main.rs`.
- **D-02:** Provide a documented DMG post-boot constructor such as `Cpu::new_dmg()`. It should start at `PC = 0x0100`, `SP = 0xFFFE`, and use the documented DMG register defaults from project docs, including `A = 0x01`, `B = 0x00`, `C = 0x13`, `D = 0x00`, `E = 0xD8`, `H = 0x01`, `L = 0x4D`, and `F = 0xB0` unless research finds a stronger project-local contradiction. Always mask the lower nibble of `F` to zero.
- **D-03:** Provide test-oriented construction or mutation helpers so unit tests can set registers, `PC`, and `SP` without relying on hidden setup or CPU execution side effects.
- **D-04:** Instruction execution should happen through a `step`-style API that receives a mutable `Bus` or equivalent. Opcode fetch and operand reads/writes must use `Bus::read8` / `Bus::write8`; CPU code should not read cartridge bytes or memory arrays directly.
- **D-05:** `step` should return a structured outcome or `Result` that includes elapsed machine cycles. Machine cycles are the primary reporting unit for Phase 2; later devices can translate to T-cycles/dots as needed.
- **D-06:** The CPU core should not print to stdout/stderr. The CLI remains a boundary around loading/reporting only.

### Unsupported and Staged Opcode Behavior
- **D-07:** Unimplemented opcodes should fail deterministically through a structured unsupported-opcode result/error containing the opcode and opcode `PC`. Do not panic, exit the process, or silently treat unimplemented opcodes as `NOP`.
- **D-08:** Unsupported-opcode behavior should be documented and tested. Preferred behavior is to preserve pre-step CPU state on unsupported opcodes while still reporting the opcode address and byte; if the planner chooses PC advancement during fetch, that choice must be explicit in tests.
- **D-09:** Keep instruction implementation aligned to the roadmap slices: basic fetch/register state first, then `NOP`/`HALT`/`STOP` placeholders and basic 8-bit loads, then arithmetic/logical helpers, then control-flow/stack helpers, then the CB-prefix boundary.
- **D-10:** `HALT` and `STOP` are placeholders in Phase 2. They may record halted/stopped state and return deterministic cycles, but must not implement interrupt wake behavior, the HALT bug, speed switching, timer effects, or CGB behavior.
- **D-11:** Arithmetic/logical helpers should prioritize flag correctness with targeted tests for `Z`, `N`, `H`, and `C`, especially lower-nibble carry/borrow behavior. `DAA` should remain deferred unless planned as a small isolated, test-heavy addition inside the arithmetic slice.
- **D-12:** Stack helpers must use Bus-backed reads/writes and little-endian push/pop behavior. CPU code should not bypass the Bus for stack memory.

### CB-Prefix Boundary
- **D-13:** Recognize `0xCB` as a prefixed opcode boundary in Phase 2 even if CB operations themselves are not implemented yet.
- **D-14:** Prefer a CB skeleton that fetches or identifies the second opcode byte and returns a structured unsupported-CB result/error containing the subopcode and original opcode `PC`.
- **D-15:** If CB operations are deferred, every CB opcode should fail deterministically. Do not implement broad bit/rotate/shift behavior unless it is deliberately scoped into the `02-05` plan slice.
- **D-16:** Decoder organization is flexible. A `match`-based decoder is acceptable at this stage if helpers and tests make later expansion straightforward.

### Validation Boundary
- **D-17:** Phase 2 validation should focus on unit tests for CPU initialization, register/flag helpers, Bus-backed fetch, `PC` advancement for implemented instructions, implemented opcode side effects, stack behavior, and cycle returns.
- **D-18:** ROM-driven execution, deterministic timeouts, and serial pass/fail capture remain Phase 2.1. Phase 2 should still leave a clean stepping surface that Phase 2.1 can drive.
- **D-19:** Keep the implementation dependency-free unless a dependency removes real emulator-correctness risk. For Phase 2, the expected path is standard-library Rust plus Cargo tests.

### the agent's Discretion
- Exact type names for `Cpu`, `Registers`, flags, cycle counts, and CPU errors/outcomes.
- Whether CPU state is exposed through public fields, getters, snapshots, or test-only helpers, provided tests can inspect important state cleanly.
- Exact decoder shape, helper function boundaries, and file splits inside the CPU module.
- Exact wording of unsupported-opcode errors, provided they include opcode and opcode-address context.
- Exact test fixture helpers and assertion style.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope and Requirements
- `.planning/ROADMAP.md` — Phase 2 goal, five plan slices, success criteria, and explicit deferral of serial harnessing to Phase 2.1.
- `.planning/REQUIREMENTS.md` — `BOOT-01` and `CPU-01` through `CPU-06` define the Phase 2 requirement surface.
- `.planning/PROJECT.md` — DMG-first, Bus-centered, dependency-light, correctness-first project constraints.
- `.planning/phases/01-cartridge-and-bus-foundation/01-CONTEXT.md` — Locked Phase 1 decisions: reusable core modules, thin CLI, Bus-centered reads/writes, explicit unsupported behavior, and deferred ROM harnessing.

### Game Boy Hardware Notes
- `docs/gameboy_architecture_summary.md` — Local DMG hardware summary for CPU registers, flags, post-boot values, memory map, interrupt vectors, and timing model.
- `docs/hot_to_proceed.md` — Local implementation notes: Bus-centered design, CPU/device separation, instruction group order, and test-ROM philosophy.

### Current Code Integration Points
- `src/lib.rs` — Reusable emulator module exports; Phase 2 should add/export the CPU module here.
- `src/bus.rs` — Existing `Bus::read8` / `Bus::write8` API and memory-range behavior that CPU fetch/stack logic must use.
- `src/cartridge.rs` — ROM-only cartridge behavior backing Bus ROM reads.
- `src/main.rs` — Thin CLI boundary; CPU execution should stay out of this file for Phase 2 unless a later plan explicitly adds a CLI stepping mode.
- `tests/cli.rs` — Existing integration-test style for user-facing CLI behavior; CPU behavior should mostly use focused unit tests.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Bus` in `src/bus.rs` already provides `read8(addr: u16)` and `write8(addr: u16, value: u8)` across cartridge ROM, WRAM, Echo RAM, OAM placeholder storage, I/O register storage, HRAM, and IE.
- `Cartridge` in `src/cartridge.rs` already gives ROM-only read behavior and a deterministic unsupported-cartridge pattern.
- `src/lib.rs` is already present and exports reusable core modules, so adding `pub mod cpu;` follows the established direction.
- Existing tests use small in-memory ROM fixtures for core behavior and CLI integration tests for process-boundary behavior.

### Established Patterns
- The crate is Rust 2021 and dependency-free.
- Emulator core modules should return structured results/errors instead of printing or panicking on ordinary unsupported behavior.
- Hardware responsibilities are separated by module: cartridge and bus exist; CPU should join that core layer without absorbing timer, PPU, joypad, APU, or host concerns.
- Tests are expected to run through normal Cargo commands and remain compatible with `cargo fmt`, `cargo test`, and `cargo clippy`.

### Integration Points
- CPU fetch and stack operations connect to `Bus`, not directly to `Cartridge` or raw ROM bytes.
- CPU initialization uses documented post-boot DMG state while leaving boot ROM emulation out of scope.
- Future Phase 2.1 ROM harnessing should be able to create a CPU, create/load a Bus, and call the Phase 2 `step` surface under a deterministic limit.
- Future timer/interrupt work should receive machine-cycle counts from CPU steps without forcing Phase 2 to implement timer or interrupt semantics early.

</code_context>

<specifics>
## Specific Ideas

- The interactive question tool was unavailable in this Codex mode, so the workflow fallback selected conservative defaults for all identified gray areas.
- Use deterministic unsupported behavior as a recurring design pattern, mirroring Phase 1's explicit unsupported cartridge type errors.
- Keep Phase 2 boring in the best way: state, fetch, flags, stack, cycles, and tests before any attempt to run interesting ROMs.

</specifics>

<deferred>
## Deferred Ideas

- Serial transfer register handling and deterministic ROM test harnessing — Phase 2.1.
- Timer registers, `IF`/`IE`/`IME`, delayed `ei`, interrupt service, and HALT wake behavior — Phase 3.
- Expanded blargg/mooneye validation harnessing — Phase 4.
- PPU mode/access restrictions and interrupt hooks — Phase 5.
- Nintendo boot ROM emulation, full rendering, Joypad input, broad MBC support, save RAM, APU/audio, host UI, and CGB behavior — v2 or later.

</deferred>

---

*Phase: 02-cpu-core-foundation*
*Context gathered: 2026-05-02*
