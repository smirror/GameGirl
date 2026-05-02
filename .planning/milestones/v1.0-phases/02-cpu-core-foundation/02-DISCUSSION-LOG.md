# Phase 2: CPU Core Foundation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-02
**Phase:** 2-CPU Core Foundation
**Areas discussed:** CPU stepping surface, Unsupported and staged opcode behavior, CB-prefix boundary, Validation boundary

---

## CPU Stepping Surface

| Option | Description | Selected |
|--------|-------------|----------|
| Bus-backed step result with cycles | Add a reusable CPU module whose `step`-style API fetches through `Bus` and returns machine cycles. | yes |
| Trace-heavy debugger surface | Add richer trace/debug output now, beyond what Phase 2 needs for tests. | |
| CLI-driven execution now | Wire CPU execution into the CLI immediately. | |

**User's choice:** Workflow fallback selected the recommended default because `request_user_input` is unavailable in this Codex mode.
**Notes:** This keeps CPU reusable for later hosts and avoids turning the CLI into an emulator runner before the core is ready.

---

## Unsupported and Staged Opcode Behavior

| Option | Description | Selected |
|--------|-------------|----------|
| Structured deterministic errors | Return unsupported-opcode results/errors with opcode and PC context. | yes |
| Panic/fail fast | Panic on unimplemented opcodes during early development. | |
| Treat as NOP | Ignore unknown opcodes and continue execution. | |

**User's choice:** Workflow fallback selected the recommended default.
**Notes:** This mirrors Phase 1's explicit unsupported cartridge behavior and prevents false success while instruction coverage is incomplete.

---

## CB-Prefix Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| CB skeleton with unsupported subopcodes | Recognize `0xCB`, identify the second byte, and fail deterministically until CB operations are implemented. | yes |
| Generic unsupported 0xCB | Treat the prefix byte itself as just another unsupported opcode. | |
| Implement CB subset now | Pull bit/rotate/shift work forward before the roadmap slice calls for it. | |

**User's choice:** Workflow fallback selected the recommended default.
**Notes:** This gives downstream planning a clear 02-05 boundary without pretending CB behavior exists early.

---

## Validation Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Unit tests first | Cover CPU state, flags, Bus-backed fetch, PC/stack behavior, and cycle returns with focused tests. | yes |
| ROM harness now | Add deterministic ROM execution and serial pass/fail capture in Phase 2. | |
| No extra tests until Phase 2.1 | Build CPU code first and defer most verification. | |

**User's choice:** Workflow fallback selected the recommended default.
**Notes:** This respects Phase 2.1 as the inserted serial/ROM harness phase while keeping Phase 2 testable.

---

## the agent's Discretion

- Exact Rust type names and file organization inside the CPU module.
- Exact decoder implementation shape, provided staged opcodes and tests remain clear.
- Exact CPU error/outcome enum names and wording.
- Exact test helper design for setting and inspecting CPU state.

## Deferred Ideas

- Serial test output and deterministic ROM harnessing remain Phase 2.1.
- Timer, interrupt service, delayed `ei`, and HALT wake behavior remain Phase 3.
- PPU access restrictions, rendering, MBC expansion, audio, host UI, CGB behavior, and Nintendo boot ROM emulation remain out of scope for Phase 2.
