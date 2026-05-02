use crate::cartridge::Cartridge;

#[allow(dead_code)]
pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; 0x2000],
    oam: [u8; 0xA0],
    io: [u8; 0x80],
    hram: [u8; 0x7F],
    interrupt_enable: u8,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: 0,
        }
    }

    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
