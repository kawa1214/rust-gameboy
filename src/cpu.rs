use crate::bus::Bus;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// 16-bit Name/Function
/// AF     Accumulator & Flags
/// BC     BC
/// DE     DE
/// HL     HL
/// SP     Stack Pointer
/// PC     Program Counter/Pointer
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
}

#[derive(FromPrimitive)]
enum Cc {
    Nz = 0,
    Z = 1,
    Nc = 2,
    C = 3,
}

#[derive(FromPrimitive)]
enum Rp {
    Bc = 0,
    De = 1,
    Hl = 2,
    Sp = 3,
}

#[derive(FromPrimitive)]
enum Rp2 {
    Bc = 0,
    De = 1,
    Hl = 2,
    Af = 3,
}

#[derive(FromPrimitive)]
enum R {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    Hl = 6,
    A = 7,
}

#[derive(FromPrimitive)]
enum Alu {
    AddA = 0,
    AdcA = 1,
    Sub = 2,
    SubA = 3,
    And = 4,
    Xor = 5,
    Or = 6,
    Cp = 7,
}

#[derive(FromPrimitive)]
enum Rot {
    Rlc = 0,
    Rrc = 1,
    Rl = 2,
    Rr = 3,
    Sla = 4,
    Sra = 5,
    Swap = 6,
    Srl = 7,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            f: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0x0000,
            pc: 0x0000,
        }
    }

    pub fn set_a(&mut self, val: u8) {
        self.a = val;
    }

    pub fn set_bc(&mut self, bc: u16) {
        self.bc = bc;
    }

    pub fn set_de(&mut self, de: u16) {
        self.de = de;
    }

    pub fn set_hl(&mut self, hl: u16) {
        //self.bus.write(self.hl, val);
        self.hl = hl;
    }

    fn get_a(&mut self) -> u8 {
        self.a
    }

    fn get_r(&self, r: R) -> u8 {
        match r {
            R::B => self.get_b(),
            R::C => self.get_c(),
            R::D => self.get_d(),
            R::E => self.get_e(),
            R::H => self.get_h(),
            R::Hl => self.get_hl(),
            R::L => self.get_l(),
            R::A => self.a,
        }
    }

    fn get_b(&self) -> u8 {
        ((self.bc & 0xFF00) >> 8) as u8
    }

    fn get_c(&self) -> u8 {
        (self.bc & 0x00FF) as u8
    }

    fn get_d(&self) -> u8 {
        ((self.de & 0xFF00) >> 8) as u8
    }

    fn get_e(&self) -> u8 {
        (self.de & 0x00FF) as u8
    }

    fn get_h(&self) -> u8 {
        ((self.hl & 0xFF00) >> 8) as u8
    }

    fn get_hl(&self) -> u8 {
        let test = 123;
        // self.bus.read(self.hl)
        test
    }

    fn get_l(&self) -> u8 {
        (self.hl & 0x00FF) as u8
    }

    fn set_b(&mut self, val: u8) {
        self.bc &= 0x00FF;
        self.bc |= ((val as u16) << 8) as u16;
    }

    fn set_c(&mut self, val: u8) {
        self.bc &= 0xFF00;
        self.bc |= val as u16;
    }

    fn set_d(&mut self, val: u8) {
        self.de &= 0x00FF;
        self.de |= ((val as u16) << 8) as u16;
    }

    fn set_e(&mut self, val: u8) {
        self.de &= 0xFF00;
        self.de |= val as u16;
    }

    fn set_h(&mut self, val: u8) {
        self.hl &= 0x00FF;
        self.hl |= ((val as u16) << 8) as u16;
    }

    fn set_l(&mut self, val: u8) {
        self.hl &= 0xFF00;
        self.hl |= val as u16;
    }

    fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0x00F0) as u8;
    }

    fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    fn get_rp2(&mut self, rp2: Rp2, high: bool) -> u16 {
        match rp2 {
            Rp2::Bc => self.bc,
            Rp2::De => self.de,
            Rp2::Hl => self.hl,
            Rp2::Af => {
                if high {
                    return self.af();
                } else {
                    return self.sp;
                }
            }
        }
    }

    fn set_r16(&mut self, rp2: Rp2, val: u16, high: bool) {
        match rp2 {
            Rp2::Bc => {
                self.bc = val;
            }
            Rp2::De => {
                self.de = val;
            }
            Rp2::Hl => {
                self.hl = val;
            }
            Rp2::Af => {
                if !high {
                    self.set_af(val);
                } else {
                    self.sp = val;
                }
            }
        }
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

    fn get_c(&self, cc: Cc) -> bool {
        match cc {
            Cc::Nz => !self.z,
            Cc::Z => self.z,
            Cc::Nc => !self.c,
            Cc::C => self.c,
        }
    }

    fn set_z(&mut self, val: bool) {
        self.z = val;
    }

    fn set_n(&mut self, val: bool) {
        self.n = val;
    }

    fn set_h(&mut self, val: bool) {
        self.h = val;
    }

    fn set_c(&mut self, val: bool) {
        self.c = val;
    }
}

