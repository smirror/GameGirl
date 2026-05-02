---
quick_id: 260502-gih
type: quick
status: complete
date: 2026-05-02
description: "Incorporate roadmap feedback: insert early minimal serial ROM validation harness, narrow CPU phase scope, and clarify ROM-only/MBC and post-boot assumptions"
---

# Quick Task 260502-gih Plan

## Objective

Incorporate roadmap feedback so validation appears earlier, CPU implementation is staged more safely, and v1.0 assumptions around cartridge support and boot state are explicit.

## Tasks

### Task 1: Update roadmap phase sequencing

**Files**: `.planning/ROADMAP.md`

**Action**: Insert Phase 2.1 for minimal serial ROM validation, split Phase 2 CPU work into smaller plan slices, rename Phase 4 as validation harness expansion, and update execution order/progress counts.

**Verify**: Roadmap shows `1 -> 2 -> 2.1 -> 3 -> 4 -> 5` and Phase 2 has five plans.

### Task 2: Update requirements and traceability

**Files**: `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md`, `.planning/STATE.md`

**Action**: Add explicit requirements for non-ROM-only cartridge rejection, post-boot DMG startup, serial test output capture, and expanded blargg/mooneye validation. Update traceability and state totals.

**Verify**: Requirement coverage totals match mapped requirements and Phase 2.1 owns `SERIAL-01`, `TEST-02`, and `TEST-03`.

### Task 3: Remove stale Phase 4-only harness references

**Files**: `.planning/phases/01-cartridge-and-bus-foundation/*.md`

**Action**: Align existing Phase 1 context/research/plan artifacts so serial pass/fail and deterministic ROM timeout work now points to Phase 2.1 while broader blargg/mooneye expansion remains Phase 4.

**Verify**: Search results no longer claim that serial pass/fail detection must wait until Phase 4.
