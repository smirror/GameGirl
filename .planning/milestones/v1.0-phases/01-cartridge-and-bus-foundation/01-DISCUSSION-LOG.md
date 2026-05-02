# Phase 1: Cartridge and Bus Foundation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-02
**Phase:** 01-cartridge-and-bus-foundation
**Areas discussed:** Module boundaries, Cartridge loading and validation, Bus scope and memory behavior, Testing boundary

---

## Module Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Keep everything in `src/main.rs` | Fastest immediate patch, but prolongs the single-file prototype | |
| Create `src/lib.rs` plus focused modules | Makes cartridge and Bus testable and reusable by future CPU/host code | ✓ |
| You decide | Let the planner choose during implementation | |

**User's choice:** Runtime fallback selected the recommended option because the interactive question tool is unavailable in this mode.
**Notes:** This matches `.planning/research/SUMMARY.md` and project constraints: thin CLI, reusable core, Bus-centered architecture.

---

## Cartridge Loading and Validation

| Option | Description | Selected |
|--------|-------------|----------|
| Strict full validation | Validate logo/checksum immediately and reject anything suspicious | |
| Pragmatic header validation | Hard-fail unreadable/too-short/unsupported basics; parse metadata; defer full logo/checksum hard gate | ✓ |
| Minimal byte loading | Only read bytes and defer most validation | |

**User's choice:** Runtime fallback selected the recommended option because the interactive question tool is unavailable in this mode.
**Notes:** Phase 1 needs real binary ROM loading and useful errors without accidentally becoming a full compatibility-validation phase.

---

## Bus Scope and Memory Behavior

| Option | Description | Selected |
|--------|-------------|----------|
| Only cartridge and RAM | Smallest Bus, but too thin for later CPU work | |
| Core DMG map skeleton | Include cartridge ROM, WRAM/Echo, OAM placeholder/storage, unusable range, I/O stubs, HRAM, IE | ✓ |
| Full device-accurate map | Adds timer/PPU/interrupt semantics too early | |

**User's choice:** Runtime fallback selected the recommended option because the interactive question tool is unavailable in this mode.
**Notes:** This keeps Phase 1 within scope while preventing CPU code from bypassing the Bus later.

---

## Testing Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Unit tests only | Simple, but may miss CLI/core integration problems | |
| Unit plus focused integration tests | Covers cartridge parsing, errors, and Bus ranges without starting ROM execution harness work | ✓ |
| Start ROM harness now | Valuable, but minimal serial ROM harness now belongs to inserted Phase 2.1 after CPU foundation; broader blargg/mooneye expansion remains Phase 4 | |

**User's choice:** Runtime fallback selected the recommended option because the interactive question tool is unavailable in this mode.
**Notes:** Tests should use in-memory byte fixtures first; checked-in ROM files are optional smoke inputs, not Phase 1 pass/fail execution.

---

## the agent's Discretion

- Exact Rust module names and layout inside the core boundary.
- Exact custom error type names and text, as long as user-facing failures are clear.
- Whether OAM has simple storage immediately or a prepared placeholder.
- Whether logo/checksum fields are parsed now or deferred.

## Deferred Ideas

- CPU execution and cycle reporting.
- Timer and interrupt behavior.
- ROM pass/fail harness.
- PPU rendering and PPU access restrictions beyond simple Bus scaffolding.
- Broad MBC support, save RAM, CGB, audio, and desktop UI.
