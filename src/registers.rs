#[derive(Clone, Copy)]
pub enum Register {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    SP,
    PC,
    AF,
    BC,
    DE,
    HL,
}

pub struct Registers {
    pub a: u8,
    pub f: u8, // Z, N, H, C
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    // set registers to default resetted values
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn get_register8(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::F => self.f,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            _ => 0,
        }
    }

    pub fn get_register16(&self, reg: Register) -> u16 {
        match reg {
            Register::SP => self.sp,
            Register::PC => self.pc,
            Register::AF => self.get_af(),
            Register::BC => self.get_bc(),
            Register::DE => self.get_de(),
            Register::HL => self.get_hl(),
            _ => 0,
        }
    }

    pub fn set_register8(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::F => self.f = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            _ => (),
        }
    }

    pub fn set_register16(&mut self, reg: Register, value: u16) {
        match &reg {
            Register::SP => self.sp = value,
            Register::PC => self.pc = value,
            Register::AF => self.set_af(value),
            Register::BC => self.set_bc(value),
            Register::DE => self.set_de(value),
            Register::HL => self.set_hl(value),
            _ => (),
        }
    }

    // functions to read and write 16-bit registers

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value as u8) & 0xF0;
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn set_z_flag(&mut self, set: bool) {
        if set {
            self.f |= 0b1000_0000; // Set Z flag (bit 7)
        } else {
            self.f &= !0b1000_0000; // Clear Z flag
        }
    }

    pub fn set_s_flag(&mut self, set: bool) {
        if set {
            self.f |= 0b0100_0000; // Set N flag (bit 6)
        } else {
            self.f &= !0b0100_0000; // Clear N flag
        }
    }

    pub fn set_h_flag(&mut self, set: bool) {
        if set {
            self.f |= 0b0010_0000; // Set H flag (bit 5)
        } else {
            self.f &= !0b0010_0000; // Clear H flag
        }
    }

    pub fn set_c_flag(&mut self, set: bool) {
        if set {
            self.f |= 0b0001_0000; // Set C flag (bit 4)
        } else {
            self.f &= !0b0001_0000; // Clear C flag
        }
    }

    pub fn get_z_flag(&self) -> bool {
        (self.f & 0b1000_0000) != 0
    }

    pub fn get_s_flag(&self) -> bool {
        (self.f & 0b0100_0000) != 0
    }

    pub fn get_h_flag(&self) -> bool {
        (self.f & 0b0010_0000) != 0
    }

    pub fn get_c_flag(&self) -> bool {
        (self.f & 0b0001_0000) != 0
    }
}
