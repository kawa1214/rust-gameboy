use gb::gb::Gb;
use std::fs;

fn main() {
    let mut gb = Gb::new("test_roms/cpu_instrs.gb");
    for _ in 0..5 {
        gb.step();
    }

    /*
    // Bit in opcode
    let test = 0xEC as u8; // 11101100

    let x = test >> 6; //11
    let y = test << 2 >> 5; // 101
    let z = test << 5 >> 5; // 100
    let p = y >> 1; // 10
    let q = y << 2 >> 4; //1

    println!("x: {:b}, y: {:b}, z: {:b}, p: {:b}, q: {:b}", x, y, z, p, q);
    */
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
