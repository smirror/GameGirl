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

fn rom_check() -> String {
    // check if file path is provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a ROM file")
    }

    // Read Rom file
    args[1].to_string()
}

// ROM file path
fn rom_file_path(file_path: &str) -> String {
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

    // Read Rom data
    let file = rom_file_path(&rom_check());
    log::info!("Loading ROM file: {}", file);

    let rom = load_rom(file);

    // debug: read ROM data
    log::debug!("ROM data: {:?}", rom.get_buf());
}

#[cfg(test)]
fn test_rom_file_path() {
    let file = rom_file_path("roms/hello-world/hello-world.gb");
    assert_eq!(file, "roms/hello-world/hello-world.gb");

    let file = rom_file_path("roms/hello-world/hello-world.gbc");
    assert_eq!(file, "File does not exist");

    let file = rom_file_path("roms/hello-world/hello-world.txt");
    assert_eq!(file, "Invalid file format. Only .gb and .gbc files are supported");
}

#[cfg(test)]
fn test_load_rom() {
    let file = rom_file_path("roms/hello-world/hello-world.gb");
    let rom = load_rom(file);
    assert_eq!(rom.read(0x100), 195);
}