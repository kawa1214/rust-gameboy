use std::fs;
use std::fs::File;
use std::io::BufReader;

use gb::rom::Rom;

fn main() {
    let mut reader = BufReader::new(File::open("test_roms/cpu_instrs.gb").unwrap());
    let rom = Rom::new(&mut reader);
    println!("{:#?}", rom);
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
        let mut reader = BufReader::new(File::open(file_dir).unwrap());
        let rom = Rom::new(&mut reader);
        println!("{:#?}", rom);
    }
}
