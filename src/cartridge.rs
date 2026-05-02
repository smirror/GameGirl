use std::io;
use std::path::{Path, PathBuf};

pub const MIN_CARTRIDGE_HEADER_LEN: usize = 0x150;

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

pub fn load_rom_file(path: impl AsRef<Path>) -> Result<Vec<u8>, CartridgeError> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|source| CartridgeError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    validate_rom_bytes(&bytes)?;
    Ok(bytes)
}
