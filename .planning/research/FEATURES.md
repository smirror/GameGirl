# v1.1 Feature Research: Hardware-Accurate Core Architecture

## Table Stakes

### Cartridge And Header Policy

- One load path should construct a validated `Cartridge`, not split raw-byte and cartridge construction paths.
- Header parsing should distinguish hard validation from metadata:
  - Header checksum is boot-critical when modeling real boot behavior.
  - Global checksum is metadata/warning, not a normal boot blocker.
- CGB flag handling should use header byte `0x0143`; file extension must not choose hardware mode.
- CGB-only ROMs should be rejected while GameGirl is DMG-only.

### Mapper Boundary

- `0000-7FFF` and `A000-BFFF` are cartridge-owned address ranges.
- Writes to `0000-7FFF` are mapper control writes for MBC cartridges.
- External RAM reads/writes must go through the mapper.
- NoMbc should be implemented first; MBC1, MBC3, and MBC5 should follow as explicit controller implementations.

### Full Address-Space Routing

- Bus/interconnect must route the full DMG CPU-visible address space:
  - Cartridge ROM/control: `0000-7FFF`
  - VRAM: `8000-9FFF`
  - External cartridge RAM/RTC window: `A000-BFFF`
  - WRAM and Echo RAM: `C000-FDFF`
  - OAM and not-usable range: `FE00-FEFF`
  - I/O registers: `FF00-FF7F`
  - HRAM and IE: `FF80-FFFF`

### Side-Effect-Aware MMIO

Key I/O registers need device handlers rather than plain byte storage:

- `FF00` joypad active-low matrix behavior.
- `FF01/FF02` serial transfer request/completion behavior and test-output capture.
- `FF04-FF07` timer/divider registers and write side effects.
- `FF0F/FFFF` interrupt request/enable bits.
- `FF46` OAM DMA start/progress behavior.
- `FF50` boot ROM lock/unmap behavior.

### Boot Policy

- `SkipBootRom` should initialize CPU and MMIO to documented post-boot state.
- `UseBootRom` should start at `0x0000`, map boot ROM over cartridge reads, and unmap via `FF50`.
- Boot policy must be explicit in API/tests so post-boot defaults are not mixed with boot-ROM execution.

### M-Cycle Progression

- CPU instructions already return machine cycles; v1.1 should consume those cycles through a machine/interconnect tick loop.
- Timer, serial, DMA, PPU mode skeleton, and interrupt hooks should advance from M-cycles rather than from ad hoc instruction counters.

### Validation Harness

- ROM harness should classify pass/fail/timeout deterministically.
- Serial output should be usable as an early test ROM signal.
- Test ROMs should be capability-gated so unsupported hardware is not mistaken for emulator regression.

## Differentiators

- Side-effect-free `peek8` for disassembly/debugging that cannot trigger MMIO actions.
- Address-space tests that assert behavior by hardware region rather than by storage array implementation.
- Mapper tests that document each controller's bank-zero and RAM-enable quirks as they are added.

## Deferred

- Full CGB execution.
- Pixel rendering.
- Host UI.
- APU/audio output.
- RTC wall-clock persistence.
- Battery save persistence.
- Exotic cartridge hardware.
