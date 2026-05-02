use crate::cartridge::Cartridge;
use crate::memory_map::{
    CARTRIDGE_ROM_END, CARTRIDGE_ROM_START, ECHO_RAM_END, ECHO_RAM_START, HRAM_END, HRAM_SIZE,
    HRAM_START, INTERRUPT_ENABLE, IO_REGISTERS_END, IO_REGISTERS_SIZE, IO_REGISTERS_START, OAM_END,
    OAM_SIZE, OAM_START, UNUSABLE_END, UNUSABLE_START, WRAM_END, WRAM_SIZE, WRAM_START,
};

pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    oam: [u8; OAM_SIZE],
    io_registers: [u8; IO_REGISTERS_SIZE],
    hram: [u8; HRAM_SIZE],
    interrupt_enable: u8,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io_registers: [0; IO_REGISTERS_SIZE],
            hram: [0; HRAM_SIZE],
            interrupt_enable: 0,
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.read_rom(addr),
            WRAM_START..=WRAM_END => self.wram[wram_index(addr)],
            ECHO_RAM_START..=ECHO_RAM_END => self.wram[echo_wram_index(addr)],
            OAM_START..=OAM_END => self.oam[oam_index(addr)],
            UNUSABLE_START..=UNUSABLE_END => 0xFF,
            IO_REGISTERS_START..=IO_REGISTERS_END => self.io_registers[io_index(addr)],
            HRAM_START..=HRAM_END => self.hram[hram_index(addr)],
            INTERRUPT_ENABLE => self.interrupt_enable,
            _ => 0xFF,
        }
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        match addr {
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.write_rom(addr, value),
            WRAM_START..=WRAM_END => self.wram[wram_index(addr)] = value,
            ECHO_RAM_START..=ECHO_RAM_END => self.wram[echo_wram_index(addr)] = value,
            OAM_START..=OAM_END => self.oam[oam_index(addr)] = value,
            UNUSABLE_START..=UNUSABLE_END => {}
            IO_REGISTERS_START..=IO_REGISTERS_END => self.io_registers[io_index(addr)] = value,
            HRAM_START..=HRAM_END => self.hram[hram_index(addr)] = value,
            INTERRUPT_ENABLE => self.interrupt_enable = value,
            _ => {}
        }
    }
}

fn wram_index(addr: u16) -> usize {
    usize::from(addr - WRAM_START)
}

fn echo_wram_index(addr: u16) -> usize {
    usize::from(addr - ECHO_RAM_START) % WRAM_SIZE
}

fn oam_index(addr: u16) -> usize {
    usize::from(addr - OAM_START)
}

fn io_index(addr: u16) -> usize {
    usize::from(addr - IO_REGISTERS_START)
}

fn hram_index(addr: u16) -> usize {
    usize::from(addr - HRAM_START)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bus() -> Bus {
        let mut rom = vec![0; crate::memory_map::CARTRIDGE_ROM_SIZE];
        rom[usize::from(CARTRIDGE_ROM_START)] = 0xAA;
        rom[usize::from(CARTRIDGE_ROM_END)] = 0xBB;
        Bus::new(Cartridge::from_bytes(rom).expect("valid ROM-only cartridge"))
    }

    #[test]
    fn delegates_rom_reads_and_writes_to_cartridge() {
        let mut bus = bus();

        assert_eq!(bus.read8(CARTRIDGE_ROM_START), 0xAA);
        assert_eq!(bus.read8(CARTRIDGE_ROM_END), 0xBB);
        bus.write8(CARTRIDGE_ROM_START, 0x99);
        assert_eq!(bus.read8(CARTRIDGE_ROM_START), 0xAA);
    }

    #[test]
    fn mirrors_wram_and_echo_ram() {
        let mut bus = bus();

        bus.write8(WRAM_START, 0x12);
        assert_eq!(bus.read8(0xE000), 0x12);

        bus.write8(0xE000, 0x34);
        assert_eq!(bus.read8(WRAM_START), 0x34);
    }

    #[test]
    fn unusable_range_reads_ff_and_ignores_writes() {
        let mut bus = bus();

        bus.write8(0xFEA0, 0x12);
        bus.write8(0xFEFF, 0x34);

        assert_eq!(bus.read8(0xFEA0), 0xFF);
        assert_eq!(bus.read8(0xFEFF), 0xFF);
    }

    #[test]
    fn oam_placeholder_range_is_writable() {
        let mut bus = bus();

        bus.write8(0xFE00, 0x12);
        bus.write8(0xFE9F, 0x34);

        assert_eq!(bus.read8(0xFE00), 0x12);
        assert_eq!(bus.read8(0xFE9F), 0x34);
    }

    #[test]
    fn io_hram_and_ie_are_writable() {
        let mut bus = bus();

        bus.write8(IO_REGISTERS_START, 0x12);
        bus.write8(0xFF80, 0x34);
        bus.write8(INTERRUPT_ENABLE, 0x56);

        assert_eq!(bus.read8(IO_REGISTERS_START), 0x12);
        assert_eq!(bus.read8(0xFF80), 0x34);
        assert_eq!(bus.read8(INTERRUPT_ENABLE), 0x56);
    }

    #[test]
    fn unmapped_ranges_read_ff_and_ignore_writes() {
        let mut bus = bus();

        bus.write8(crate::memory_map::VRAM_START, 0x12);
        bus.write8(0xA000, 0x34);

        assert_eq!(bus.read8(crate::memory_map::VRAM_START), 0xFF);
        assert_eq!(bus.read8(0xA000), 0xFF);
    }
}
