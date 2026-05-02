# Feature Research

**Domain:** Rust DMG Game Boy emulator
**Researched:** 2026-05-02
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Binary ROM loading | Emulator must consume `.gb` bytes, not text | LOW | Replace `read_to_string` with `fs::read`; parse header from `0x0100-0x014F` |
| Cartridge header parsing | Needed to know title, ROM size, RAM size, and mapper type | MEDIUM | Pan Docs places cartridge header in the first ROM bank |
| DMG memory map and bus | CPU-visible devices are selected by address | MEDIUM | Required before CPU can execute real ROM reads/writes |
| CPU registers and fetch/decode/execute | Core of every emulator | HIGH | Start with register model and a small instruction subset |
| CPU flags and stack behavior | Required for control flow and arithmetic correctness | HIGH | Small mistakes cascade into test failures |
| Timer and interrupts | Many games and tests rely on timing and `IF`/`IE`/`IME` behavior | HIGH | Use internal counter model from the start |
| ROM test harness | Existing fixtures need automated pass/fail checks | MEDIUM | Start with targeted unit tests, then ROM-level harness |

### Differentiators (Competitive Advantage)

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Test-ROM-driven milestones | Makes progress measurable and prevents demo-only correctness | MEDIUM | Use local blargg/mooneye assets as phase gates |
| Clear hardware module boundaries | Makes future timing fixes safer | MEDIUM | Bus, CPU, cartridge, timer, interrupts, PPU, joypad |
| DMG-first precision notes | Avoids mixing DMG/CGB behavior accidentally | LOW | Keep CGB decisions out of v1 requirements |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Full UI first | It feels like a visible emulator quickly | Encourages drawing before execution correctness exists | Build core and tests first, add host UI later |
| Implement all instructions in one giant pass | Seems faster than phased work | Hard to debug flags, cycles, and decode issues | Implement and test instruction families incrementally |
| CGB support from day one | More complete emulator story | Adds banked VRAM/WRAM, palettes, speed switching, and model differences too early | Keep architecture extensible but scope v1 to DMG |
| Audio early | Fun and visible | APU timing is its own complex subsystem | Defer until CPU/bus/timer/PPU foundations are stable |

## Feature Dependencies

```text
Binary ROM loading
    requires -> Cartridge header parsing
        requires -> Bus memory mapping
            requires -> CPU fetch/decode/execute
                requires -> Flags, stack, jumps, calls
                    enhances -> ROM test harness

Timer and interrupt state
    requires -> Bus I/O registers
    enhances -> CPU test ROM compatibility

PPU mode timing
    requires -> Bus memory mapping
    enhances -> VRAM/OAM access correctness
```

### Dependency Notes

- **ROM loading requires header parsing:** The emulator needs cartridge metadata before it can choose mapper behavior.
- **Bus requires cartridge and memory regions:** CPU execution must read opcodes from the cartridge address space and write device registers through one boundary.
- **Timer/interrupts require bus I/O registers:** `IF`, `IE`, `DIV`, `TIMA`, `TMA`, and `TAC` are memory-mapped.
- **ROM harness depends on execution loop:** Test ROMs become useful once the CPU can step and expose pass/fail signals.

## MVP Definition

### Launch With (v1)

- [ ] Binary ROM byte loading and basic cartridge header parsing - real ROM input starts here
- [ ] DMG bus and memory map skeleton - all hardware access needs one route
- [ ] CPU registers and initial instruction execution loop - enables first executable behavior
- [ ] Focused CPU instruction groups with unit tests - prevents a fragile giant opcode table
- [ ] Timer/interrupt registers and initial timing model - required for meaningful tests
- [ ] ROM test harness foundation - makes correctness observable

### Add After Validation (v1.x)

- [ ] PPU mode timing and VRAM/OAM restrictions - needed before rendering correctness
- [ ] Background rendering - first visible output path
- [ ] Joypad input - required for interactive games
- [ ] MBC1/MBC3/MBC5 expansion - needed for wider ROM compatibility

### Future Consideration (v2+)

- [ ] CGB support - defer until DMG path is stable
- [ ] APU audio output - defer until core timing is stable
- [ ] Desktop UI polish - defer until emulator core can run target ROMs

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Binary ROM loading | HIGH | LOW | P1 |
| Cartridge header parsing | HIGH | MEDIUM | P1 |
| Bus/memory map | HIGH | MEDIUM | P1 |
| CPU registers and fetch/decode/execute | HIGH | HIGH | P1 |
| CPU instruction families | HIGH | HIGH | P1 |
| Timer and interrupts | HIGH | HIGH | P1 |
| ROM harness | HIGH | MEDIUM | P1 |
| PPU rendering | MEDIUM | HIGH | P2 |
| Joypad | MEDIUM | MEDIUM | P2 |
| APU | MEDIUM | HIGH | P3 |
| CGB | MEDIUM | HIGH | P3 |

## Competitor Feature Analysis

| Feature | Reference Emulators | Test Suites | Our Approach |
|---------|---------------------|-------------|--------------|
| CPU correctness | Mature emulators implement instruction families and flags carefully | blargg CPU tests | Build small, tested instruction groups |
| Timing correctness | Mature emulators model timer/interrupt quirks explicitly | mooneye acceptance tests | Use internal counter and phase gates |
| Cartridge support | Mature emulators support many MBCs | mooneye emulator-only MBC tests | Start with simplest cartridge path, expand after bus is stable |

## Sources

- https://gbdev.io/pandocs/Memory_Map.html - cartridge header and memory map expectations
- https://gbdev.io/pandocs/Interrupts.html - `IME`, `IE`, `IF`, and delayed `ei`
- https://github.com/Gekkio/mooneye-test-suite - suite structure and pass/fail reporting
- `docs/hot_to_proceed.md` - local implementation roadmap
- `.planning/codebase/CONCERNS.md` - current gaps and risks

---
*Feature research for: Rust DMG Game Boy emulator*
*Researched: 2026-05-02*