pub struct Cpu {
    pub registers: Registers,
    pub flag_registers: FlagsRegisters,

    pub clock_cycles_wait: u8,

    halt: bool,
    ime: bool,

    bus: Bus,
}

impl Cpu {
    #[allow(unused_variables)]
    pub fn new(bus: Bus) -> Cpu {
        return Cpu {
            registers: Registers::new(),
            flag_registers: FlagsRegisters::new(),
            clock_cycles_wait: 0,

            halt: false,
            ime: false,

            bus: bus,
        };
    }

    fn get_n(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.registers.pc);
        self.registers.pc += 1;

        byte
    }

    fn get_nn(&mut self) -> u16 {
        let word = self.bus.read_word(self.registers.pc);
        self.registers.pc += 2;

        word
    }

    pub fn step(&mut self) -> () {
        let opcode = self.get_n();
        println!("opcode: {:02X}, pc: {:02X}", opcode, self.registers.pc);
        self.call_operation(opcode);

        self.registers.pc += 1;
    }

    fn call_operation(&mut self, opcode: u8) {
        // https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html
        let x = opcode >> 6;
        let y = opcode << 2 >> 5;
        let z = opcode << 5 >> 5;
        let p = y >> 1;
        let q = y << 2 >> 4;

        // xx yyy zzz
        //    ppq
        println!("x: {:b}, y: {:b}, z: {:b}, p: {:b}, q: {:b}", x, y, z, p, q);
        match x {
            0 => match z {
                0 => match y {
                    0 => self.nop(),
                    1 => self.ld_nn_sp(),
                    2 => self.stop(),
                    3 => self.jr_d(),
                    4..=7 => {
                        let cc = Cc::from_u8(y - 4).unwrap();
                        self.jr_cc_d(cc);
                    }
                    _ => panic!("unknown type  x: {:X}, z: {:X}, y: {:X}", x, z, y),
                },
                1 => match q {
                    0 => {
                        let rp = Rp::from_u8(p).unwrap();
                        self.ld_rp_nn(rp)
                    }
                    1 => {
                        let rp = Rp::from_u8(p).unwrap();
                        self.add_hl_rp(rp);
                    }
                    _ => panic!("unknown type  x: {:X}, z: {:X}, q: {:X}", x, z, q),
                },
                2 => match q {
                    0 => match p {
                        0 => self.ld_bc_a(),
                        1 => self.ld_hl_de_a(),
                        2 => self.ld_hl_p_a(),
                        3 => self.ld_hl_m_a(),
                        _ => panic!(
                            "unknown type  x: {:X}, z: {:X}, q: {:X}, p: {:X}",
                            x, z, q, p
                        ),
                    },
                    1 => match p {
                        0 => self.ld_a_bc(),
                        1 => self.ld_a_de(),
                        2 => self.ld_a_p_hl(),
                        3 => self.ld_a_m_hl(),
                        _ => panic!(
                            "unknown type  x: {:X}, z: {:X}, q: {:X}, p: {:X}",
                            x, z, q, p
                        ),
                    },
                    _ => panic!("unknown type  x: {:X}, z: {:X}, q: {:X}", x, z, q),
                },
                3 => match q {
                    0 => {
                        let rp = Rp::from_u8(p).unwrap();
                        self.inc_rp(rp);
                    }
                    1 => {
                        let rp = Rp::from_u8(p).unwrap();
                        self.dec_rp(rp);
                    }
                    _ => panic!("unknown type  x: {:X}, z: {:X}, q: {:X}", x, z, q),
                },
                4 => {
                    let r = R::from_u8(p).unwrap();
                    self.inc_r(r);
                }
                5 => {
                    let r = R::from_u8(p).unwrap();
                    self.dec_r(r);
                }
                6 => {
                    let r = R::from_u8(y).unwrap();
                    self.ld_r_n(r);
                }
                7 => match y {
                    0 => self.rlca(),
                    1 => self.rrca(),
                    2 => self.rla(),
                    3 => self.rra(),
                    4 => self.daa(),
                    5 => self.cpl(),
                    6 => self.scf(),
                    7 => self.ccf(),
                    _ => panic!("unknown type  x: {:X}, z: {:X}, y: {:X}", x, z, y),
                },
                _ => panic!("unknown type  x: {:X}, z: {:X}", x, z),
            },
            1 => {
                if z == 6 && y == 6 {
                    self.halt();
                } else {
                    let l = R::from_u8(y).unwrap();
                    let r = R::from_u8(z).unwrap();
                    self.ld_r_r(l, r);
                }
            }
            2 => {
                let alu = Alu::from_u8(y).unwrap();
                let r = R::from_u8(y).unwrap();
                match alu {
                    Alu::AddA => self.add_a_r(r),
                    Alu::AdcA => self.adc_a_r(r),
                    Alu::Sub => self.sub_n(), // 怪しい
                    Alu::SubA => self.sub_a_r(r),
                    Alu::And => self.and_a_r(r),
                    Alu::Xor => self.xor_a_r(r),
                    Alu::Or => self.or_a_r(r),
                    Alu::Cp => self.cp_a_r(r),
                }
            }
            3 => match z {
                0 => match y {
                    0..=3 => {
                        let cc = Cc::from_u8(y).unwrap();
                        self.ret_cc(cc);
                    }
                    4 => self.ld_8n_a(),
                    5 => self.add_sp_d(),
                    6 => self.ld_a_8n(),
                    7 => self.lh_hl_sp_d(),
                    _ => panic!("unknown type  x: {:X}, z: {:X}, y: {:X}", x, z, y),
                },
                1 => match q {
                    0 => {
                        let rp2 = Rp2::from_u8(p).unwrap();
                        self.pop_rp2(rp2);
                    }
                    1 => match p {
                        0 => self.ret(),
                        1 => self.reti(),
                        2 => self.jp_hl(),
                        3 => self.ld_sp_hl(),
                        _ => panic!(
                            "unknown type  x: {:X}, z: {:X}, q: {:X}, p: {:X}",
                            x, z, q, p
                        ),
                    },
                    _ => panic!("unknown type  x: {:X}, z: {:X}, q: {:X}", x, z, q),
                },
                2 => match y {
                    0..=3 => {
                        let cc = Cc::from_u8(y).unwrap();
                        self.jp_cc_nn(cc);
                    }
                    4 => self.ld_8c_a(),
                    5 => self.ld_nn_a(),
                    6 => self.ld_a_8c(),
                    7 => self.ld_a_nn(),
                    _ => panic!("unknown type  x: {:X}, z: {:X}, y: {:X}", x, z, y),
                },
                3 => match y {
                    0 => self.jp_nn(),
                    1 => {
                        //self.cb_prefix();
                        let prefixed_opcode = self.get_n();
                        self.call_prefixed_operation(prefixed_opcode);
                    }
                    6 => self.di(),
                    7 => self.ei(),
                    _ => panic!("unknown type  x: {:X}, z: {:X}, y: {:X}", x, z, y),
                },
                4 => match q {
                    0..=3 => {
                        let cc = Cc::from_u8(y).unwrap();
                        self.call_cc_nn(cc);
                    }
                    _ => panic!("unknown type  x: {:X}, z: {:X}, q: {:X}", x, z, q),
                },
                5 => match q {
                    0 => {
                        let rp2 = Rp2::from_u8(p).unwrap();
                        self.push_rp2(rp2);
                    }
                    1 => match p {
                        0 => self.call_nn(),
                        _ => panic!("unknown type  x: {:X}, z: {:X}, q: {:X}", x, z, q),
                    },
                    _ => panic!("unknown type  x: {:X}, z: {:X}, q: {:X}", x, z, q),
                },
                6 => {
                    let alu = Alu::from_u8(y).unwrap();
                    match alu {
                        Alu::AddA => self.add_a_n(),
                        Alu::AdcA => self.adc_a_n(),
                        Alu::Sub => self.sub_n(), // 怪しい
                        Alu::SubA => self.sub_a_n(),
                        Alu::And => self.and_a_n(),
                        Alu::Xor => self.xor_a_n(),
                        Alu::Or => self.or_a_n(),
                        Alu::Cp => self.cp_a_n(),
                    }
                }
                7 => {
                    self.rst(y * 8);
                }
                _ => panic!("unknown type  x: {:X}, z: {:X}", x, z),
            },
            _ => panic!("unknown type x: {:X}", x),
        }
    }

    fn call_prefixed_operation(&mut self, opcode: u8) {
        let x = opcode >> 6;
        let y = opcode << 2 >> 5;
        let z = opcode << 5 >> 5;
        let p = y >> 1;
        let q = y << 2 >> 4;

        match x {
            0 => {
                //rot
            }
            1 => {
                //bit
            }
            2 => {
                //res
            }
            4 => {
                //set
            }
            _ => panic!("unknown prefixed type  x: {:X}", x),
        }
    }

    fn nop(&mut self) {}

    fn ld_nn_sp(&mut self) {
        let nn = self.get_nn();
        self.bus.write_word(nn, self.registers.sp);
    }

    fn stop(&mut self) {}

    fn jr_d(&mut self) {
        let d = self.get_n();
        self.registers.pc = self.registers.pc.wrapping_add(d as u16);
    }

    fn jr_cc_d(&mut self, cc: Cc) {
        let d = self.get_n();
        let val = match cc {
            Cc::Nz => !self.flag_registers.z,
            Cc::Z => self.flag_registers.z,
            Cc::Nc => !self.flag_registers.c,
            Cc::C => self.flag_registers.c,
        };

        if val {
            self.registers.pc = self.registers.pc.wrapping_add(d as u16);
        }
    }

    fn ld_rp_nn(&mut self, rp: Rp) {
        let nn = self.get_nn();
        match rp {
            Rp::Bc => self.registers.bc = nn,
            Rp::De => self.registers.de = nn,
            Rp::Hl => self.registers.hl = nn,
            Rp::Sp => self.registers.sp = nn,
        }
    }

    fn add_hl_rp(&mut self, rp: Rp) {
        match rp {
            Rp::Bc => self.registers.hl.wrapping_add(self.registers.bc),
            Rp::De => self.registers.hl.wrapping_add(self.registers.de),
            Rp::Hl => self.registers.hl.wrapping_add(self.registers.hl),
            Rp::Sp => self.registers.hl.wrapping_add(self.registers.sp),
        };
    }

    fn ld_bc_a(&mut self) {
        self.bus.write_byte(self.registers.bc, self.registers.a);
    }

    fn ld_hl_de_a(&mut self) {
        self.bus.write_byte(self.registers.hl, self.registers.a);
    }

    fn ld_hl_p_a(&mut self) {
        self.bus.write_byte(self.registers.hl, self.registers.a);
        self.registers.hl = self.registers.hl.wrapping_add(1);
    }

    fn ld_hl_m_a(&mut self) {
        self.bus.write_byte(self.registers.hl, self.registers.a);
        self.registers.hl = self.registers.hl.wrapping_sub(1);
    }

    fn ld_a_bc(&mut self) {
        self.bus.write_byte(self.registers.bc, self.registers.a);
    }

    fn ld_a_de(&mut self) {
        self.bus.write_byte(self.registers.de, self.registers.a);
    }

    fn ld_a_p_hl(&mut self) {
        self.bus.write_byte(self.registers.hl, self.registers.a);
        self.registers.hl = self.registers.hl.wrapping_add(1);
    }

    fn ld_a_m_hl(&mut self) {
        self.bus.write_byte(self.registers.hl, self.registers.a);
        self.registers.hl = self.registers.hl.wrapping_sub(1);
    }

    fn inc_rp(&mut self, rp: Rp) {
        match rp {
            Rp::Bc => self.registers.bc = self.registers.bc.wrapping_add(1),
            Rp::De => self.registers.de = self.registers.de.wrapping_add(1),
            Rp::Hl => self.registers.hl = self.registers.hl.wrapping_add(1),
            Rp::Sp => self.registers.sp = self.registers.sp.wrapping_add(1),
        }
    }

    fn dec_rp(&mut self, rp: Rp) {
        match rp {
            Rp::Bc => self.registers.bc = self.registers.bc.wrapping_sub(1),
            Rp::De => self.registers.de = self.registers.de.wrapping_sub(1),
            Rp::Hl => self.registers.hl = self.registers.hl.wrapping_sub(1),
            Rp::Sp => self.registers.sp = self.registers.sp.wrapping_sub(1),
        }
    }

    fn inc_r(&mut self, r: R) {
        match r {
            R::B => self.registers.set_b(self.registers.get_b().wrapping_add(1)),
            R::C => self.registers.set_c(self.registers.get_c().wrapping_add(1)),
            R::D => self.registers.set_d(self.registers.get_d().wrapping_add(1)),
            R::E => self.registers.set_e(self.registers.get_e().wrapping_add(1)),
            R::H => self.registers.set_h(self.registers.get_h().wrapping_add(1)),
            R::Hl => self.registers.set_hl(self.registers.hl.wrapping_add(1)),
            R::L => self.registers.set_l(self.registers.get_l().wrapping_add(1)),
            R::A => self.registers.set_a(self.registers.a.wrapping_add(1)),
        }
    }

    fn dec_r(&mut self, r: R) {
        match r {
            R::B => self.registers.set_b(self.registers.get_b().wrapping_sub(1)),
            R::C => self.registers.set_c(self.registers.get_c().wrapping_sub(1)),
            R::D => self.registers.set_d(self.registers.get_d().wrapping_sub(1)),
            R::E => self.registers.set_e(self.registers.get_e().wrapping_sub(1)),
            R::H => self.registers.set_h(self.registers.get_h().wrapping_sub(1)),
            R::Hl => self.registers.set_hl(self.registers.hl.wrapping_sub(1)),
            R::L => self.registers.set_l(self.registers.get_l().wrapping_sub(1)),
            R::A => self.registers.set_a(self.registers.a.wrapping_sub(1)),
        }
    }

    fn ld_r_n(&mut self, r: R) {
        let n = self.get_n();
        match r {
            R::B => self.registers.set_b(n),
            R::C => self.registers.set_c(n),
            R::D => self.registers.set_d(n),
            R::E => self.registers.set_e(n),
            R::H => self.registers.set_h(n),
            R::Hl => self.registers.set_hl(n as u16),
            R::L => self.registers.set_l(n),
            R::A => self.registers.set_a(n),
        }
    }

    fn rlca(&mut self) {
        let val = self.registers.a;
        let c = (val >> 7) & 1;
        let result = val.rotate_left(1);

        self.flag_registers.set_z(false);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(c == 1);

        self.registers.a = result;
    }

    fn rrca(&mut self) {
        let val = self.registers.a;
        let c = val & 1;
        let result = val.rotate_right(1);

        self.registers.a = result;

        self.flag_registers.set_z(false);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(c == 1);
    }

    fn rla(&mut self) {
        let val = self.registers.a;
        let c = (val >> 7) & 1;
        let result = val << 1 | self.flag_registers.c as u8;

        self.registers.a = result;

        self.flag_registers.set_z(false);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(c == 1);
    }

    fn rra(&mut self) {
        let val = self.registers.a;
        let c = val & 1;
        let result = val >> 1 | ((self.flag_registers.c as u8) << 7);

        self.registers.a = result;

        self.flag_registers.set_z(false);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(c == 1);
    }

    fn daa(&mut self) {
        if !self.flag_registers.n {
            if self.flag_registers.c || self.registers.a > 0x99 {
                self.registers.a = self.registers.a.wrapping_add(0x60);
                self.flag_registers.set_c(true);
            }
            if self.flag_registers.h || (self.registers.a & 0x0F) > 0x09 {
                self.registers.a = self.registers.a.wrapping_add(0x06);
            }
        } else {
            if self.flag_registers.c {
                self.registers.a = self.registers.a.wrapping_sub(0x60);
            }
            if self.flag_registers.h {
                self.registers.a = self.registers.a.wrapping_sub(0x06);
            }
        }

        self.flag_registers.set_z(self.registers.a == 0);
        self.flag_registers.set_h(false);
    }

    fn cpl(&mut self) {
        let val = self.registers.a;
        let result = !val;

        self.registers.a = result;
        self.flag_registers.set_n(true);
        self.flag_registers.set_h(true);
    }

    fn scf(&mut self) {
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(true);
    }

    fn ccf(&mut self) {
        let c = self.flag_registers.c;
        let result = !c;

        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(result);
    }

    fn ld_r_r(&mut self, left: R, right: R) {
        let val = match right {
            R::B => self.registers.get_b(),
            R::C => self.registers.get_c(),
            R::D => self.registers.get_d(),
            R::E => self.registers.get_e(),
            R::H => self.registers.get_h(),
            R::Hl => self.registers.get_hl(),
            R::L => self.registers.get_l(),
            R::A => self.registers.get_a(),
        };

        match left {
            R::B => self.registers.set_b(val),
            R::C => self.registers.set_c(val),
            R::D => self.registers.set_d(val),
            R::E => self.registers.set_e(val),
            R::H => self.registers.set_h(val),
            R::Hl => self.registers.set_hl(val as u16),
            R::L => self.registers.set_l(val),
            R::A => self.registers.set_a(val),
        }
    }

    fn halt(&mut self) {
        self.halt = true;
    }

    fn add_a_r(&mut self, r: R) {
        let left = self.registers.a;
        let right = self.registers.get_r(r);
        let result = left.wrapping_add(right);

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(false);
        self.flag_registers
            .set_h(self.half_carry_positive(left, right));
        self.flag_registers.set_c(self.carry_positive(left, right));
    }

    fn adc_a_r(&mut self, r: R) {
        let c = self.flag_registers.c as u8;
        let right = self.registers.get_r(r);
        let left = self.registers.a;
        let result1 = left.wrapping_add(right);
        let result2 = result1.wrapping_add(c);

        let c1 = self.carry_positive(left, right);
        let h1 = self.half_carry_positive(left, right);
        let c2 = self.carry_positive(result1, c);
        let h2 = self.half_carry_positive(result1, c);

        self.registers.a = result2;

        self.flag_registers.set_z(result2 == 0);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(h1 || h2);
        self.flag_registers.set_c(c1 || c2);
    }

    fn sub_n(&mut self) {
        let left = self.registers.a;
        let right = self.get_n();
        let result = left.wrapping_sub(right);

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(true);
        self.flag_registers
            .set_h(self.half_carry_negative(left, right));
        self.flag_registers.set_c(self.carry_negative(left, right));
    }

    fn sub_a_r(&mut self, r: R) {
        let left = self.registers.a;
        let right = self.registers.get_r(r);
        let result = left.wrapping_sub(right);

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(true);
        self.flag_registers
            .set_h(self.half_carry_negative(left, right));
        self.flag_registers.set_c(self.carry_negative(left, right));
    }

    fn xor_a_r(&mut self, r: R) {
        let left = self.registers.a;
        let right = self.registers.get_r(r);
        let result = left & right;

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(true);
        self.flag_registers.set_c(false);
    }

    fn and_a_r(&mut self, r: R) {
        let left = self.registers.a;
        let right = self.registers.get_r(r);
        let result = left & right;

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(true);
        self.flag_registers.set_c(false);
    }

    fn or_a_r(&mut self, r: R) {
        let left = self.registers.a;
        let right = self.registers.get_r(r);
        let result = left | right;

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(false);
    }

    fn cp_a_r(&mut self, r: R) {
        let left = self.registers.a;
        let right = self.registers.get_r(r);
        let result = left.wrapping_sub(right);

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(true);
        self.flag_registers
            .set_h(self.half_carry_negative(left, right));
        self.flag_registers.set_c(self.carry_negative(left, right));
    }

    fn ret_cc(&mut self, cc: Cc) {
        let val = self.flag_registers.get_c(cc);

        if val {
            let nn = self.get_nn();
            self.registers.pc = nn;
        }
    }

    fn ld_8n_a(&mut self) {
        let index = self.get_n();
        let addr = 0xFF00 + index as u16;
        self.bus.write_byte(addr, self.registers.a);
    }

    fn add_sp_d(&mut self) {
        let left = self.registers.sp;
        let right = self.bus.read_byte(self.registers.pc) as i8 as u16;
        let result = left.wrapping_add(right);

        self.registers.sp = result;

        self.flag_registers.set_z(false);
        self.flag_registers.set_n(false);
        self.flag_registers
            .set_h(self.half_carry_positive(left as u8, right as u8));
        self.flag_registers
            .set_c(self.carry_positive(left as u8, right as u8));
    }

    fn ld_a_8n(&mut self) {
        let index = self.get_n();
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let addr = 0xFF00 + index as u16;
        let val = self.bus.read_byte(addr);
        self.registers.a = val;
    }

    fn pop_rp2(&mut self, rp2: Rp2) {
        let val = self.get_nn();

        self.registers.set_r16(rp2, val, true);
    }

    fn ret(&mut self) {
        let addr = self.get_nn();
        self.registers.pc = addr;
    }

    fn reti(&mut self) {
        let addr = self.get_nn();
        self.registers.sp = self.registers.sp.wrapping_add(2);
        self.registers.pc = addr;

        self.ime = true;
    }

    fn jp_hl(&mut self) {
        self.registers.pc = self.registers.hl;
    }

    fn ld_sp_hl(&mut self) {
        self.registers.sp = self.registers.hl;
    }

    fn jp_cc_nn(&mut self, cc: Cc) {
        let val = self.flag_registers.get_c(cc);

        if val {
            let nn = self.get_nn();
            self.registers.pc = nn;
        }
    }

    fn ld_8c_a(&mut self) {
        let index = self.flag_registers.c;
        let addr = 0xFF00 + index as u16;
        self.bus.write_byte(addr, self.registers.a);
    }

    fn ld_nn_a(&mut self) {
        let addr = self.get_nn();
        let val = self.registers.a;
        self.bus.write_byte(addr, val);
    }

    fn ld_a_8c(&mut self) {
        let index = self.flag_registers.c;
        let addr = 0xFF00 + index as u16;
        let val = self.bus.read_byte(addr);
        self.registers.a = val;
    }

    fn ld_a_nn(&mut self) {
        let addr = self.get_nn();
        let val = self.bus.read_byte(addr);
        self.registers.a = val;
    }

    fn jp_nn(&mut self) {
        let addr = self.bus.read_word(self.registers.pc);
        self.registers.pc = addr;
    }

    fn di(&mut self) {
        self.ime = false;
    }

    fn ei(&mut self) {
        self.ime = true;
    }

    fn call_cc_nn(&mut self, cc: Cc) {
        let val = self.flag_registers.get_c(cc);

        if val {
            let addr = self.get_nn();
            self.bus.write_word(self.registers.sp, self.registers.pc);
            self.registers.pc = addr;
        }
    }

    fn push_rp2(&mut self, rp2: Rp2) {
        let val = self.registers.get_rp2(rp2, true);
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.bus.write_word(self.registers.sp, val);
    }

    fn call_nn(&mut self) {
        let addr = self.get_nn();
        self.call(addr);
    }

    fn add_a_n(&mut self) {
        let right = self.get_n();
        let left = self.registers.a;
        let result = left.wrapping_add(right);

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(false);
        self.flag_registers
            .set_h(self.half_carry_positive(left, right));
        self.flag_registers.set_c(self.carry_positive(left, right));
    }

    fn adc_a_n(&mut self) {
        let c = self.flag_registers.c as u8;
        let right = self.get_n();
        let left = self.registers.a;
        let result1 = left.wrapping_add(right);
        let result2 = result1.wrapping_add(c);

        let c1 = self.carry_positive(left, right);
        let h1 = self.half_carry_positive(left, right);
        let c2 = self.carry_positive(result1, c);
        let h2 = self.half_carry_positive(result1, c);

        self.registers.a = result2;

        self.flag_registers.set_z(result2 == 0);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(h1 || h2);
        self.flag_registers.set_c(c1 || c2);
    }

    fn sub_a_n(&mut self) {
        let left = self.registers.a;
        let right = self.get_n();
        let result = left.wrapping_sub(right);

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(true);
        self.flag_registers
            .set_h(self.half_carry_negative(left, right));
        self.flag_registers.set_c(self.carry_negative(left, right));
    }

    fn and_a_n(&mut self) {
        let left = self.registers.a;
        let right = self.get_n();
        let result = left & right;

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(true);
        self.flag_registers.set_c(false);
    }

    fn xor_a_n(&mut self) {
        let left = self.registers.a;
        let right = self.get_n();
        let result = left ^ right;

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(false);
    }

    fn or_a_n(&mut self) {
        let left = self.registers.a;
        let right = self.get_n();
        let result = left | right;

        self.registers.a = result;

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(false);
        self.flag_registers.set_h(false);
        self.flag_registers.set_c(false);
    }

    fn cp_a_n(&mut self) {
        let left = self.registers.a;
        let right = self.get_n();
        let result = left.wrapping_sub(right);

        self.flag_registers.set_z(result == 0);
        self.flag_registers.set_n(true);
        self.flag_registers
            .set_h(self.half_carry_negative(left, right));
        self.flag_registers.set_c(self.carry_negative(left, right));
    }

    fn rst(&mut self, addr: u8) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.bus.write_word(self.registers.sp, self.registers.pc);
        self.registers.pc = addr as u16;
    }

    // common
    fn half_carry_negative(&self, left: u8, right: u8) -> bool {
        (left & 0x0F) < (right & 0x0F)
    }

    fn carry_negative(&self, left: u8, right: u8) -> bool {
        left.overflowing_sub(right).1
    }

    fn carry_positive(&self, left: u8, right: u8) -> bool {
        left.overflowing_add(right).1
    }

    fn half_carry_positive(&self, left: u8, right: u8) -> bool {
        (left & 0x0F) + (right & 0x0F) > 0x0F
    }

    fn lh_hl_sp_d(&mut self) {
        let base_addr = self.registers.sp;
        let index_addr = self.get_n() as i8 as u16;

        self.registers.hl = base_addr.wrapping_add(index_addr);

        self.flag_registers.set_z(false);
        self.flag_registers.set_n(false);
        self.flag_registers
            .set_h(self.half_carry_positive(base_addr as u8, index_addr as u8));
        self.flag_registers
            .set_c(self.carry_positive(base_addr as u8, index_addr as u8));
    }

    fn call(&mut self, addr: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.bus.write_word(self.registers.sp, self.registers.pc);
        self.registers.pc = addr;
    }
}
