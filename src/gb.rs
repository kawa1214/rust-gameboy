use crate::mbc::new_mbc;
use crate::rom::Rom;
use std::fs::File;
use std::io::BufReader;

pub struct Gb {}

impl Gb {
    #[allow(unused_variables)]
    pub fn new(rom_path: &str) -> Gb {
        let mut reader = BufReader::new(File::open(rom_path).unwrap());
        let rom = Rom::new(&mut reader);
        let mbc = new_mbc(rom);
        return Gb {};
    }
}
