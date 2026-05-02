use std::io;
use std::ops::Range;
use std::path::{Path, PathBuf};

pub const MIN_CARTRIDGE_HEADER_LEN: usize = 0x150;
const ENTRY_POINT_RANGE: Range<usize> = 0x0100..0x0104;
const LOGO_RANGE: Range<usize> = 0x0104..0x0134;
const TITLE_RANGE: Range<usize> = 0x0134..0x0144;
const HEADER_RANGE: Range<usize> = 0x0100..0x0150;
const CGB_FLAG_OFFSET: usize = 0x0143;
const CARTRIDGE_TYPE_OFFSET: usize = 0x0147;
const ROM_SIZE_OFFSET: usize = 0x0148;
const RAM_SIZE_OFFSET: usize = 0x0149;
const HEADER_CHECKSUM_OFFSET: usize = 0x014D;
const GLOBAL_CHECKSUM_RANGE: Range<usize> = 0x014E..0x0150;

#[derive(Debug)]
pub struct Cartridge {
    rom: Vec<u8>,
    pub header: CartridgeHeader,
}

impl Cartridge {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, CartridgeError> {
        let header = CartridgeHeader::parse(&bytes)?;

        if let CartridgeType::Unsupported(code) = header.cartridge_type {
            return Err(CartridgeError::UnsupportedCartridgeType(code));
        }

        Ok(Self { rom: bytes, header })
    }

    pub fn rom_len(&self) -> usize {
        self.rom.len()
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        let address = usize::from(address);

        if address > 0x7FFF {
            return 0xFF;
        }

        self.rom.get(address).copied().unwrap_or(0xFF)
    }

