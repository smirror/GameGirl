# v1.1 Architecture Research: Hardware-Accurate Core Architecture

## Direction

The current `Bus` is a useful foundation, but v1.1 should evolve it from "memory owner" into "address router." Real DMG memory reads/writes interact with cartridge mapper state, boot ROM overlay, timer/divider, serial transfer, joypad selection, DMA, PPU mode access, interrupt registers, and ordinary RAM. Those responsibilities should not collapse into one anonymous I/O byte array.

## Proposed Ownership Model

```rust
pub struct Machine {
    model: Model,
    boot_mode: BootMode,
    cpu: Cpu,
    cartridge: Cartridge,
    wram: Wram,
    vram: Vram,
    oam: Oam,
    hram: [u8; 0x7f],
    joypad: Joypad,
    serial: Serial,
    timer: Timer,
    dma: DmaController,
    ppu: Ppu,
    interrupts: Interrupts,
}
```

`Machine` owns devices. `Bus` or `Interconnect` routes CPU-visible addresses to the owning device and applies side effects.

## Mapper Boundary

Recommended shape:

```rust
pub trait Mapper {
    fn read_rom(&self, addr: u16) -> u8;
    fn write_ctrl(&mut self, addr: u16, value: u8);
    fn read_eram(&self, addr: u16) -> u8;
    fn write_eram(&mut self, addr: u16, value: u8);
}
```

`Cartridge` should own a mapper implementation plus parsed header metadata. CPU and Bus should not know whether a cartridge is NoMbc, MBC1, MBC3, or MBC5.

## Execution vs Inspection

Expose two read styles:

- `read8` / `write8`: execution path, allowed to trigger device side effects.
- `peek8`: inspection path for tests, debugger, logger, or disassembler; must avoid side effects.

This distinction prevents future debug tools from accidentally changing MMIO state.

## Boot Mapping

Boot mapping should be a first-class policy:

- `SkipBootRom`: initialize post-boot CPU/MMIO state and start at `0x0100`.
- `UseBootRom`: start at `0x0000`, route low ROM reads through boot ROM while mapped, and unmap on `FF50`.

Do not combine post-boot CPU defaults with active boot ROM execution.

## M-Cycle Stepping

CPU instructions can continue returning `StepResult.machine_cycles`, but a machine-level API should consume those cycles:

```rust
pub fn step_instruction(&mut self) -> Result<StepResult, EmuError> {
    let result = self.cpu.step(&mut self.interconnect())?;
    for _ in 0..result.machine_cycles {
        self.tick_mcycle();
    }
    Ok(result)
}
```

This gives timer, serial, DMA, PPU, and interrupt hooks a stable timing surface before exact edge cases are implemented.

## Suggested Build Order

1. Header/mode/load policy and mapper trait.
2. Full address-space route including VRAM and external RAM.
3. Device-backed MMIO for key registers.
4. Boot mode and `FF50`.
5. M-cycle machine stepping.
6. MBC1, then MBC3/MBC5.
7. Capability-gated ROM harness.
