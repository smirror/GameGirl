use std::env;
use std::path::Path;
use std::process::ExitCode;

use game_girl::cartridge::{Cartridge, CartridgeHeader};

fn main() -> ExitCode {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "game_girl".to_string());

    let Some(file_path) = args.next() else {
        eprintln!("Usage: {program} <rom.gb|rom.gbc>");
        return ExitCode::FAILURE;
    };

    if !is_supported_rom_path(&file_path) {
        eprintln!("File must be a .gb or .gbc file");
        return ExitCode::FAILURE;
    }

    let bytes = match std::fs::read(&file_path) {
        Ok(bytes) => {
            println!("Loaded ROM: {} bytes", bytes.len());
            bytes
        }
        Err(error) => {
            eprintln!("could not read ROM '{file_path}': {error}");
            return ExitCode::FAILURE;
        }
    };

    let header = match CartridgeHeader::parse(&bytes) {
        Ok(header) => header,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    print_header(&header);

    match Cartridge::from_bytes(bytes) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn is_supported_rom_path(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("gb") || extension.eq_ignore_ascii_case("gbc")
        })
}

fn print_header(header: &CartridgeHeader) {
    println!("Header:");
    println!("  title: {}", header.title);
    println!(
        "  cartridge_type: {:?} (0x{:02X})",
        header.cartridge_type, header.cartridge_type_code
    );
    println!(
        "  rom_size: {} bytes (code 0x{:02X})",
        header.rom_size, header.rom_size_code
    );
    println!(
        "  ram_size: {} bytes (code 0x{:02X})",
        header.ram_size, header.ram_size_code
    );
    println!("  cgb_flag: 0x{:02X}", header.cgb_flag);
    println!("  header_checksum: 0x{:02X}", header.header_checksum);
    println!("  global_checksum: 0x{:04X}", header.global_checksum);
    println!("  entry_point: {}", format_bytes(&header.entry_point));
}

fn format_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}
