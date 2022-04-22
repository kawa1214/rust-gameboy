use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

// https://gbdev.io/pandocs/The_Cartridge_Header.html#0100-0103---entry-point
const ENTRY_POINT_START: u64 = 0x0100;
const ENTRY_POINT_END: u64 = 0x0103;
const ENTRY_POINT_LEN: u64 = ENTRY_POINT_END - ENTRY_POINT_START + 1;

const LOGO_START: u64 = 0x0104;
const LOGO_END: u64 = 0x0133;
const LOGO_LEN: u64 = LOGO_END - LOGO_START + 1;

const TITLE_START: u64 = 0x0134;
const TITLE_END: u64 = 0x0143;
const TITLE_LEN: u64 = TITLE_END - TITLE_START;

const MANUFACTURER_CODE_START: u64 = 0x013F;
const MANUFACTURER_CODE_END: u64 = 0x0142;
const MANUFACTURER_CODE_LEN: u64 = MANUFACTURER_CODE_END - MANUFACTURER_CODE_START + 1;

const CGB_FLAG: u64 = 0x0143;

const NEW_LICENSEE_CODE_START: u64 = 0x0144;
const NEW_LICENSEE_CODE_END: u64 = 0x0145;
const NEW_LICENSEE_CODE_LEN: u64 = NEW_LICENSEE_CODE_END - NEW_LICENSEE_CODE_START + 1;

const SGB_FLAG: u64 = 0x0146;

const CARTRIDGE_TYPE: u64 = 0x0147;

const ROM_SIZE: u64 = 0x0148;

const RAM_SIZE: u64 = 0x0149;

const DESTINATION_CODE: u64 = 0x014A;

const OLD_LICENSEE_CODE: u64 = 0x014B;

const MASK_ROM_VERSION_NUMBER: u64 = 0x014C;

const HEADER_CHECKSUM: u64 = 0x014D;

const GLOBAL_CHECKSUM_START: u64 = 0x014E;
const GLOBAL_CHECKSUM_END: u64 = 0x014F;
const GLOBAL_CHECKSUM_LEN: u64 = GLOBAL_CHECKSUM_END - GLOBAL_CHECKSUM_START + 1;

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0b,
    Mmm01Ram = 0x0c,
    Mmm01RamBattery = 0x0d,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1a,
    Mbc5RamBattery = 0x1b,
    Mbc5Rumble = 0x1c,
    Mbc5RumbleRam = 0x1d,
    Mbc5RumbleRamBattery = 0x1e,
    Mbc6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xfc,
    BandaiTama5 = 0xfd,
    HuC3 = 0xfe,
    HuC1RamBattery = 0xff,
}

#[derive(Debug)]
pub enum DestinationCode {
    Japanese = 0x00,
    NonJapanese = 0x01,
}

pub struct Rom {
    pub entry_point: [u8; ENTRY_POINT_LEN as usize],
    pub logo: [u8; LOGO_LEN as usize],
    pub title: [u8; TITLE_LEN as usize],
    pub manufacturer_code: [u8; MANUFACTURER_CODE_LEN as usize],
    pub cgb_flag: bool,
    pub new_licensee_code: [u8; NEW_LICENSEE_CODE_LEN as usize],
    pub sgb_flag: bool,
    pub cartridge_type: CartridgeType,
    pub rom_size: usize,
    pub ram_size: usize,
    pub destination_code: DestinationCode,
    pub old_licensee_code: u8,
    pub mask_rom_version_number: u8,
    pub header_checksum: u8,
    pub global_checksum: [u8; GLOBAL_CHECKSUM_LEN as usize],
}

impl fmt::Debug for Rom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rom")
            .field("entry_point", &bytes_to_hex(&self.entry_point[..]))
            .field("logo", &bytes_to_hex(&self.logo[..]))
            .field("title", &bytes_to_hex(&self.title[..]))
            .field("new_licensee_code", &bytes_to_hex(&self.new_licensee_code))
            .field("sgb_flag", &self.sgb_flag)
            .field("cartridge_type", &self.cartridge_type)
            .field("rom_size", &self.rom_size)
            .field("ram_size", &self.ram_size)
            .field("destination_code", &self.destination_code)
            .field("old_licensee_code", &self.old_licensee_code)
            .field("mask_rom_version_number", &self.mask_rom_version_number)
            .field("header_checksum", &self.header_checksum)
            .field("global_checksum", &bytes_to_hex(&self.global_checksum))
            .finish()
    }
}

