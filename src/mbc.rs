use crate::rom::{CartridgeType, Rom};

// TODO: move to defines
pub const KB: usize = 1024;

/// mbc(memory bank controller)
/// <https://gbdev.io/pandocs/MBCs.html>
///
/// Can map RAM that does not fit in the body
pub trait Mbc {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8) -> ();
}

pub fn new_mbc(rom: Rom) -> Box<dyn Mbc> {
    match rom.cartridge_type {
        CartridgeType::RomOnly => Box::new(RomOnly::new(rom)),
        CartridgeType::Mbc1 => Box::new(Mbc1::new(rom)),
        t => unimplemented!("unimplemented mbc: {:?}", t),
    }
}

/// The ROM is directly mapped to memory at $0000-7FFF
///
/// Optionally up to 8 KiB of RAM could be connected at $A000-BFFF,
/// using a discrete logic decoder in place of a full MBC chip
pub struct RomOnly {
    rom: Rom,
}

impl RomOnly {
    pub fn new(rom: Rom) -> RomOnly {
        return RomOnly { rom };
    }
}

impl Mbc for RomOnly {
    fn read(&self, addr: u16) -> u8 {
        if addr <= 0xBFFF {
            return self.rom.value[addr as usize];
        } else if addr > 0xBFFF {
            panic!("RomOnly::read: invalid address: 0x{:04X}", addr);
        } else {
            panic!("RomOnly::read: invalid address: 0x{:04X}", addr);
        }
    }

    #[allow(unused_variables)]
    fn write(&mut self, addr: u16, val: u8) -> () {
        panic!("RomOnly::write invalid");
    }
}

/// max 2MByte ROM and/or 32 KiB RAM
pub struct Mbc1 {
    rom: Rom,
    ram: [u8; 32 * KB],
}

impl Mbc1 {
    pub fn new(rom: Rom) -> Mbc1 {
        return Mbc1 {
            rom,
            ram: [0; 32 * KB],
        };
    }
}

impl Mbc for Mbc1 {
    fn read(&self, addr: u16) -> u8 {
        if addr <= 0xBFFF {
            return self.rom.value[addr as usize];
        } else if addr > 0xBFFF {
            panic!("RomOnly::read: invalid address: 0x{:04X}", addr);
        } else {
            panic!("RomOnly::read: invalid address: 0x{:04X}", addr);
        }
    }

    #[allow(unused_variables)]
    fn write(&mut self, addr: u16, val: u8) -> () {
        if addr <= 0x7FFF {
            panic!("Mbc1::write invalid");
        }
        if addr > 0xBFFF {
            panic!("Mbc1::write invalid");
        }
        self.ram[addr as usize] = val;
        panic!("RomOnly::write invalid");
    }
}
