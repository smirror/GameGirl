# Architecture Research

**Domain:** Rust DMG Game Boy emulator
**Researched:** 2026-05-02
**Confidence:** HIGH

## Standard Architecture

### System Overview

```text
+-------------------------------------------------------------+
|                         Host / CLI                           |
|  ROM path, errors, future display/audio/input adapters        |
+------------------------------+------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                       Emulator Core                           |
|  Step loop, clocks, frame/test execution control              |
+------------------------------+------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                            Bus                                |
|  16-bit address routing, memory map, device coordination      |
+----------+-----------+-----------+-----------+---------------+
           |           |           |           |
           v           v           v           v
       Cartridge      CPU        Timer        PPU       Joypad/APU later
       ROM/MBC       regs/op    DIV/TIMA     modes      IO registers
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| CLI | Parse ROM path, load bytes, format top-level errors | Thin `src/main.rs` boundary |
| Cartridge | Own ROM bytes, parse header, expose ROM/RAM/MBC reads/writes | `Cartridge` struct plus mapper enum/trait |
| Bus | Route reads/writes for `0000-FFFF` | `Bus` struct containing cartridge, RAM, I/O devices |
| CPU | Own registers, flags, instruction decode/execute, cycles | `Cpu` struct with step method returning elapsed cycles |
| Timer | Model DIV/TIMA/TMA/TAC and timer interrupt requests | Internal system counter plus I/O register methods |
| Interrupts | Track `IE`, `IF`, `IME`, delayed `ei`, vector dispatch | CPU/bus shared state with explicit service step |
| PPU | Track modes, LY, VRAM/OAM access, future rendering | Start with mode timing before pixels |
| Test Harness | Run unit and ROM-level checks | Cargo tests plus helper runner |

## Recommended Project Structure

```text
src/
|-- main.rs              # CLI shell
|-- lib.rs               # Emulator core public module root
|-- cartridge.rs         # ROM bytes, header, mapper dispatch
|-- bus.rs               # Address map and device read/write routing
|-- cpu/
|   |-- mod.rs           # CPU state and step loop
|   |-- registers.rs     # Register pairs and flags
|   |-- opcodes.rs       # Decode table or opcode dispatch
|   `-- instructions.rs  # Instruction family helpers
|-- timer.rs             # DIV/TIMA/TMA/TAC behavior
|-- interrupts.rs        # IE/IF/IME and vectors
|-- ppu.rs               # PPU mode state, VRAM/OAM restrictions
`-- joypad.rs            # Input register state, later phase
tests/
|-- common/              # Future ROM harness helpers
|-- cartridge.rs
|-- cpu_smoke.rs
`-- timer.rs
```

### Structure Rationale

- **`src/lib.rs`:** Lets tests and future hosts use the emulator core without going through CLI code.
- **`src/main.rs`:** Keeps user-facing argument parsing separate from hardware rules.
- **`src/bus.rs`:** Gives all memory-mapped behavior one home instead of scattering address checks through CPU code.
- **`src/cpu/`:** CPU implementation is large enough to justify submodules from the start.
- **`tests/common/`:** A ROM harness will need shared timeout/pass-fail utilities.

## Architectural Patterns

### Pattern 1: Thin Host, Reusable Core

**What:** Keep CLI/window/audio concerns outside the emulator hardware model.
**When to use:** Immediately.
**Trade-offs:** Slightly more structure up front, but much easier testing.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = std::fs::read(path)?;
    let cart = game_girl::Cartridge::from_bytes(rom)?;
    let mut emu = game_girl::Emulator::new(cart);
    emu.step();
    Ok(())
}
```

### Pattern 2: Bus-Owned Memory Map

**What:** CPU asks the bus for reads/writes; the bus routes to cartridge/RAM/I/O devices.
**When to use:** Before implementing instruction execution.
**Trade-offs:** More explicit device ownership decisions, but prevents CPU from knowing too much.

```rust
let opcode = bus.read8(cpu.pc);
cpu.pc = cpu.pc.wrapping_add(1);
```

### Pattern 3: Cycle-Returning CPU Step

**What:** CPU execution returns elapsed machine cycles or dots so timers/PPU can advance.
**When to use:** As soon as CPU stepping exists.
**Trade-offs:** More bookkeeping per instruction, but avoids bolting timing on later.

## Data Flow

### ROM Execution Flow

```text
CLI path
    -> fs::read bytes
    -> Cartridge::from_bytes
    -> Emulator::new
    -> Cpu::step(&mut Bus)
    -> Bus read/write
    -> Cartridge/RAM/Timer/Interrupts/PPU/Joypad
    -> elapsed cycles advance devices
