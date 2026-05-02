use std::env;
use std::path::Path;
use std::process::ExitCode;

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

    match game_girl::cartridge::load_rom_file(&file_path) {
        Ok(bytes) => {
            println!("Loaded ROM: {} bytes", bytes.len());
            ExitCode::SUCCESS
        }
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
