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
fn rom_file_path(file_path: &str) -> String {
    // check if file is a .gb or .gbc file
    if !file_path.ends_with(".gb") && !file_path.ends_with(".gbc") {
        panic!("Invalid file format. Only .gb and .gbc files are supported")
    }

    file_path.to_string()
}

// load ROM file
fn load_rom(file: String) -> rom::ROM {
    // check if file exists
    if !fs::metadata(&file).is_ok() {
        panic!("File does not exist")
    }


    // read file
    let data = fs::read(file).expect("Unable to read file");
    rom::ROM::new(data)
}
fn rom_check() -> String {
    // check if file path is provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a ROM file")
    }

    // Read Rom file
    args[1].to_string()
}

fn main() {
    // Set the log level to debug
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Read Rom data
    let file_path = rom_check();

    let file = rom_file_path(&file_path);
    log::info!("Loading ROM file: {}", file);

    let rom = load_rom(file);

    // debug: read ROM data
    log::debug!("ROM data: {:?}", rom.get_buf());
}

#[test]
fn test_rom_file_path() {
    let file = rom_file_path("roms/hello-world/hello-world.gb");
    assert_eq!(file, "roms/hello-world/hello-world.gb");
}

#[test]
fn test_load_rom() {
    let file = rom_file_path("roms/hello-world/hello-world.gb");
    let rom = load_rom(file);
    assert_eq!(rom.read(0x100), 195);
}