```

### State Management

```text
Emulator
|-- Cpu
`-- Bus
    |-- Cartridge
    |-- Wram/Hram
    |-- Timer
    |-- InterruptController or IE/IF state
    |-- Ppu
    `-- Joypad later
```

### Key Data Flows

1. **Opcode fetch:** CPU reads `PC` through Bus, so cartridge mapping is honored.
2. **I/O register write:** CPU writes through Bus, which updates timer/PPU/joypad/interrupt state.
3. **Timer tick:** CPU cycles advance system counter; overflow sets timer interrupt request in `IF`.
4. **ROM test result:** Harness observes serial/link/debug-break behavior or another defined pass/fail marker.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| First executable ROM | Dependency-free core, unit tests, simple CLI |
| Many test ROMs | Add ROM harness helpers, timeouts, grouped ignored/slow tests |
| Interactive emulator | Add host display/input/audio adapters outside core |
| Wider compatibility | Add MBC variants, PPU pixel pipeline detail, APU, CGB behind explicit model selection |

### Scaling Priorities

1. **First bottleneck:** Correctness visibility. Fix by adding tests before broad feature work.
2. **Second bottleneck:** Instruction dispatch maintainability. Fix by grouping opcode implementation and tests.
3. **Third bottleneck:** Timing coupling. Fix by making cycle advancement a first-class API.

## Anti-Patterns

### Anti-Pattern 1: CPU Owns the Whole Machine

**What people do:** Put cartridge, RAM, timer, interrupts, and graphics logic inside CPU methods.
**Why it's wrong:** Address routing and device timing become impossible to test independently.
**Do this instead:** CPU owns CPU state; Bus owns memory routing and device access.

### Anti-Pattern 2: Demo-Driven Correctness

**What people do:** Run one simple ROM and treat visible output as correctness.
**Why it's wrong:** Game Boy edge cases often fail only under targeted tests.
**Do this instead:** Make blargg/mooneye milestones part of the roadmap.

### Anti-Pattern 3: Timing After the Fact

**What people do:** Implement opcodes first, then add cycles later.
**Why it's wrong:** Timer, interrupts, PPU access, and HALT behavior depend on timing.
**Do this instead:** Return cycles from instruction execution from the beginning.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| None | n/a | Emulator core should not require network services |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| CLI <-> Core | Construct objects, call APIs, handle `Result` | No hardware logic in CLI |
| CPU <-> Bus | `read8`, `write8`, maybe `read16` helpers | CPU should not bypass Bus |
| Bus <-> Cartridge | Mapper read/write methods | ROM writes may change MBC state |
| CPU/Bus <-> Timer/Interrupts | Cycle advancement and I/O registers | Timer overflow sets `IF` bit |
| Bus <-> PPU | VRAM/OAM reads/writes and mode state | Access restrictions need PPU mode |

## Sources

- https://gbdev.io/pandocs/Memory_Map.html - memory regions, cartridge header, external hardware/MBC notes
- https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html - internal counter and timer overflow behavior
- https://gbdev.io/pandocs/Interrupts.html - `IME`, `IE`, `IF`, delayed `ei`
- `docs/gameboy_architecture_summary.md` - local DMG architecture summary
- `.planning/codebase/ARCHITECTURE.md` - current repo architecture

---
*Architecture research for: Rust DMG Game Boy emulator*
*Researched: 2026-05-02*
