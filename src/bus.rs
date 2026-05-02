use crate::cartridge::Cartridge;

const WRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0xA0;
const IO_SIZE: usize = 0x80;
const HRAM_SIZE: usize = 0x7F;

pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    oam: [u8; OAM_SIZE],
    io_registers: [u8; IO_SIZE],
    hram: [u8; HRAM_SIZE],
    interrupt_enable: u8,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io_registers: [0; IO_SIZE],
            hram: [0; HRAM_SIZE],
            interrupt_enable: 0,
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read_rom(addr),
            0xC000..=0xDFFF => self.wram[wram_index(addr)],
            0xE000..=0xFDFF => self.wram[echo_wram_index(addr)],
            0xFE00..=0xFE9F => self.oam[oam_index(addr)],
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00..=0xFF7F => self.io_registers[io_index(addr)],
            0xFF80..=0xFFFE => self.hram[hram_index(addr)],
            0xFFFF => self.interrupt_enable,
            _ => 0xFF,
        }
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => self.cartridge.write_rom(addr, value),
            0xC000..=0xDFFF => self.wram[wram_index(addr)] = value,
            0xE000..=0xFDFF => self.wram[echo_wram_index(addr)] = value,
            0xFE00..=0xFE9F => self.oam[oam_index(addr)] = value,
            0xFEA0..=0xFEFF => {}
            0xFF00..=0xFF7F => self.io_registers[io_index(addr)] = value,
            0xFF80..=0xFFFE => self.hram[hram_index(addr)] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => {}
        }
    }
}

fn wram_index(addr: u16) -> usize {
    usize::from(addr - 0xC000)
}

fn echo_wram_index(addr: u16) -> usize {
    usize::from(addr - 0xE000) % WRAM_SIZE
}

fn oam_index(addr: u16) -> usize {
    usize::from(addr - 0xFE00)
}

fn io_index(addr: u16) -> usize {
    usize::from(addr - 0xFF00)
}

fn hram_index(addr: u16) -> usize {
    usize::from(addr - 0xFF80)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bus() -> Bus {
        let mut rom = vec![0; 0x8000];
        rom[0x0000] = 0xAA;
        rom[0x7FFF] = 0xBB;
        Bus::new(Cartridge::from_bytes(rom).expect("valid ROM-only cartridge"))
    }

    #[test]
    fn delegates_rom_reads_and_writes_to_cartridge() {
        let mut bus = bus();

        assert_eq!(bus.read8(0x0000), 0xAA);
        bus.write8(0x0000, 0x99);
        assert_eq!(bus.read8(0x0000), 0xAA);
    }

    #[test]
    fn mirrors_wram_and_echo_ram() {
        let mut bus = bus();

        bus.write8(0xC000, 0x12);
        assert_eq!(bus.read8(0xE000), 0x12);

        bus.write8(0xE000, 0x34);
        assert_eq!(bus.read8(0xC000), 0x34);
    }

    #[test]
    fn unusable_range_reads_ff_and_ignores_writes() {
        let mut bus = bus();

        bus.write8(0xFEA0, 0x12);

        assert_eq!(bus.read8(0xFEA0), 0xFF);
    }

    #[test]
    fn io_hram_and_ie_are_writable() {
        let mut bus = bus();

        bus.write8(0xFF00, 0x12);
        bus.write8(0xFF80, 0x34);
        bus.write8(0xFFFF, 0x56);

        assert_eq!(bus.read8(0xFF00), 0x12);
        assert_eq!(bus.read8(0xFF80), 0x34);
        assert_eq!(bus.read8(0xFFFF), 0x56);
    }
}
