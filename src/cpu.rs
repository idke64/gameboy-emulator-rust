use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    device::SharedMemory,
    ppu::PPU,
    registers::{Register, Registers},
};

pub struct CPU {
    pub registers: Registers,
    pub memory: SharedMemory,
    pub ppu: Rc<RefCell<PPU>>,
    pub stopped: bool,
    pub cycle: u32,
    pub halted: bool,
    pub ime: bool,
}

impl CPU {
    pub fn new(memory: SharedMemory, ppu: Rc<RefCell<PPU>>) -> CPU {
        CPU {
            registers: Registers::new(),
            memory,
            ppu,
            stopped: false,
            cycle: 0,
            halted: false,
            ime: false,
        }
    }

    // fetches instruction from memory
    pub fn fetch_opcode(&mut self) -> u8 {
        let pc = self.registers.pc;

        let opcode = self.read_byte(pc);

        self.registers.pc = self.registers.pc.wrapping_add(1);

        opcode
    }

    pub fn execute_opcode(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_16_imm2(Register::BC),
            0x02 => self.ld_mem16_8(Register::BC, Register::A),
            0x03 => self.inc_16(Register::BC),
            0x04 => self.inc_8(Register::B),
            0x05 => self.dec_8(Register::B),
            0x06 => self.ld_8_imm1(Register::B),
            0x07 => self.rlca(),
            0x08 => self.ld_memimm2_sp(),
            0x09 => self.add_16_16(Register::HL, Register::BC),
            0x0A => self.ld_8_mem16(Register::A, Register::BC),
            0x0B => self.dec_16(Register::BC),
            0x0C => self.inc_8(Register::C),
            0x0D => self.dec_8(Register::C),
            0x0E => self.ld_8_imm1(Register::C),
            0x0F => self.rrca(),

            0x10 => self.stop(),
            0x11 => self.ld_16_imm2(Register::DE),
            0x12 => self.ld_mem16_8(Register::DE, Register::A),
            0x13 => self.inc_16(Register::DE),
            0x14 => self.inc_8(Register::D),
            0x15 => self.dec_8(Register::D),
            0x16 => self.ld_8_imm1(Register::D),
            0x17 => self.rla(),
            0x18 => self.jr_imm1(),
            0x19 => self.add_16_16(Register::HL, Register::DE),
            0x1A => self.ld_8_mem16(Register::A, Register::DE),
            0x1B => self.dec_16(Register::DE),
            0x1C => self.inc_8(Register::E),
            0x1D => self.dec_8(Register::E),
            0x1E => self.ld_8_imm1(Register::E),
            0x1F => self.rra(),

            0x20 => self.jr_nz_imm1(),
            0x21 => self.ld_16_imm2(Register::HL),
            0x22 => self.ld_mem16_8_inc_dec(Register::HL, Register::A, true),
            0x23 => self.inc_16(Register::HL),
            0x24 => self.inc_8(Register::H),
            0x25 => self.dec_8(Register::H),
            0x26 => self.ld_8_imm1(Register::H),
            0x27 => self.daa(),
            0x28 => self.jr_z_imm1(),
            0x29 => self.add_16_16(Register::HL, Register::HL),
            0x2A => self.ld_8_mem16_inc_dec(Register::A, Register::HL, true),
            0x2B => self.dec_16(Register::HL),
            0x2C => self.inc_8(Register::L),
            0x2D => self.dec_8(Register::L),
            0x2E => self.ld_8_imm1(Register::L),
            0x2F => self.cpl(),

            0x30 => self.jr_nc_imm1(),
            0x31 => self.ld_16_imm2(Register::SP),
            0x32 => self.ld_mem16_8_inc_dec(Register::HL, Register::A, false),
            0x33 => self.inc_16(Register::SP),
            0x34 => self.inc_8(Register::HL),
            0x35 => self.dec_8(Register::HL),
            0x36 => self.ld_mem16_imm1(Register::HL),
            0x37 => self.scf(),
            0x38 => self.jr_c_imm1(),
            0x39 => self.add_16_16(Register::HL, Register::SP),
            0x3A => self.ld_8_mem16_inc_dec(Register::A, Register::HL, false),
            0x3B => self.dec_16(Register::SP),
            0x3C => self.inc_8(Register::A),
            0x3D => self.dec_8(Register::A),
            0x3E => self.ld_8_imm1(Register::A),
            0x3F => self.ccf(),

            0x40 => self.ld_8_8(Register::B, Register::B),
            0x41 => self.ld_8_8(Register::B, Register::C),
            0x42 => self.ld_8_8(Register::B, Register::D),
            0x43 => self.ld_8_8(Register::B, Register::E),
            0x44 => self.ld_8_8(Register::B, Register::H),
            0x45 => self.ld_8_8(Register::B, Register::L),
            0x46 => self.ld_8_mem16(Register::B, Register::HL),
            0x47 => self.ld_8_8(Register::B, Register::A),
            0x48 => self.ld_8_8(Register::C, Register::B),
            0x49 => self.ld_8_8(Register::C, Register::C),
            0x4A => self.ld_8_8(Register::C, Register::D),
            0x4B => self.ld_8_8(Register::C, Register::E),
            0x4C => self.ld_8_8(Register::C, Register::H),
            0x4D => self.ld_8_8(Register::C, Register::L),
            0x4E => self.ld_8_mem16(Register::C, Register::HL),
            0x4F => self.ld_8_8(Register::C, Register::A),

            0x50 => self.ld_8_8(Register::D, Register::B),
            0x51 => self.ld_8_8(Register::D, Register::C),
            0x52 => self.ld_8_8(Register::D, Register::D),
            0x53 => self.ld_8_8(Register::D, Register::E),
            0x54 => self.ld_8_8(Register::D, Register::H),
            0x55 => self.ld_8_8(Register::D, Register::L),
            0x56 => self.ld_8_mem16(Register::D, Register::HL),
            0x57 => self.ld_8_8(Register::D, Register::A),
            0x58 => self.ld_8_8(Register::E, Register::B),
            0x59 => self.ld_8_8(Register::E, Register::C),
            0x5A => self.ld_8_8(Register::E, Register::D),
            0x5B => self.ld_8_8(Register::E, Register::E),
            0x5C => self.ld_8_8(Register::E, Register::H),
            0x5D => self.ld_8_8(Register::E, Register::L),
            0x5E => self.ld_8_mem16(Register::E, Register::HL),
            0x5F => self.ld_8_8(Register::E, Register::A),

            0x60 => self.ld_8_8(Register::H, Register::B),
            0x61 => self.ld_8_8(Register::H, Register::C),
            0x62 => self.ld_8_8(Register::H, Register::D),
            0x63 => self.ld_8_8(Register::H, Register::E),
            0x64 => self.ld_8_8(Register::H, Register::H),
            0x65 => self.ld_8_8(Register::H, Register::L),
            0x66 => self.ld_8_mem16(Register::H, Register::HL),
            0x67 => self.ld_8_8(Register::H, Register::A),
            0x68 => self.ld_8_8(Register::L, Register::B),
            0x69 => self.ld_8_8(Register::L, Register::C),
            0x6A => self.ld_8_8(Register::L, Register::D),
            0x6B => self.ld_8_8(Register::L, Register::E),
            0x6C => self.ld_8_8(Register::L, Register::H),
            0x6D => self.ld_8_8(Register::L, Register::L),
            0x6E => self.ld_8_mem16(Register::L, Register::HL),
            0x6F => self.ld_8_8(Register::L, Register::A),

            0x70 => self.ld_mem16_8(Register::HL, Register::B),
            0x71 => self.ld_mem16_8(Register::HL, Register::C),
            0x72 => self.ld_mem16_8(Register::HL, Register::D),
            0x73 => self.ld_mem16_8(Register::HL, Register::E),
            0x74 => self.ld_mem16_8(Register::HL, Register::H),
            0x75 => self.ld_mem16_8(Register::HL, Register::L),
            0x76 => self.halt(),
            0x77 => self.ld_mem16_8(Register::HL, Register::A),
            0x78 => self.ld_8_8(Register::A, Register::B),
            0x79 => self.ld_8_8(Register::A, Register::C),
            0x7A => self.ld_8_8(Register::A, Register::D),
            0x7B => self.ld_8_8(Register::A, Register::E),
            0x7C => self.ld_8_8(Register::A, Register::H),
            0x7D => self.ld_8_8(Register::A, Register::L),
            0x7E => self.ld_8_mem16(Register::A, Register::HL),
            0x7F => self.ld_8_8(Register::A, Register::A),

            0x80 => self.add_8_8(Register::A, Register::B),
            0x81 => self.add_8_8(Register::A, Register::C),
            0x82 => self.add_8_8(Register::A, Register::D),
            0x83 => self.add_8_8(Register::A, Register::E),
            0x84 => self.add_8_8(Register::A, Register::H),
            0x85 => self.add_8_8(Register::A, Register::L),
            0x86 => self.add_8_mem16(Register::A, Register::HL),
            0x87 => self.add_8_8(Register::A, Register::A),
            0x88 => self.adc_8_8(Register::A, Register::B),
            0x89 => self.adc_8_8(Register::A, Register::C),
            0x8A => self.adc_8_8(Register::A, Register::D),
            0x8B => self.adc_8_8(Register::A, Register::E),
            0x8C => self.adc_8_8(Register::A, Register::H),
            0x8D => self.adc_8_8(Register::A, Register::L),
            0x8E => self.adc_8_mem16(Register::A, Register::HL),
            0x8F => self.adc_8_8(Register::A, Register::A),

            0x90 => self.sub_8_8(Register::A, Register::B),
            0x91 => self.sub_8_8(Register::A, Register::C),
            0x92 => self.sub_8_8(Register::A, Register::D),
            0x93 => self.sub_8_8(Register::A, Register::E),
            0x94 => self.sub_8_8(Register::A, Register::H),
            0x95 => self.sub_8_8(Register::A, Register::L),
            0x96 => self.sub_8_mem16(Register::A, Register::HL),
            0x97 => self.sub_8_8(Register::A, Register::A),
            0x98 => self.sbc_8_8(Register::A, Register::B),
            0x99 => self.sbc_8_8(Register::A, Register::C),
            0x9A => self.sbc_8_8(Register::A, Register::D),
            0x9B => self.sbc_8_8(Register::A, Register::E),
            0x9C => self.sbc_8_8(Register::A, Register::H),
            0x9D => self.sbc_8_8(Register::A, Register::L),
            0x9E => self.sbc_8_mem16(Register::A, Register::HL),
            0x9F => self.sbc_8_8(Register::A, Register::A),

            0xA0 => self.and_8_8(Register::A, Register::B),
            0xA1 => self.and_8_8(Register::A, Register::C),
            0xA2 => self.and_8_8(Register::A, Register::D),
            0xA3 => self.and_8_8(Register::A, Register::E),
            0xA4 => self.and_8_8(Register::A, Register::H),
            0xA5 => self.and_8_8(Register::A, Register::L),
            0xA6 => self.and_8_mem16(Register::A, Register::HL),
            0xA7 => self.and_8_8(Register::A, Register::A),
            0xA8 => self.xor_8_8(Register::A, Register::B),
            0xA9 => self.xor_8_8(Register::A, Register::C),
            0xAA => self.xor_8_8(Register::A, Register::D),
            0xAB => self.xor_8_8(Register::A, Register::E),
            0xAC => self.xor_8_8(Register::A, Register::H),
            0xAD => self.xor_8_8(Register::A, Register::L),
            0xAE => self.xor_8_mem16(Register::A, Register::HL),
            0xAF => self.xor_8_8(Register::A, Register::A),

            0xB0 => self.or_8_8(Register::A, Register::B),
            0xB1 => self.or_8_8(Register::A, Register::C),
            0xB2 => self.or_8_8(Register::A, Register::D),
            0xB3 => self.or_8_8(Register::A, Register::E),
            0xB4 => self.or_8_8(Register::A, Register::H),
            0xB5 => self.or_8_8(Register::A, Register::L),
            0xB6 => self.or_8_mem16(Register::A, Register::HL),
            0xB7 => self.or_8_8(Register::A, Register::A),
            0xB8 => self.cp_8_8(Register::A, Register::B),
            0xB9 => self.cp_8_8(Register::A, Register::C),
            0xBA => self.cp_8_8(Register::A, Register::D),
            0xBB => self.cp_8_8(Register::A, Register::E),
            0xBC => self.cp_8_8(Register::A, Register::H),
            0xBD => self.cp_8_8(Register::A, Register::L),
            0xBE => self.cp_8_mem16(Register::A, Register::HL),
            0xBF => self.cp_8_8(Register::A, Register::A),

            0xC0 => self.ret_nz(),
            0xC1 => self.pop_16(Register::BC),
            0xC2 => self.jp_nz_imm2(),
            0xC3 => self.jp_imm2(),
            0xC4 => self.call_nz_imm2(),
            0xC5 => self.push_16(Register::BC),
            0xC6 => self.add_8_imm1(Register::A),
            0xC7 => self.rst(0x00),
            0xC8 => self.ret_z(),
            0xC9 => self.ret(),
            0xCA => self.jp_z_imm2(),
            0xCB => self.prefix_cb(),
            0xCC => self.call_z_imm2(),
            0xCD => self.call_imm2(),
            0xCE => self.adc_8_imm1(Register::A),
            0xCF => self.rst(0x08),

            0xD0 => self.ret_nc(),
            0xD1 => self.pop_16(Register::DE),
            0xD2 => self.jp_nc_imm2(),
            0xD3 => (),
            0xD4 => self.call_nc_imm2(),
            0xD5 => self.push_16(Register::DE),
            0xD6 => self.sub_8_imm1(Register::A),
            0xD7 => self.rst(0x10),
            0xD8 => self.ret_c(),
            0xD9 => self.reti(),
            0xDA => self.jp_c_imm2(),
            0xDB => (),
            0xDC => self.call_c_imm2(),
            0xDD => (),
            0xDE => self.sbc_8_imm1(Register::A),
            0xDF => self.rst(0x18),

            0xE0 => self.ldh_memimm1_8(Register::A),
            0xE1 => self.pop_16(Register::HL),
            0xE2 => self.ld_mem8_8(Register::C, Register::A),
            0xE3 => (),
            0xE4 => (),
            0xE5 => self.push_16(Register::HL),
            0xE6 => self.and_8_imm1(Register::A),
            0xE7 => self.rst(0x20),
            0xE8 => self.add_16_imm1(Register::SP),
            0xE9 => self.jp_mem16(Register::HL),
            0xEA => self.ld_memimm2_8(Register::A),
            0xEB => (),
            0xEC => (),
            0xED => (),
            0xEE => self.xor_8_imm1(Register::A),
            0xEF => self.rst(0x28),

            0xF0 => self.ldh_8_memimm1(Register::A),
            0xF1 => self.pop_16(Register::AF),
            0xF2 => self.ld_8_mem8(Register::A, Register::C),
            0xF3 => self.di(),
            0xF4 => (),
            0xF5 => self.push_16(Register::AF),
            0xF6 => self.or_8_imm1(Register::A),
            0xF7 => self.rst(0x30),
            0xF8 => self.ld_hl_sp_r8(),
            0xF9 => self.ld_16_16(Register::SP, Register::HL),
            0xFA => self.ld_8_memimm2(Register::A),
            0xFB => self.ei(),
            0xFC => (),
            0xFD => (),
            0xFE => self.cp_8_imm1(Register::A),
            0xFF => self.rst(0x38),
            _ => (),
        }
    }

    pub fn execute_cb_opcode(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.rlc_8(Register::B),
            0x01 => self.rlc_8(Register::C),
            0x02 => self.rlc_8(Register::D),
            0x03 => self.rlc_8(Register::E),
            0x04 => self.rlc_8(Register::H),
            0x05 => self.rlc_8(Register::L),
            0x06 => self.rlc_mem16(Register::HL),
            0x07 => self.rlc_8(Register::A),
            0x08 => self.rrc_8(Register::B),
            0x09 => self.rrc_8(Register::C),
            0x0A => self.rrc_8(Register::D),
            0x0B => self.rrc_8(Register::E),
            0x0C => self.rrc_8(Register::H),
            0x0D => self.rrc_8(Register::L),
            0x0E => self.rrc_mem16(Register::HL),
            0x0F => self.rrc_8(Register::A),

            0x10 => self.rl_8(Register::B),
            0x11 => self.rl_8(Register::C),
            0x12 => self.rl_8(Register::D),
            0x13 => self.rl_8(Register::E),
            0x14 => self.rl_8(Register::H),
            0x15 => self.rl_8(Register::L),
            0x16 => self.rl_mem16(Register::HL),
            0x17 => self.rl_8(Register::A),
            0x18 => self.rr_8(Register::B),
            0x19 => self.rr_8(Register::C),
            0x1A => self.rr_8(Register::D),
            0x1B => self.rr_8(Register::E),
            0x1C => self.rr_8(Register::H),
            0x1D => self.rr_8(Register::L),
            0x1E => self.rr_mem16(Register::HL),
            0x1F => self.rr_8(Register::A),

            0x20 => self.sla_8(Register::B),
            0x21 => self.sla_8(Register::C),
            0x22 => self.sla_8(Register::D),
            0x23 => self.sla_8(Register::E),
            0x24 => self.sla_8(Register::H),
            0x25 => self.sla_8(Register::L),
            0x26 => self.sla_mem16(Register::HL),
            0x27 => self.sla_8(Register::A),
            0x28 => self.sra_8(Register::B),
            0x29 => self.sra_8(Register::C),
            0x2A => self.sra_8(Register::D),
            0x2B => self.sra_8(Register::E),
            0x2C => self.sra_8(Register::H),
            0x2D => self.sra_8(Register::L),
            0x2E => self.sra_mem16(Register::HL),
            0x2F => self.sra_8(Register::A),

            0x30 => self.swap_8(Register::B),
            0x31 => self.swap_8(Register::C),
            0x32 => self.swap_8(Register::D),
            0x33 => self.swap_8(Register::E),
            0x34 => self.swap_8(Register::H),
            0x35 => self.swap_8(Register::L),
            0x36 => self.swap_mem16(Register::HL),
            0x37 => self.swap_8(Register::A),
            0x38 => self.srl_8(Register::B),
            0x39 => self.srl_8(Register::C),
            0x3A => self.srl_8(Register::D),
            0x3B => self.srl_8(Register::E),
            0x3C => self.srl_8(Register::H),
            0x3D => self.srl_8(Register::L),
            0x3E => self.srl_mem16(Register::HL),
            0x3F => self.srl_8(Register::A),

            0x40 => self.bit_8(0, Register::B),
            0x41 => self.bit_8(0, Register::C),
            0x42 => self.bit_8(0, Register::D),
            0x43 => self.bit_8(0, Register::E),
            0x44 => self.bit_8(0, Register::H),
            0x45 => self.bit_8(0, Register::L),
            0x46 => self.bit_mem16(0, Register::HL),
            0x47 => self.bit_8(0, Register::A),
            0x48 => self.bit_8(1, Register::B),
            0x49 => self.bit_8(1, Register::C),
            0x4A => self.bit_8(1, Register::D),
            0x4B => self.bit_8(1, Register::E),
            0x4C => self.bit_8(1, Register::H),
            0x4D => self.bit_8(1, Register::L),
            0x4E => self.bit_mem16(1, Register::HL),
            0x4F => self.bit_8(1, Register::A),

            0x50 => self.bit_8(2, Register::B),
            0x51 => self.bit_8(2, Register::C),
            0x52 => self.bit_8(2, Register::D),
            0x53 => self.bit_8(2, Register::E),
            0x54 => self.bit_8(2, Register::H),
            0x55 => self.bit_8(2, Register::L),
            0x56 => self.bit_mem16(2, Register::HL),
            0x57 => self.bit_8(2, Register::A),
            0x58 => self.bit_8(3, Register::B),
            0x59 => self.bit_8(3, Register::C),
            0x5A => self.bit_8(3, Register::D),
            0x5B => self.bit_8(3, Register::E),
            0x5C => self.bit_8(3, Register::H),
            0x5D => self.bit_8(3, Register::L),
            0x5E => self.bit_mem16(3, Register::HL),
            0x5F => self.bit_8(3, Register::A),

            0x60 => self.bit_8(4, Register::B),
            0x61 => self.bit_8(4, Register::C),
            0x62 => self.bit_8(4, Register::D),
            0x63 => self.bit_8(4, Register::E),
            0x64 => self.bit_8(4, Register::H),
            0x65 => self.bit_8(4, Register::L),
            0x66 => self.bit_mem16(4, Register::HL),
            0x67 => self.bit_8(4, Register::A),
            0x68 => self.bit_8(5, Register::B),
            0x69 => self.bit_8(5, Register::C),
            0x6A => self.bit_8(5, Register::D),
            0x6B => self.bit_8(5, Register::E),
            0x6C => self.bit_8(5, Register::H),
            0x6D => self.bit_8(5, Register::L),
            0x6E => self.bit_mem16(5, Register::HL),
            0x6F => self.bit_8(5, Register::A),

            0x70 => self.bit_8(6, Register::B),
            0x71 => self.bit_8(6, Register::C),
            0x72 => self.bit_8(6, Register::D),
            0x73 => self.bit_8(6, Register::E),
            0x74 => self.bit_8(6, Register::H),
            0x75 => self.bit_8(6, Register::L),
            0x76 => self.bit_mem16(6, Register::HL),
            0x77 => self.bit_8(6, Register::A),
            0x78 => self.bit_8(7, Register::B),
            0x79 => self.bit_8(7, Register::C),
            0x7A => self.bit_8(7, Register::D),
            0x7B => self.bit_8(7, Register::E),
            0x7C => self.bit_8(7, Register::H),
            0x7D => self.bit_8(7, Register::L),
            0x7E => self.bit_mem16(7, Register::HL),
            0x7F => self.bit_8(7, Register::A),

            0x80 => self.res_8(0, Register::B),
            0x81 => self.res_8(0, Register::C),
            0x82 => self.res_8(0, Register::D),
            0x83 => self.res_8(0, Register::E),
            0x84 => self.res_8(0, Register::H),
            0x85 => self.res_8(0, Register::L),
            0x86 => self.res_mem16(0, Register::HL),
            0x87 => self.res_8(0, Register::A),
            0x88 => self.res_8(1, Register::B),
            0x89 => self.res_8(1, Register::C),
            0x8A => self.res_8(1, Register::D),
            0x8B => self.res_8(1, Register::E),
            0x8C => self.res_8(1, Register::H),
            0x8D => self.res_8(1, Register::L),
            0x8E => self.res_mem16(1, Register::HL),
            0x8F => self.res_8(1, Register::A),

            0x90 => self.res_8(2, Register::B),
            0x91 => self.res_8(2, Register::C),
            0x92 => self.res_8(2, Register::D),
            0x93 => self.res_8(2, Register::E),
            0x94 => self.res_8(2, Register::H),
            0x95 => self.res_8(2, Register::L),
            0x96 => self.res_mem16(2, Register::HL),
            0x97 => self.res_8(2, Register::A),
            0x98 => self.res_8(3, Register::B),
            0x99 => self.res_8(3, Register::C),
            0x9A => self.res_8(3, Register::D),
            0x9B => self.res_8(3, Register::E),
            0x9C => self.res_8(3, Register::H),
            0x9D => self.res_8(3, Register::L),
            0x9E => self.res_mem16(3, Register::HL),
            0x9F => self.res_8(3, Register::A),

            0xA0 => self.res_8(4, Register::B),
            0xA1 => self.res_8(4, Register::C),
            0xA2 => self.res_8(4, Register::D),
            0xA3 => self.res_8(4, Register::E),
            0xA4 => self.res_8(4, Register::H),
            0xA5 => self.res_8(4, Register::L),
            0xA6 => self.res_mem16(4, Register::HL),
            0xA7 => self.res_8(4, Register::A),
            0xA8 => self.res_8(5, Register::B),
            0xA9 => self.res_8(5, Register::C),
            0xAA => self.res_8(5, Register::D),
            0xAB => self.res_8(5, Register::E),
            0xAC => self.res_8(5, Register::H),
            0xAD => self.res_8(5, Register::L),
            0xAE => self.res_mem16(5, Register::HL),
            0xAF => self.res_8(5, Register::A),

            0xB0 => self.res_8(6, Register::B),
            0xB1 => self.res_8(6, Register::C),
            0xB2 => self.res_8(6, Register::D),
            0xB3 => self.res_8(6, Register::E),
            0xB4 => self.res_8(6, Register::H),
            0xB5 => self.res_8(6, Register::L),
            0xB6 => self.res_mem16(6, Register::HL),
            0xB7 => self.res_8(6, Register::A),
            0xB8 => self.res_8(7, Register::B),
            0xB9 => self.res_8(7, Register::C),
            0xBA => self.res_8(7, Register::D),
            0xBB => self.res_8(7, Register::E),
            0xBC => self.res_8(7, Register::H),
            0xBD => self.res_8(7, Register::L),
            0xBE => self.res_mem16(7, Register::HL),
            0xBF => self.res_8(7, Register::A),

            0xC0 => self.set_8(Register::B, 0),
            0xC1 => self.set_8(Register::C, 0),
            0xC2 => self.set_8(Register::D, 0),
            0xC3 => self.set_8(Register::E, 0),
            0xC4 => self.set_8(Register::H, 0),
            0xC5 => self.set_8(Register::L, 0),
            0xC6 => self.set_mem16(Register::HL, 0),
            0xC7 => self.set_8(Register::A, 0),
            0xC8 => self.set_8(Register::B, 1),
            0xC9 => self.set_8(Register::C, 1),
            0xCA => self.set_8(Register::D, 1),
            0xCB => self.set_8(Register::E, 1),
            0xCC => self.set_8(Register::H, 1),
            0xCD => self.set_8(Register::L, 1),
            0xCE => self.set_mem16(Register::HL, 1),
            0xCF => self.set_8(Register::A, 1),

            0xD0 => self.set_8(Register::B, 2),
            0xD1 => self.set_8(Register::C, 2),
            0xD2 => self.set_8(Register::D, 2),
            0xD3 => self.set_8(Register::E, 2),
            0xD4 => self.set_8(Register::H, 2),
            0xD5 => self.set_8(Register::L, 2),
            0xD6 => self.set_mem16(Register::HL, 2),
            0xD7 => self.set_8(Register::A, 2),
            0xD8 => self.set_8(Register::B, 3),
            0xD9 => self.set_8(Register::C, 3),
            0xDA => self.set_8(Register::D, 3),
            0xDB => self.set_8(Register::E, 3),
            0xDC => self.set_8(Register::H, 3),
            0xDD => self.set_8(Register::L, 3),
            0xDE => self.set_mem16(Register::HL, 3),
            0xDF => self.set_8(Register::A, 3),

            0xE0 => self.set_8(Register::B, 4),
            0xE1 => self.set_8(Register::C, 4),
            0xE2 => self.set_8(Register::D, 4),
            0xE3 => self.set_8(Register::E, 4),
            0xE4 => self.set_8(Register::H, 4),
            0xE5 => self.set_8(Register::L, 4),
            0xE6 => self.set_mem16(Register::HL, 4),
            0xE7 => self.set_8(Register::A, 4),
            0xE8 => self.set_8(Register::B, 5),
            0xE9 => self.set_8(Register::C, 5),
            0xEA => self.set_8(Register::D, 5),
            0xEB => self.set_8(Register::E, 5),
            0xEC => self.set_8(Register::H, 5),
            0xED => self.set_8(Register::L, 5),
            0xEE => self.set_mem16(Register::HL, 5),
            0xEF => self.set_8(Register::A, 5),

            0xF0 => self.set_8(Register::B, 6),
            0xF1 => self.set_8(Register::C, 6),
            0xF2 => self.set_8(Register::D, 6),
            0xF3 => self.set_8(Register::E, 6),
            0xF4 => self.set_8(Register::H, 6),
            0xF5 => self.set_8(Register::L, 6),
            0xF6 => self.set_mem16(Register::HL, 6),
            0xF7 => self.set_8(Register::A, 6),
            0xF8 => self.set_8(Register::B, 7),
            0xF9 => self.set_8(Register::C, 7),
            0xFA => self.set_8(Register::D, 7),
            0xFB => self.set_8(Register::E, 7),
            0xFC => self.set_8(Register::H, 7),
            0xFD => self.set_8(Register::L, 7),
            0xFE => self.set_mem16(Register::HL, 7),
            0xFF => self.set_8(Register::A, 7),
            _ => (),
        }
    }

    pub fn cycle(&mut self) {
        let cycles_per_frame = 70224;

        while self.cycle < cycles_per_frame {
            self.handle_interrupt();
            let opcode = self.fetch_opcode();
            self.execute_opcode(opcode);
        }

        self.cycle -= cycles_per_frame;
    }

    pub fn handle_interrupt(&mut self) {
        if self.ime {
            let interrupt_enable = self.memory.borrow().interrupt_enable;
            let interrupt_flags = self.read_byte(0xFF0F);

            let requested = interrupt_enable & interrupt_flags;

            if requested != 0 {
                if self.halted {
                    self.halted = false;
                }

                let interrupt_bit = requested.trailing_zeros() as u8;
                let interrupt_vector = match interrupt_bit {
                    0 => 0x0040,
                    1 => 0x0048,
                    2 => 0x0050,
                    3 => 0x0058,
                    4 => 0x0060,
                    _ => return,
                };

                self.write_byte(0xFF0F, interrupt_flags & !(1 << interrupt_bit));
                self.ime = false;
                let pc = self.registers.pc;
                self.push_stack(pc);
                self.registers.pc = interrupt_vector;

                self.handle_cycles(20);
            }
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let byte = self.memory.borrow().read_byte(addr);
        byte
    }

    pub fn write_byte(&self, addr: u16, value: u8) {
        self.memory.borrow_mut().write_byte(addr, value);
    }

    pub fn push_stack(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, (value & 0xFF) as u8);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, (value >> 8) as u8);
    }

    pub fn pop_stack(&mut self) -> u16 {
        let high = self.read_byte(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let low = self.read_byte(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);
        (high << 8) | low
    }

    pub fn handle_cycles(&mut self, cycles: u32) {
        self.cycle += cycles;
        self.ppu.borrow_mut().step(cycles);
    }

    // Get bit at position
    pub fn get_bit_at_position(byte: u8, position: u8) -> u8 {
        (byte >> position) & 1
    }

    // Update z flag based on its expected
    pub fn update_z_flag(&mut self, result: u8) {
        self.registers.set_z_flag(result == 0);
    }

    pub fn update_s_flag(&mut self, is_subtraction: bool) {
        self.registers.set_s_flag(is_subtraction);
    }

    pub fn update_c_flag8(&mut self, operand1: u8, operand2: u8, is_subtraction: bool) {
        let carry = if is_subtraction {
            operand1 < operand2
        } else {
            (operand1 as u16 + operand2 as u16) & 0x100 != 0
        };

        self.registers.set_c_flag(carry);
    }

    pub fn update_h_flag8(&mut self, operand1: u8, operand2: u8, is_subtraction: bool) {
        let half_carry = if is_subtraction {
            (operand1 & 0x0F) < (operand2 & 0x0F)
        } else {
            ((operand1 & 0x0F) + (operand2 & 0x0F)) & 0x10 != 0
        };

        self.registers.set_h_flag(half_carry);
    }

    pub fn update_c_flag16(&mut self, operand1: u16, operand2: u16, is_subtraction: bool) {
        let carry = if is_subtraction {
            operand1 < operand2
        } else {
            (operand1 as u32 + operand2 as u32) & 0x10000 != 0
        };

        self.registers.set_c_flag(carry);
    }

    pub fn update_h_flag16(&mut self, operand1: u16, operand2: u16, is_subtraction: bool) {
        let half_carry = if is_subtraction {
            (operand1 & 0x0FFF) < (operand2 & 0x0FFF)
        } else {
            ((operand1 & 0x0FFF) + (operand2 & 0x0FFF)) & 0x1000 != 0
        };

        self.registers.set_h_flag(half_carry);
    }

    pub fn add(&mut self, op1: u8, op2: u8) -> u8 {
        let result = op1.wrapping_add(op2);
        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.update_h_flag8(op1, op2, false);
        self.update_c_flag8(op1, op2, false);
        result
    }

    pub fn sub(&mut self, op1: u8, op2: u8) -> u8 {
        let result = op1.wrapping_sub(op2);
        self.update_z_flag(result);
        self.registers.set_s_flag(true);
        self.update_h_flag8(op1, op2, true);
        self.update_c_flag8(op1, op2, true);
        result
    }

    pub fn and(&mut self, op1: u8, op2: u8) -> u8 {
        let result = op1 & op2;
        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(true);
        self.registers.set_c_flag(false);
        result
    }

    pub fn or(&mut self, op1: u8, op2: u8) -> u8 {
        let result = op1 | op2;
        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);
        result
    }

    pub fn adc(&mut self, op1: u8, op2: u8) -> u8 {
        let carry = if self.registers.get_c_flag() { 1 } else { 0 };
        let result = op1.wrapping_add(op2).wrapping_add(carry);
        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.update_h_flag8(op1, op2.wrapping_add(carry), false);
        self.update_c_flag8(op1, op2.wrapping_add(carry), false);
        result
    }

    pub fn sbc(&mut self, op1: u8, op2: u8) -> u8 {
        let carry = if self.registers.get_c_flag() { 1 } else { 0 };
        let result = op1.wrapping_sub(op2).wrapping_sub(carry);
        self.update_z_flag(result);
        self.registers.set_s_flag(true);
        self.update_h_flag8(op1, op2.wrapping_add(carry), true);
        self.update_c_flag8(op1, op2.wrapping_add(carry), true);
        result
    }

    pub fn xor(&mut self, op1: u8, op2: u8) -> u8 {
        let result = op1 ^ op2;
        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);
        result
    }

    pub fn cp(&mut self, op1: u8, op2: u8) -> u8 {
        let result = op1.wrapping_sub(op2);
        self.update_z_flag(result);
        self.registers.set_s_flag(true);
        self.update_h_flag8(op1, op2, true);
        self.update_c_flag8(op1, op2, true);
        result
    }
}
