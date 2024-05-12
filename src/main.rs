use std::env;
use std::fs;

// mod cpu;
// mod bus;
// mod timer;
// mod ram;
mod rom;
// mod ppu;
// mod joypad;

// struct GamegirlSystem {
//     cpu: cpu::CPU,
//     bus: bus::Bus,
//     v_ram: ram::RAM,
//     w_ram: ram::RAM,
//     h_ram: ram::RAM,
//     oam_ram: ram::RAM,
//     // ppu: ppu::PPU,
//     timer: timer,
// }

// ROM file path
fn check_rom_file_path(file_path: &str) -> String {
    // check if file exists
    if !fs::metadata(&file_path).is_ok() {
        panic!("File does not exist")
    }

    // check if file is a .gb or .gbc file
    if !file_path.ends_with(".gb") && !file_path.ends_with(".gbc") {
        panic!("Invalid file format. Only .gb and .gbc files are supported")
    }

    file_path.to_string()
}

// load ROM file
fn load_rom(file: String) -> rom::ROM {
    // read file
    let data = fs::read(file).expect("Unable to read file");
    rom::ROM::new(data)
}

fn main() {
    // Set the log level to debug
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // check if file path is provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a ROM file")
    }

    // Read Rom file
    let file_path = &args[1];

    // load Rom data
    let file = check_rom_file_path(file_path);
    log::info!("Loading ROM file: {}", file);

    let rom = load_rom(file);

    // debug: read ROM data
    log::debug!("ROM data: {:?}", rom.get_buf());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom_file_path_returns_provided_file_path() {
        let file_path = "roms/hello-world/hello-world.gb";
        let result = check_rom_file_path(file_path);
        assert_eq!(result, file_path);
    }

    #[test]
    #[should_panic(expected = "File does not exist")]
    fn rom_file_path_panics_when_file_does_not_exist() {
        let file_path = "nonexistent.gb";
        check_rom_file_path(file_path);
    }

    #[test]
    #[should_panic(expected = "Invalid file format. Only .gb and .gbc files are supported")]
    fn rom_file_path_panics_when_file_is_not_gb_or_gbc() {
        let file_path = "src/main.rs";
        check_rom_file_path(file_path);
    }

    #[test]
    fn load_rom_returns_rom_with_correct_data() {
        let file = "roms/hello-world/hello-world.gb";
        let rom = load_rom(file.to_string());
        assert_eq!(rom.read(0x100), 195);
    }
}