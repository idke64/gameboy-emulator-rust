use crate::cpu::CPU;

impl CPU {
    // increment cycles
    pub fn handle_cycles(&mut self, cycles: u32) {
        self.cycle += cycles as u32;
    }

    // get bit at position
    pub fn get_bit_at_position(byte: u8, position: u8) -> u8 {
        (byte >> position) & 1
    }

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
