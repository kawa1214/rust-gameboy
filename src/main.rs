use gb::gb::Gb;
use std::fs;

fn main() {
    Gb::new("test_roms/cpu_instrs.gb");
}

#[allow(dead_code)]
fn test() {
    let test_rom_files: Vec<String> = fs::read_dir("test_roms")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();

    for test_rom_file in test_rom_files {
        let file_dir = format!("test_roms/{}", test_rom_file);
        println!("{}", file_dir);
        Gb::new("test_roms/hello_world.gb");
    }
}
