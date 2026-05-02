# Pitfalls Research

**Domain:** Rust DMG Game Boy emulator
**Researched:** 2026-05-02
**Confidence:** HIGH

## Critical Pitfalls

### Pitfall 1: Treating ROMs as Text

**What goes wrong:**
Real `.gb` files fail to load or are conceptually handled as text instead of bytes.

**Why it happens:**
The current CLI prototype uses `fs::read_to_string`, which is convenient for text files but wrong for binary cartridges.

**How to avoid:**
Use `std::fs::read` and pass `Vec<u8>` into a cartridge parser.

**Warning signs:**
Panics on valid ROMs, lossy conversions, or code that assumes UTF-8.

**Phase to address:**
Phase 1.

---

### Pitfall 2: Scattering Memory Map Logic

**What goes wrong:**
CPU code accumulates address ranges and device side effects directly.

**Why it happens:**
It seems quicker to special-case reads/writes close to instruction code.

**How to avoid:**
Create a Bus abstraction before broad instruction work.

**Warning signs:**
Multiple files match on `0xFF00`, `0x8000`, `0xA000`, or cartridge ranges independently.

**Phase to address:**
Phase 1.

---

### Pitfall 3: Adding Timing Too Late

**What goes wrong:**
Instructions work in isolation but timer, interrupt, HALT, and PPU tests fail.

**Why it happens:**
Cycle accounting is deferred until after opcode behavior exists.

**How to avoid:**
Have instruction execution return elapsed cycles from the first CPU milestone and advance timer/PPU state through that path.

**Warning signs:**
Instruction functions return only values/state, not timing; tests ignore cycles.

**Phase to address:**
Phase 2 and Phase 3.

---

### Pitfall 4: Incorrect Timer Model

**What goes wrong:**
Timer tests fail around DIV writes, TAC changes, overflow reload, and interrupt requests.

**Why it happens:**
Developers model DIV/TIMA as independent counters instead of using the internal system counter behavior.

**How to avoid:**
Model a system counter and use selected-bit falling edges for TIMA ticks. Respect one-M-cycle delayed overflow reload.

**Warning signs:**
Timer code increments TIMA by elapsed time directly with no selected-bit edge logic.

**Phase to address:**
Phase 3.

---

### Pitfall 5: Test ROMs Exist But Are Not Harnessed

**What goes wrong:**
The repository has strong validation assets but progress remains manual and subjective.

**Why it happens:**
Building a harness feels secondary to implementing emulator features.

**How to avoid:**
Add a minimal automated harness as soon as CPU stepping can run enough instructions to observe pass/fail signals.

**Warning signs:**
Manual notes say a ROM passes, but `cargo test` cannot reproduce it.

**Phase to address:**
Phase 4.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| One large `main.rs` | Fast start | No reusable core, hard tests | Only before Phase 1 begins |
| Giant opcode match without grouped tests | Quick coverage illusion | Hard to isolate broken flags/cycles | Accept only with per-family tests |
| Stub all devices as zero | CPU demos may start | Hidden dependency bugs in real ROMs | Accept briefly if tracked by explicit tests |
| Ignore CGB flags entirely | Simpler DMG scope | Accidental behavior ambiguity | Accept for v1 if documented as DMG-only |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Cartridge header | Reading fields without length checks | Validate minimum length before indexing |
| MBC writes | Treating ROM area as read-only only | Route writes to cartridge mapper state |
| Interrupt registers | Storing `IME` as memory-mapped state | Model `IME` as CPU-internal, with `IE`/`IF` mapped |
| Timer registers | Updating TIMA independently | Use system counter edge behavior |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Over-allocating per instruction | Slow ROM execution | Keep CPU step allocation-free | Once running full frames |
| String-based debug output in hot loop | Huge slowdown | Gate tracing behind debug/config flags | Any real ROM loop |
| Premature dynamic dispatch everywhere | Harder optimization/debugging | Use enums/simple structs first | Larger MBC/device set |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Unchecked ROM indexing | Panic on malformed input | Bounds-check header and mapper reads |
| Unsafe Rust for speed before profiling | Memory safety bugs | Stay safe Rust until measured need exists |
| Trusting path extension as file validity | Invalid files reach parser | Validate bytes and header fields |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Panic on normal user errors | Confusing CLI failures | Return structured errors at CLI boundary |
| No pass/fail reporting for tests | Hard to know progress | Make test harness results visible in `cargo test` |
| UI before core | Pretty shell with no correctness | Make core milestones observable first |

## "Looks Done But Isn't" Checklist

- [ ] **ROM loading:** Often missing binary read and header validation - verify with real `.gb` bytes.
- [ ] **CPU opcodes:** Often missing correct flags/cycles - verify by instruction family.
- [ ] **Timer:** Often missing DIV/TAC edge behavior - verify with mooneye timer tests.
- [ ] **Interrupts:** Often missing delayed `ei` and vector timing - verify with targeted tests.
- [ ] **PPU mode:** Often missing VRAM/OAM access restrictions - verify before rendering-heavy work.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Text ROM loading | LOW | Replace with `fs::read`, add CLI tests |
| Scattered bus logic | MEDIUM | Extract Bus, move all read/write paths behind it |
| Timing added late | HIGH | Refactor CPU step signatures and backfill cycle tests |
| Wrong timer model | HIGH | Rewrite around system counter and mooneye tests |
| No ROM harness | MEDIUM | Add helper runner, start with a small passing subset |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Treating ROMs as text | Phase 1 | Binary load unit/integration test |
| Scattered memory map logic | Phase 1 | Bus read/write tests for core regions |
| Adding timing too late | Phase 2 | CPU step returns cycles for every implemented instruction |
| Incorrect timer model | Phase 3 | Timer register and edge tests |
| Test ROMs not harnessed | Phase 4 | At least one ROM fixture executable through `cargo test` |

## Sources

- https://doc.rust-lang.org/std/fs/fn.read.html - byte-oriented file reads
- https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html - timer edge and overflow behavior
- https://gbdev.io/pandocs/Interrupts.html - interrupt register and `IME` behavior
- https://github.com/Gekkio/mooneye-test-suite - pass/fail reporting and test suite structure
- `.planning/codebase/CONCERNS.md` - current repo-specific risks

---
*Pitfalls research for: Rust DMG Game Boy emulator*
*Researched: 2026-05-02*
