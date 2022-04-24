use crate::bus::Bus;

/// 16-bit Name/Function
/// AF     Accumulator & Flags
/// BC     BC
/// DE     DE
/// HL     HL
/// SP     Stack Pointer
/// PC     Program Counter/Pointer
pub struct Registers {
    pub a: u8,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0x0000,
            pc: 0x0000,
        }
    }

    fn r8(&mut self, index: u8) -> u8 {
        match index {
            0 => self.b(),
            1 => self.c(),
            2 => self.d(),
            3 => self.e(),
            4 => self.h(),
            5 => self.l(),
            //6 => self.bus.read(self.hl),
            7 => self.a,
            _ => unimplemented!("unknown r8 {}", index),
        }
    }

    fn b(&self) -> u8 {
        ((self.bc & 0xFF00) >> 8) as u8
    }

    fn c(&self) -> u8 {
        (self.bc & 0x00FF) as u8
    }

    fn d(&self) -> u8 {
        ((self.de & 0xFF00) >> 8) as u8
    }

    fn e(&self) -> u8 {
        (self.de & 0x00FF) as u8
    }

    fn h(&self) -> u8 {
        ((self.hl & 0xFF00) >> 8) as u8
    }

    fn l(&self) -> u8 {
        (self.hl & 0x00FF) as u8
    }
}

/// Bit	Name Explanation
/// 7	z    Zero flag
/// 6	n    Subtraction flag (BCD)
/// 5	h    Half Carry flag (BCD)
/// 4	c    Carry flag
pub struct FlagsRegisters {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl FlagsRegisters {
    pub fn new() -> FlagsRegisters {
        FlagsRegisters {
            z: false,
            n: false,
            h: false,
            c: false,
        }
    }
}

pub struct Cpu {
    pub registers: Registers,
    pub flag_registers: FlagsRegisters,

    pub clock_cycles_wait: u8,

    bus: Bus,
}

impl Cpu {
    #[allow(unused_variables)]
    pub fn new(bus: Bus) -> Cpu {
        return Cpu {
            registers: Registers::new(),
            flag_registers: FlagsRegisters::new(),
            clock_cycles_wait: 0,
            bus: bus,
        };
    }

    pub fn step(&mut self) -> () {
        let operation_code = self.bus.read(self.registers.pc);
        println!(
            "operation_code: {:02X}, pc: {:02X}",
            operation_code, self.registers.pc
        );

        match operation_code {
            0 => {
                // Nopping
                //print!("step 0: {:02X}", self.registers.pc);
                //self.registers.pc += 1;
            }
            0x003C => {
                //self.inc_r(self.registers.pc);
            }
            0x00C3 => {
                self.jp_16(self.registers.pc);
            }
            unknown => panic!("unknown operation code type {:#X}", unknown),
        };

        self.registers.pc += 1;
    }

    fn inc_r(&mut self, index: u8) -> () {
        let l = self.registers.r8(index);
        let r = 1;
        let result = l.wrapping_add(r);
        //let value = self.registers.get_8(index);
    }

    fn jp_16(&mut self, addr: u16) -> () {
        let r_addr = self.bus.read_word(addr);

        self.clock_cycles_wait += 16;

        self.registers.pc = r_addr;
        println!("jp_16: {:02X}", r_addr);
    }
}