    pub fn write_rom(&mut self, _address: u16, _value: u8) {
        // MBC register writes are deferred until bank-controller behavior is modeled.
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartridgeHeader {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub cartridge_type_code: u8,
    pub rom_size: usize,
    pub rom_size_code: u8,
    pub ram_size: usize,
    pub ram_size_code: u8,
    pub entry_point: [u8; 4],
    pub logo: [u8; 0x30],
    pub header_bytes: [u8; 0x50],
    pub cgb_flag: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

impl CartridgeHeader {
    pub fn parse(bytes: &[u8]) -> Result<Self, CartridgeError> {
        validate_rom_bytes(bytes)?;

        let cartridge_type_code = bytes[CARTRIDGE_TYPE_OFFSET];
        let rom_size_code = bytes[ROM_SIZE_OFFSET];
        let ram_size_code = bytes[RAM_SIZE_OFFSET];
        let global_checksum = u16::from_be_bytes(slice_to_array(bytes, GLOBAL_CHECKSUM_RANGE));

        Ok(Self {
            title: parse_title(bytes),
            cartridge_type: CartridgeType::from_code(cartridge_type_code),
            cartridge_type_code,
            rom_size: rom_size_from_code(rom_size_code)?,
            rom_size_code,
            ram_size: ram_size_from_code(ram_size_code)?,
            ram_size_code,
            entry_point: slice_to_array(bytes, ENTRY_POINT_RANGE),
            logo: slice_to_array(bytes, LOGO_RANGE),
            header_bytes: slice_to_array(bytes, HEADER_RANGE),
            cgb_flag: bytes[CGB_FLAG_OFFSET],
            header_checksum: bytes[HEADER_CHECKSUM_OFFSET],
            global_checksum,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    Huc3,
    Huc1RamBattery,
    Unsupported(u8),
}

impl CartridgeType {
    fn from_code(code: u8) -> Self {
        match code {
            0x00 => Self::RomOnly,
            0x01 => Self::Mbc1,
            0x02 => Self::Mbc1Ram,
            0x03 => Self::Mbc1RamBattery,
            0x05 => Self::Mbc2,
            0x06 => Self::Mbc2Battery,
            0x08 => Self::RomRam,
            0x09 => Self::RomRamBattery,
            0x0B => Self::Mmm01,
            0x0C => Self::Mmm01Ram,
            0x0D => Self::Mmm01RamBattery,
            0x0F => Self::Mbc3TimerBattery,
            0x10 => Self::Mbc3TimerRamBattery,
            0x11 => Self::Mbc3,
            0x12 => Self::Mbc3Ram,
            0x13 => Self::Mbc3RamBattery,
            0x19 => Self::Mbc5,
            0x1A => Self::Mbc5Ram,
            0x1B => Self::Mbc5RamBattery,
            0x1C => Self::Mbc5Rumble,
            0x1D => Self::Mbc5RumbleRam,
            0x1E => Self::Mbc5RumbleRamBattery,
            0x20 => Self::Mbc6,
            0x22 => Self::Mbc7SensorRumbleRamBattery,
            0xFC => Self::PocketCamera,
            0xFD => Self::BandaiTama5,
            0xFE => Self::Huc3,
            0xFF => Self::Huc1RamBattery,
            other => Self::Unsupported(other),
        }
    }
}

#[derive(Debug)]
pub enum CartridgeError {
    Io { path: PathBuf, source: io::Error },
    TooShort { len: usize },
    UnsupportedCartridgeType(u8),
    UnsupportedRomSize(u8),
    UnsupportedRamSize(u8),
}

impl std::fmt::Display for CartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(f, "could not read ROM '{}': {source}", path.display())
            }
            Self::TooShort { len } => write!(
                f,
                "ROM is too short: {len} bytes read, expected at least {MIN_CARTRIDGE_HEADER_LEN} bytes"
            ),
            Self::UnsupportedCartridgeType(code) => {
                write!(f, "unsupported cartridge type: 0x{code:02X}")
            }
            Self::UnsupportedRomSize(code) => write!(f, "unsupported ROM size code: 0x{code:02X}"),
            Self::UnsupportedRamSize(code) => write!(f, "unsupported RAM size code: 0x{code:02X}"),
        }
    }
}

impl std::error::Error for CartridgeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn validate_rom_bytes(bytes: &[u8]) -> Result<(), CartridgeError> {
    if bytes.len() < MIN_CARTRIDGE_HEADER_LEN {
        return Err(CartridgeError::TooShort { len: bytes.len() });
    }

    Ok(())
}

fn parse_title(bytes: &[u8]) -> String {
    let title_bytes = &bytes[TITLE_RANGE];
    let title_len = title_bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(title_bytes.len());

    String::from_utf8_lossy(&title_bytes[..title_len]).to_string()
}

fn rom_size_from_code(code: u8) -> Result<usize, CartridgeError> {
    match code {
        0x00 => Ok(32 * 1024),
        0x01 => Ok(64 * 1024),
        0x02 => Ok(128 * 1024),
        0x03 => Ok(256 * 1024),
        0x04 => Ok(512 * 1024),
        0x05 => Ok(1024 * 1024),
        0x06 => Ok(2 * 1024 * 1024),
        0x07 => Ok(4 * 1024 * 1024),
        0x08 => Ok(8 * 1024 * 1024),
        other => Err(CartridgeError::UnsupportedRomSize(other)),
    }
}

fn ram_size_from_code(code: u8) -> Result<usize, CartridgeError> {
    match code {
        0x00 => Ok(0),
        0x01 => Ok(2 * 1024),
        0x02 => Ok(8 * 1024),
        0x03 => Ok(32 * 1024),
        0x04 => Ok(128 * 1024),
        0x05 => Ok(64 * 1024),
        other => Err(CartridgeError::UnsupportedRamSize(other)),
    }
}

fn slice_to_array<const N: usize>(bytes: &[u8], range: Range<usize>) -> [u8; N] {
    bytes[range]
        .try_into()
        .expect("header range length mismatch")
}

pub fn load_rom_file(path: impl AsRef<Path>) -> Result<Vec<u8>, CartridgeError> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|source| CartridgeError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    Cartridge::from_bytes(bytes.clone())?;
    Ok(bytes)
}

pub fn load_cartridge_file(path: impl AsRef<Path>) -> Result<Cartridge, CartridgeError> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|source| CartridgeError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    Cartridge::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rom_bytes() -> Vec<u8> {
        vec![0; MIN_CARTRIDGE_HEADER_LEN]
    }

    fn full_rom_bytes() -> Vec<u8> {
        vec![0; 0x8000]
    }

    #[test]
    fn rejects_rom_shorter_than_header() {
        let bytes = vec![0; MIN_CARTRIDGE_HEADER_LEN - 1];
        let result = validate_rom_bytes(&bytes);

        match result {
            Err(CartridgeError::TooShort { len }) => {
                assert_eq!(len, MIN_CARTRIDGE_HEADER_LEN - 1);
            }
            other => panic!("expected too-short error, got {other:?}"),
        }
    }

    #[test]
    fn accepts_rom_with_complete_header_region() {
        let bytes = rom_bytes();

        assert!(validate_rom_bytes(&bytes).is_ok());
    }

    #[test]
    fn too_short_error_display_is_human_readable() {
        let error = CartridgeError::TooShort { len: 3 };

        assert_eq!(
            error.to_string(),
            "ROM is too short: 3 bytes read, expected at least 336 bytes"
        );
    }

