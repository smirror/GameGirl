use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn cli_path() -> &'static str {
    env!("CARGO_BIN_EXE_game_girl")
}

fn temp_path(file_name: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "game_girl_cli_{}_{}_{}",
        std::process::id(),
        suffix,
        file_name
    ))
}

fn rom_only_bytes() -> Vec<u8> {
    let mut bytes = vec![0; 0x8000];
    bytes[0x0134..0x0138].copy_from_slice(b"TEST");
    bytes[0x0147] = 0x00;
    bytes[0x0148] = 0x00;
    bytes[0x0149] = 0x00;
    bytes
}

#[test]
fn loads_valid_gb_rom() {
    let path = temp_path("valid.gb");
    fs::write(&path, rom_only_bytes()).expect("test ROM should be writable");

    let output = Command::new(cli_path())
        .arg(&path)
        .output()
        .expect("CLI should run");

    fs::remove_file(&path).ok();

    assert!(
        output.status.success(),
        "expected success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Loaded ROM: 32768 bytes"));
    assert!(stdout.contains("Header:"));
    assert!(stdout.contains("title: TEST"));
    assert!(stdout.contains("cartridge_type: RomOnly (0x00)"));
    assert!(stdout.contains("rom_size: 32768 bytes (code 0x00)"));
    assert!(stdout.contains("ram_size: 0 bytes (code 0x00)"));
}

#[test]
fn prints_header_before_rejecting_unsupported_cartridge_type() {
    let path = temp_path("unsupported.gb");
    let mut bytes = rom_only_bytes();
    bytes[0x0147] = 0x04;
    fs::write(&path, bytes).expect("test ROM should be writable");

    let output = Command::new(cli_path())
        .arg(&path)
        .output()
        .expect("CLI should run");

    fs::remove_file(&path).ok();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("cartridge_type: Unsupported(4) (0x04)"),
        "stdout should include parsed header, got: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("unsupported cartridge type: 0x04"),
        "stderr should explain unsupported type, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn loads_known_mbc_cartridge_type() {
    let path = temp_path("mbc1.gb");
    let mut bytes = rom_only_bytes();
    bytes[0x0147] = 0x03;
    bytes[0x0149] = 0x02;
    fs::write(&path, bytes).expect("test ROM should be writable");

    let output = Command::new(cli_path())
        .arg(&path)
        .output()
        .expect("CLI should run");

    fs::remove_file(&path).ok();

    assert!(
        output.status.success(),
        "expected success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cartridge_type: Mbc1RamBattery (0x03)"));
    assert!(stdout.contains("ram_size: 8192 bytes (code 0x02)"));
}

#[test]
fn accepts_case_insensitive_gbc_extension() {
    let path = temp_path("valid.GBC");
    fs::write(&path, rom_only_bytes()).expect("test ROM should be writable");

    let output = Command::new(cli_path())
        .arg(&path)
        .output()
        .expect("CLI should run");

    fs::remove_file(&path).ok();

    assert!(
        output.status.success(),
        "expected success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn rejects_missing_argument_with_usage() {
    let output = Command::new(cli_path()).output().expect("CLI should run");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Usage:"),
        "stderr should include usage, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn rejects_non_rom_extension_before_reading_file() {
    let output = Command::new(cli_path())
        .arg("notes.txt")
        .output()
        .expect("CLI should run");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("File must be a .gb or .gbc file"),
        "stderr should explain supported extensions, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn reports_missing_rom_file() {
    let path = temp_path("missing.gb");

    let output = Command::new(cli_path())
        .arg(&path)
        .output()
        .expect("CLI should run");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("could not read ROM"),
        "stderr should explain read failure, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn reports_too_short_rom_file() {
    let path = temp_path("too-short.gb");
    fs::write(&path, [0x01, 0x02, 0x03]).expect("test ROM should be writable");

    let output = Command::new(cli_path())
        .arg(&path)
        .output()
        .expect("CLI should run");

    fs::remove_file(&path).ok();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("ROM is too short: 3 bytes read, expected at least 336 bytes"),
        "stderr should explain header length failure, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
