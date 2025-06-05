use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <rom.gb|rom.gbc>", args[0]);
        return;
    }

    let file_path = &args[1];

    if !file_path.ends_with(".gb") && !file_path.ends_with(".gbc") {
        println!("File must be a .gb or .gbc file");
        return;
    }

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    println!("with text:\n{}", contents);
}
