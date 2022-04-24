use crate::mbc::Mbc;
use crate::ram::Ram;

// The bus sits between the CPU and various hardware modules, and routes data reads/writes based on the given address

pub struct Bus {
    mbc: Box<dyn Mbc>,
    ram: Ram,
    pub ie: u8,
}

impl Bus {
    pub fn new(mbc: Box<dyn Mbc>) -> Bus {
        let ram = Ram::new();
        return Bus {
            mbc: mbc,
            ram: ram,
            ie: 0,
        };
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let low = self.read(addr);
        let high = self.read(addr + 1);

        return ((high as u16) << 8) | (low as u16);
    }

    /// https://gbdev.io/pandocs/Memory_Map.html
    pub fn read(&self, addr: u16) -> u8 {
        let val = match addr {
            // mbc
            0x0000..=0x7FFF => self.mbc.read(addr),
            0xA000..=0xBFFF => self.mbc.read(addr),
            0x8000..=0x9FFF => unimplemented!("unimplemented: ppc 0x{:04X}", addr),
            // ram
            // TODO: ram側に実装する
            0xC000..=0xDFFF => self.ram.work[(addr - 0xC000) as usize], //In CGB mode, switchable bank 1~7
            0xE000..=0xFDFF => self.ram.work[(addr - 0xE000) as usize], // ECHO RAM: Nintendo prohibits developers from using this memory range.
            0xFF80..=0xFFFE => self.ram.high[(addr - 0xFF80) as usize],

            // TODO: ppu

            // TODO: IO

            // Nintendo indicates use of this area is prohibited.
            // This area returns $FF when OAM is blocked,
            // and otherwise the behavior depends on the hardware revision.
            0xFEA0..=0xFEFF => 0,
            _ => 0,
        };

        return val;
    }

    pub fn write(&mut self, addr: u16, val: u8) -> () {}
}