    #[test]
    fn constructs_rom_only_cartridge() {
        let cartridge = Cartridge::from_bytes(rom_bytes()).expect("valid ROM-only cartridge");

        assert_eq!(cartridge.header.cartridge_type, CartridgeType::RomOnly);
        assert_eq!(cartridge.header.rom_size, 32 * 1024);
        assert_eq!(cartridge.header.ram_size, 0);
        assert_eq!(cartridge.rom_len(), MIN_CARTRIDGE_HEADER_LEN);
    }

    #[test]
    fn rejects_unsupported_cartridge_type() {
        let mut bytes = rom_bytes();
        bytes[CARTRIDGE_TYPE_OFFSET] = 0x04;

        match Cartridge::from_bytes(bytes) {
            Err(CartridgeError::UnsupportedCartridgeType(0x04)) => {}
            other => panic!("expected unsupported cartridge type, got {other:?}"),
        }
    }

    #[test]
    fn recognizes_known_mbc_cartridge_types() {
        let mut bytes = rom_bytes();
        bytes[CARTRIDGE_TYPE_OFFSET] = 0x03;
        bytes[RAM_SIZE_OFFSET] = 0x02;

        let cartridge = Cartridge::from_bytes(bytes).expect("known MBC cartridge type");

        assert_eq!(
            cartridge.header.cartridge_type,
            CartridgeType::Mbc1RamBattery
        );
        assert_eq!(cartridge.header.cartridge_type_code, 0x03);
        assert_eq!(cartridge.header.ram_size, 8 * 1024);
    }

    #[test]
    fn rejects_unsupported_rom_size_code() {
        let mut bytes = rom_bytes();
        bytes[ROM_SIZE_OFFSET] = 0xFF;

        match Cartridge::from_bytes(bytes) {
            Err(CartridgeError::UnsupportedRomSize(0xFF)) => {}
            other => panic!("expected unsupported ROM size, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unsupported_ram_size_code() {
        let mut bytes = rom_bytes();
        bytes[RAM_SIZE_OFFSET] = 0xFF;

        match Cartridge::from_bytes(bytes) {
            Err(CartridgeError::UnsupportedRamSize(0xFF)) => {}
            other => panic!("expected unsupported RAM size, got {other:?}"),
        }
    }

    #[test]
    fn parses_header_title_entry_and_size_metadata() {
        let mut bytes = rom_bytes();
        bytes[ENTRY_POINT_RANGE].copy_from_slice(&[0x00, 0xC3, 0x50, 0x01]);
        bytes[0x0134..0x013C].copy_from_slice(b"GAMEGIRL");
        bytes[ROM_SIZE_OFFSET] = 0x02;
        bytes[RAM_SIZE_OFFSET] = 0x03;
        bytes[HEADER_CHECKSUM_OFFSET] = 0x42;
        bytes[GLOBAL_CHECKSUM_RANGE].copy_from_slice(&[0x12, 0x34]);

        let cartridge = Cartridge::from_bytes(bytes).expect("valid ROM-only cartridge");

        assert_eq!(cartridge.header.title, "GAMEGIRL");
        assert_eq!(cartridge.header.cartridge_type, CartridgeType::RomOnly);
        assert_eq!(cartridge.header.cartridge_type_code, 0x00);
        assert_eq!(cartridge.header.rom_size_code, 0x02);
        assert_eq!(cartridge.header.rom_size, 128 * 1024);
        assert_eq!(cartridge.header.ram_size_code, 0x03);
        assert_eq!(cartridge.header.ram_size, 32 * 1024);
        assert_eq!(cartridge.header.entry_point, [0x00, 0xC3, 0x50, 0x01]);
        assert_eq!(cartridge.header.header_checksum, 0x42);
        assert_eq!(cartridge.header.global_checksum, 0x1234);
    }

    #[test]
    fn rom_only_reads_fixed_rom_range() {
        let mut bytes = full_rom_bytes();
        bytes[0x0000] = 0xAA;
        bytes[0x7FFF] = 0xBB;
        let cartridge = Cartridge::from_bytes(bytes).expect("valid ROM-only cartridge");

        assert_eq!(cartridge.read_rom(0x0000), 0xAA);
        assert_eq!(cartridge.read_rom(0x7FFF), 0xBB);
    }

    #[test]
    fn rom_only_reads_missing_fixture_bytes_as_ff() {
        let cartridge = Cartridge::from_bytes(rom_bytes()).expect("valid ROM-only cartridge");

        assert_eq!(cartridge.read_rom(0x4000), 0xFF);
    }

    #[test]
    fn rom_only_writes_do_not_mutate_rom() {
        let mut bytes = full_rom_bytes();
        bytes[0x0000] = 0xAA;
        let mut cartridge = Cartridge::from_bytes(bytes).expect("valid ROM-only cartridge");

        cartridge.write_rom(0x0000, 0x99);

        assert_eq!(cartridge.read_rom(0x0000), 0xAA);
    }
}
