pub const CARTRIDGE_ROM_START: u16 = 0x0000;
pub const CARTRIDGE_ROM_END: u16 = 0x7FFF;
pub const CARTRIDGE_ROM_SIZE: usize = 0x8000;

pub const DMG_ENTRY_POINT: u16 = 0x0100;
pub const DMG_STACK_POINTER: u16 = 0xFFFE;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;

pub const EXTERNAL_RAM_START: u16 = 0xA000;
pub const EXTERNAL_RAM_END: u16 = 0xBFFF;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;
pub const WRAM_SIZE: usize = 0x2000;

pub const ECHO_RAM_START: u16 = 0xE000;
pub const ECHO_RAM_END: u16 = 0xFDFF;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
pub const OAM_SIZE: usize = 0xA0;

pub const UNUSABLE_START: u16 = 0xFEA0;
pub const UNUSABLE_END: u16 = 0xFEFF;

pub const IO_REGISTERS_START: u16 = 0xFF00;
pub const IO_REGISTERS_END: u16 = 0xFF7F;
pub const IO_REGISTERS_SIZE: usize = 0x80;

pub const HRAM_START: u16 = 0xFF80;
pub const HRAM_END: u16 = 0xFFFE;
pub const HRAM_SIZE: usize = 0x7F;

pub const INTERRUPT_ENABLE: u16 = 0xFFFF;
