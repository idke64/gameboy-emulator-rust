use crate::cpu::CPU;
use crate::registers::Register;

impl CPU {
    pub fn rlc_8(&mut self, reg: Register) {
        let value = self.registers.get_register8(reg);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 7) == 1);

        self.registers.set_register8(reg, value.rotate_left(1));

        self.update_z_flag(self.registers.get_register8(reg));

        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn rl_8(&mut self, reg: Register) {
        let value = self.registers.get_register8(reg);

        let carry = self.registers.get_c_flag() as u8;

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 7) == 1);

        self.registers.set_register8(reg, (value << 1) | carry);

        self.update_z_flag(self.registers.get_register8(reg));

        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn rlc_mem16(&mut self, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 7) == 1);

        let result = value.rotate_left(1);
        self.memory.write_byte(addr, result);

        self.update_z_flag(result);

        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(12);
    }

    pub fn rl_mem16(&mut self, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        let carry = self.registers.get_c_flag() as u8;

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 7) == 1);

        let result = (value << 1) | carry;
        self.memory.write_byte(addr, result);

        self.update_z_flag(result);

        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(12);
    }

    pub fn rrc_8(&mut self, reg: Register) {
        let value = self.registers.get_register8(reg);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 0) == 1);

        let result = value.rotate_right(1);
        self.registers.set_register8(reg, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn rr_8(&mut self, reg: Register) {
        let value = self.registers.get_register8(reg);
        let carry = self.registers.get_c_flag() as u8;

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 0) == 1);

        let result = (value >> 1) | (carry << 7);
        self.registers.set_register8(reg, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn rrc_mem16(&mut self, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 0) == 1);

        let result = value.rotate_right(1);
        self.memory.write_byte(addr, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(12);
    }

    pub fn rr_mem16(&mut self, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);
        let carry = self.registers.get_c_flag() as u8;

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 0) == 1);

        let result = (value >> 1) | (carry << 7);
        self.memory.write_byte(addr, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(12);
    }

    pub fn sla_8(&mut self, reg: Register) {
        let value = self.registers.get_register8(reg);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 7) == 1);

        let result = value << 1;
        self.registers.set_register8(reg, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn sla_mem16(&mut self, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 7) == 1);

        let result = value << 1;
        self.memory.write_byte(addr, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(12);
    }

    pub fn sra_8(&mut self, reg: Register) {
        let value = self.registers.get_register8(reg);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 7) == 1);

        let result = (value >> 1) | (value & 0x80);
        self.registers.set_register8(reg, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn sra_mem16(&mut self, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 7) == 1);

        let result = (value >> 1) | (value & 0x80);
        self.memory.write_byte(addr, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(12);
    }

    pub fn swap_8(&mut self, reg: Register) {
        let value = self.registers.get_register8(reg);

        let result = (value >> 4) | (value << 4);

        self.registers.set_register8(reg, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);

        self.handle_cycles(4);
    }

    pub fn swap_mem16(&mut self, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        let result = (value >> 4) | (value << 4);

        self.memory.write_byte(addr, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);

        self.handle_cycles(12);
    }

    pub fn srl_8(&mut self, reg: Register) {
        let value = self.registers.get_register8(reg);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 0) == 1);

        let result = value >> 1;

        self.registers.set_register8(reg, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(8);
    }

    pub fn srl_mem16(&mut self, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        self.registers
            .set_c_flag(CPU::get_bit_at_position(value, 0) == 1);

        let result = value >> 1;

        self.memory.write_byte(addr, result);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(16);
    }

    pub fn bit_8(&mut self, bit: u8, reg: Register) {
        let value = self.registers.get_register8(reg);
        let bit_set = CPU::get_bit_at_position(value, bit);

        self.update_z_flag(bit_set);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(true);

        self.handle_cycles(4);
    }

    pub fn bit_mem16(&mut self, bit: u8, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        let bit_set = CPU::get_bit_at_position(value, bit);

        self.update_z_flag(bit_set);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(true);

        self.handle_cycles(12);
    }

    pub fn res_8(&mut self, bit: u8, reg: Register) {
        let value = self.registers.get_register8(reg);

        let result = value & !(1 << bit);

        self.registers.set_register8(reg, result);
        self.handle_cycles(4);
    }

    pub fn res_mem16(&mut self, bit: u8, reg: Register) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        let result = value & !(1 << bit);

        self.memory.write_byte(addr, result);
        self.handle_cycles(12);
    }

    pub fn set_8(&mut self, reg: Register, bit: u8) {
        let value = self.registers.get_register8(reg);

        let result = value | (1 << bit);

        self.registers.set_register8(reg, result);

        self.handle_cycles(4);
    }

    pub fn set_mem16(&mut self, reg: Register, bit: u8) {
        let addr = self.registers.get_register16(reg);
        let value = self.memory.read_byte(addr);

        let result = value | (1 << bit);

        self.memory.write_byte(addr, result);

        self.handle_cycles(12);
    }
}