impl Default for Rom {
    fn default() -> Self {
        Rom {
            entry_point: [0; ENTRY_POINT_LEN as usize],
            logo: [0; LOGO_LEN as usize],
            title: [0; TITLE_LEN as usize],
            manufacturer_code: [0; MANUFACTURER_CODE_LEN as usize],
            cgb_flag: false,
            new_licensee_code: [0; NEW_LICENSEE_CODE_LEN as usize],
            sgb_flag: false,
            cartridge_type: CartridgeType::RomOnly,
            rom_size: 0,
            ram_size: 0,
            destination_code: DestinationCode::Japanese,
            old_licensee_code: 0,
            mask_rom_version_number: 0,
            header_checksum: 0,
            global_checksum: [0; GLOBAL_CHECKSUM_LEN as usize],
        }
    }
}

impl Rom {
    pub fn new(reader: &mut BufReader<File>) -> Rom {
        let mut rom = Rom::default();

        reader.seek(SeekFrom::Start(ENTRY_POINT_START)).unwrap();
        reader.read_exact(&mut rom.entry_point[..]).unwrap();

        reader.seek(SeekFrom::Start(LOGO_START)).unwrap();
        reader.read_exact(&mut rom.logo[..]).unwrap();

        reader.seek(SeekFrom::Start(TITLE_START)).unwrap();
        reader.read_exact(&mut rom.title[..]).unwrap();

        reader.seek(SeekFrom::Start(CGB_FLAG)).unwrap();
        rom.cgb_flag = match reader.take(1).bytes().next() {
            Some(Ok(0x80)) => false,
            Some(Ok(0xC0)) => true,
            Some(Ok(_unknown)) => false,
            Some(Err(e)) => panic!("error occured while reading the CGB Flag {}", e),
            None => panic!("unexpected EOF while reading the CGB Flag"),
        };

        reader
            .seek(SeekFrom::Start(NEW_LICENSEE_CODE_START))
            .unwrap();
        reader.read_exact(&mut rom.new_licensee_code[..]).unwrap();

        reader.seek(SeekFrom::Start(SGB_FLAG)).unwrap();
        rom.sgb_flag = match reader.take(1).bytes().next() {
            Some(Ok(0x00)) => false,
            Some(Ok(0x03)) => true,
            Some(Ok(unknown)) => panic!("unknown SGB Flag {:#X}", unknown),
            Some(Err(e)) => panic!("error occured while reading the SGB Flag {}", e),
            None => panic!("unexpected EOF while reading the SGB Flag"),
        };

        reader.seek(SeekFrom::Start(CARTRIDGE_TYPE)).unwrap();

        if let Some(Ok(value)) = reader.take(1).bytes().next() {
            rom.cartridge_type = match value {
                0x00 => CartridgeType::RomOnly,
                0x01 => CartridgeType::Mbc1,
                0x02 => CartridgeType::Mbc1Ram,
                0x03 => CartridgeType::Mbc1RamBattery,
                0x05 => CartridgeType::Mbc2,
                0x06 => CartridgeType::Mbc2Battery,
                0x08 => CartridgeType::RomRam,
                0x09 => CartridgeType::RomRamBattery,
                0x0b => CartridgeType::Mmm01,
                0x0c => CartridgeType::Mmm01Ram,
                0x0d => CartridgeType::Mmm01RamBattery,
                0x11 => CartridgeType::Mbc3,
                0x12 => CartridgeType::Mbc3Ram,
                0x13 => CartridgeType::Mbc3RamBattery,
                0x19 => CartridgeType::Mbc5,
                0x1a => CartridgeType::Mbc5Ram,
                0x1b => CartridgeType::Mbc5RamBattery,
                0x1c => CartridgeType::Mbc5Rumble,
                0x1d => CartridgeType::Mbc5RumbleRam,
                0x1e => CartridgeType::Mbc5RumbleRamBattery,
                0x20 => CartridgeType::Mbc6,
                0x22 => CartridgeType::Mbc7SensorRumbleRamBattery,
                0xfc => CartridgeType::PocketCamera,
                0xfd => CartridgeType::BandaiTama5,
                0xfe => CartridgeType::HuC3,
                0xff => CartridgeType::HuC1RamBattery,
                unknown => panic!("unknown cartridge type {:#X}", unknown),
            };
        } else {
            panic!("unexpected EOF while reading the Cartridge Type");
        }

        reader.seek(SeekFrom::Start(ROM_SIZE)).unwrap();
        rom.rom_size = match reader.take(1).bytes().next() {
            Some(Ok(n @ 0x00..=0x08)) => ((32 * 1024) << n) as usize,
            Some(Ok(0x52)) => (1.1 * 1024.0 * 1024.0) as usize,
            Some(Ok(0x53)) => (1.2 * 1024.0 * 1024.0) as usize,
            Some(Ok(0x54)) => (1.5 * 1024.0 * 1024.0) as usize,
            Some(Ok(unknown)) => panic!("unknown ROM Size {:#X}", unknown),
            Some(Err(e)) => panic!("error occured while reading the ROM Size {}", e),
            None => panic!("unexpected EOF while reading the ROM Size"),
        };

        reader.seek(SeekFrom::Start(RAM_SIZE)).unwrap();
        rom.ram_size = match reader.take(1).bytes().next() {
            Some(Ok(0x00)) => 0_usize,
            Some(Ok(0x01)) => 2 * 1024 * 1024_usize,
            Some(Ok(0x02)) => 8 * 1024 * 1024_usize,
            Some(Ok(0x03)) => 32 * 1024 * 1024_usize,
            Some(Ok(0x04)) => 128 * 1024 * 1024_usize,
            Some(Ok(0x05)) => 64 * 1024 * 1024_usize,
            Some(Ok(unknown)) => panic!("unknown RAM Size {:#X}", unknown),
            Some(Err(e)) => panic!("error occured while reading the RAM Size {}", e),
            None => panic!("unexpected EOF while reading the RAM Size"),
        };

        reader.seek(SeekFrom::Start(DESTINATION_CODE)).unwrap();
        if let Some(Ok(value)) = reader.take(1).bytes().next() {
            rom.destination_code = match value {
                0x00 => DestinationCode::Japanese,
                0x01 => DestinationCode::NonJapanese,
                unknown => panic!("unknown cartridge type {:#X}", unknown),
            };
        } else {
            panic!("unexpected EOF while reading the Cartridge Type");
        }

        reader.seek(SeekFrom::Start(OLD_LICENSEE_CODE)).unwrap();
        rom.old_licensee_code = match reader.take(1).bytes().next() {
            Some(Ok(value)) => value,
            Some(Err(e)) => panic!("error occured while reading the Old Licensee Code {}", e),
            None => panic!("unexpected EOF while reading the Old Licensee Code"),
        };

        reader
            .seek(SeekFrom::Start(MASK_ROM_VERSION_NUMBER))
            .unwrap();
        rom.mask_rom_version_number = match reader.take(1).bytes().next() {
            Some(Ok(value)) => value,
            Some(Err(e)) => panic!(
                "error occured while reading the Mask ROM Version Number {}",
                e
            ),
            None => panic!("unexpected EOF while reading the Mask ROM Version Number"),
        };

        reader
            .seek(SeekFrom::Start(MASK_ROM_VERSION_NUMBER))
            .unwrap();
        rom.mask_rom_version_number = match reader.take(1).bytes().next() {
            Some(Ok(value)) => value,
            Some(Err(e)) => panic!(
                "error occured while reading the Compatible Cartridge Version Number {}",
                e
            ),
            None => panic!("unexpected EOF while reading the Compatible Cartridge Version Number"),
        };

        reader.seek(SeekFrom::Start(HEADER_CHECKSUM)).unwrap();
        rom.header_checksum = match reader.take(1).bytes().next() {
            Some(Ok(value)) => value,
            Some(Err(e)) => panic!("error occured while reading the Header Checksum {}", e),
            None => panic!("unexpected EOF while reading the Header Checksum"),
        };

        reader.seek(SeekFrom::Start(GLOBAL_CHECKSUM_START)).unwrap();
        reader.read_exact(&mut rom.global_checksum[..]).unwrap();

        let mut chksum: u8 = 0;
        reader.seek(SeekFrom::Start(TITLE_START)).unwrap();
        for _ in 0x0134..=0x014C {
            if let Some(Ok(b)) = reader.take(1).bytes().next() {
                chksum = chksum.wrapping_sub(b).wrapping_sub(1);
            } else {
                panic!("error occured while checking header chksum");
            }
        }
        if rom.header_checksum != chksum {
            panic!(
                "invalid checksum expected: {}, actual: {}",
                rom.header_checksum, chksum
            );
        }

        let mut rom_bytes = Vec::new();
        reader.seek(SeekFrom::Start(0)).unwrap();
        reader.read_to_end(&mut rom_bytes).unwrap();
        if rom.rom_size != rom_bytes.len() {
            panic!(
                "invalid rom size expected: {}, actual: {}",
                rom.rom_size,
                rom_bytes.len(),
            );
        }

        return rom;
        //return Rom;
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| format!("{:02X} ", b))
        .collect::<String>()
}
