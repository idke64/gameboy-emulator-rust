use crate::cpu::CPU;
use crate::registers::Register;

impl CPU {
    pub fn nop(&mut self) {
        self.handle_cycles(4);
    }

    pub fn stop(&mut self) {
        self.stopped = true;
        self.handle_cycles(4);
    }

    pub fn ld_8_8(&mut self, dest: Register, src: Register) {
        let value = self.registers.get_register8(src);
        self.registers.set_register8(dest, value);
        self.handle_cycles(4);
    }

    pub fn ld_8_imm1(&mut self, dest: Register) {
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        self.registers.set_register8(dest, next_byte);
        self.handle_cycles(8);
    }

    pub fn ld_16_imm2(&mut self, dest: Register) {
        let next_byte1 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        let next_byte2 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let word = ((next_byte2 as u16) << 8) | (next_byte1 as u16);

        self.registers.set_register16(dest, word);
        self.handle_cycles(12);
    }

    pub fn ld_mem16_8(&mut self, reg16: Register, src: Register) {
        let addr = self.registers.get_register16(reg16);
        let value = self.registers.get_register8(src);
        self.memory.write_byte(addr, value);
        self.handle_cycles(8);
    }

    pub fn ld_8_mem16(&mut self, dest: Register, reg16: Register) {
        let addr = self.registers.get_register16(reg16);
        let byte = self.memory.read_byte(addr);
        self.registers.set_register8(dest, byte);
        self.handle_cycles(8);
    }

    pub fn ld_mem16_8_inc_dec(&mut self, reg16: Register, src: Register, increment: bool) {
        self.ld_mem16_8(reg16, src);
        let reg16_value = self.registers.get_register16(reg16);
        if increment {
            self.registers
                .set_register16(reg16, reg16_value.wrapping_add(1));
        } else {
            self.registers
                .set_register16(reg16, reg16_value.wrapping_sub(1));
        }
    }

    pub fn ld_8_mem16_inc_dec(&mut self, dest: Register, reg16: Register, increment: bool) {
        self.ld_8_mem16(dest, reg16);
        let reg16_value = self.registers.get_register16(reg16);
        if increment {
            self.registers
                .set_register16(reg16, reg16_value.wrapping_add(1));
        } else {
            self.registers
                .set_register16(reg16, reg16_value.wrapping_sub(1));
        }
    }

    pub fn ld_memimm2_sp(&mut self) {
        let next_byte1 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        let next_byte2 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let word = ((next_byte2 as u16) << 8) | (next_byte1 as u16);

        let sp = self.registers.sp;

        self.memory.write_byte(word, (sp & 0x00FF) as u8);
        self.memory
            .write_byte(word.wrapping_add(1), (sp >> 8) as u8);

        self.handle_cycles(20);
    }

    pub fn ld_mem8_8(&mut self, reg8: Register, src: Register) {
        let offset = self.registers.get_register8(reg8) as u16;
        let addr = 0xFF00 + offset;
        let value = self.registers.get_register8(src);
        self.memory.write_byte(addr, value);
        self.handle_cycles(8);
    }

    pub fn ld_8_mem8(&mut self, dest: Register, reg8: Register) {
        let offset = self.registers.get_register8(reg8) as u16;
        let addr = 0xFF00 + offset;
        let value = self.memory.read_byte(addr);
        self.registers.set_register8(dest, value);
        self.handle_cycles(8);
    }

    pub fn ld_memimm2_8(&mut self, src: Register) {
        let next_byte1 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        let next_byte2 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let addr = ((next_byte2 as u16) << 8) | (next_byte1 as u16);

        let value = self.registers.get_register8(src);

        self.memory.write_byte(addr, value);
        self.handle_cycles(16);
    }

    pub fn ld_8_memimm2(&mut self, dest: Register) {
        let next_byte1 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        let next_byte2 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let addr = ((next_byte2 as u16) << 8) | (next_byte1 as u16);

        let value = self.memory.read_byte(addr);

        self.registers.set_register8(dest, value);
        self.handle_cycles(16);
    }

    pub fn ld_16_16(&mut self, dest: Register, src: Register) {
        let value = self.registers.get_register16(src);
        self.registers.set_register16(dest, value);
        self.handle_cycles(8);
    }

    pub fn ld_hl_sp_r8(&mut self) {
        let r8 = self.memory.read_byte(self.registers.pc) as i8;
        self.registers.pc += 1;

        let sp = self.registers.sp;
        let result = sp.wrapping_add(r8 as u16);

        self.registers.set_hl(result);

        self.registers.set_z_flag(false);
        self.registers.set_s_flag(false);
        self.update_h_flag8(sp as u8, r8 as u8, false);
        self.update_c_flag8(sp as u8, r8 as u8, false);

        self.handle_cycles(12);
    }

    pub fn ldh_memimm1_8(&mut self, src: Register) {
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let addr = 0xFF00 + next_byte as u16;
        let value = self.registers.get_register8(src);
        self.memory.write_byte(addr, value);

        self.handle_cycles(12);
    }

    pub fn ldh_8_memimm1(&mut self, dest: Register) {
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let addr = 0xFF00 + next_byte as u16;
        let value = self.memory.read_byte(addr);
        self.registers.set_register8(dest, value);

        self.handle_cycles(12);
    }

    pub fn ld_mem16_imm1(&mut self, reg16: Register) {
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let addr = self.registers.get_register16(reg16);
        self.memory.write_byte(addr, next_byte);

        self.handle_cycles(12);
    }

    pub fn inc_16(&mut self, reg16: Register) {
        self.registers
            .set_register16(reg16, self.registers.get_register16(reg16).wrapping_add(1));
        self.handle_cycles(8);
    }

    pub fn inc_8(&mut self, reg8: Register) {
        let reg8_value = self.registers.get_register8(reg8);
        let result = reg8_value.wrapping_add(1);

        self.update_z_flag(result);
        self.registers.set_s_flag(false);
        self.update_h_flag8(reg8_value, 1, false); // Set H flag for addition

        self.registers.set_register8(reg8, result);

        self.handle_cycles(4);
    }

    pub fn dec_16(&mut self, reg16: Register) {
        self.registers
            .set_register16(reg16, self.registers.get_register16(reg16).wrapping_sub(1));
        self.handle_cycles(8);
    }

    pub fn dec_8(&mut self, reg8: Register) {
        let reg8_value = self.registers.get_register8(reg8);
        let result = reg8_value.wrapping_sub(1);

        self.update_z_flag(result);
        self.registers.set_s_flag(true);
        self.update_h_flag8(reg8_value, 1, true);

        self.registers.set_register8(reg8, result);
        self.handle_cycles(4);
    }

    pub fn rlca(&mut self) {
        let a = self.registers.a;

        self.registers
            .set_c_flag(CPU::get_bit_at_position(a, 7) == 1);

        self.registers.a = a.rotate_left(1);

        self.registers.set_z_flag(false);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn rrca(&mut self) {
        let a = self.registers.a;

        self.registers
            .set_c_flag(CPU::get_bit_at_position(a, 0) == 1);

        self.registers.a = a.rotate_right(1);

        self.registers.set_z_flag(false);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn jr_nz_imm1(&mut self) {
        if !self.registers.get_z_flag() {
            self.jr_imm1();
        } else {
            self.handle_cycles(8);
        }
    }

    pub fn jr_nc_imm1(&mut self) {
        if !self.registers.get_c_flag() {
            self.jr_imm1();
        } else {
            self.handle_cycles(8);
        }
    }

    pub fn jr_z_imm1(&mut self) {
        if self.registers.get_z_flag() {
            self.jr_imm1();
        } else {
            self.handle_cycles(8);
        }
    }

    pub fn jr_c_imm1(&mut self) {
        if self.registers.get_c_flag() {
            self.jr_imm1();
        } else {
            self.handle_cycles(8);
        }
    }

    pub fn jr_imm1(&mut self) {
        let offset = self.memory.read_byte(self.registers.pc) as i8;
        self.registers.pc += 1;

        let new_pc = self.registers.pc.wrapping_add(offset as u16);
        self.registers.pc = new_pc;
        self.handle_cycles(12);
    }

    pub fn rla(&mut self) {
        let a = self.registers.a;
        let carry = self.registers.get_c_flag() as u8;

        self.registers
            .set_c_flag(CPU::get_bit_at_position(a, 7) == 1);

        self.registers.a = (a << 1) | carry;

        self.registers.set_z_flag(false);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn rra(&mut self) {
        let a = self.registers.a;
        let carry = self.registers.get_c_flag() as u8;

        self.registers
            .set_c_flag(CPU::get_bit_at_position(a, 0) == 1);

        self.registers.a = (a >> 1) | (carry << 7);

        self.registers.set_z_flag(false);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn daa(&mut self) {
        let mut adjust = 0;
        let mut carry = false;

        if !self.registers.get_s_flag() {
            if self.registers.get_h_flag() || (self.registers.a & 0x0F) > 9 {
                adjust |= 0x06;
            }
            if self.registers.get_c_flag() || (self.registers.a & 0xF0) > 0x90 {
                adjust |= 0x60;
                carry = true;
            }
        } else {
            if self.registers.get_h_flag() {
                adjust |= 0x06;
            }
            if self.registers.get_c_flag() {
                adjust |= 0x60;
            }
        }
        if !self.registers.get_s_flag() {
            self.registers.a = self.registers.a.wrapping_add(adjust);
        } else {
            self.registers.a = self.registers.a.wrapping_sub(adjust);
        }

        self.update_z_flag(self.registers.a);
        self.registers.set_h_flag(false);

        if carry {
            self.registers.set_c_flag(carry);
        }

        self.handle_cycles(4);
    }

    pub fn scf(&mut self) {
        self.registers.set_c_flag(true);
        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn halt(&mut self) {
        self.halted = true;

        self.handle_cycles(4);
    }

    pub fn cpl(&mut self) {
        self.registers.a = !self.registers.a;

        self.registers.set_s_flag(true);
        self.registers.set_h_flag(true);

        self.handle_cycles(4);
    }

    pub fn ccf(&mut self) {
        self.registers.set_c_flag(!self.registers.get_c_flag());

        self.registers.set_s_flag(false);
        self.registers.set_h_flag(false);

        self.handle_cycles(4);
    }

    pub fn add_16_16(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register16(dest);
        let reg2 = self.registers.get_register16(src);
        let result = reg1.wrapping_add(reg2);

        self.registers.set_s_flag(false);
        self.update_c_flag16(reg1, reg2, false);
        self.update_h_flag16(reg1, reg2, false);

        self.registers.set_register16(dest, result);

        self.handle_cycles(8);
    }

    pub fn add_8_8(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register8(dest);
        let reg2 = self.registers.get_register8(src);
        let result = self.add(reg1, reg2);

        self.registers.set_register8(dest, result);

        self.handle_cycles(4);
    }

    pub fn sub_8_8(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register8(dest);
        let reg2 = self.registers.get_register8(src);
        let result = self.sub(reg1, reg2);

        self.registers.set_register8(dest, result);

        self.handle_cycles(4);
    }

    pub fn and_8_8(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register8(dest);
        let reg2 = self.registers.get_register8(src);
        let result = self.and(reg1, reg2);

        self.registers.set_register8(dest, result);

        self.handle_cycles(4);
    }

    pub fn or_8_8(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register8(dest);
        let reg2 = self.registers.get_register8(src);
        let result = self.or(reg1, reg2);

        self.registers.set_register8(dest, result);

        self.handle_cycles(4);
    }

    pub fn add_8_mem16(&mut self, dest: Register, reg16: Register) {
        let reg = self.registers.get_register8(dest);
        let addr = self.registers.get_register16(reg16);
        let value = self.memory.read_byte(addr);

        let result = self.and(reg, value);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn sub_8_mem16(&mut self, dest: Register, reg16: Register) {
        let reg = self.registers.get_register8(dest);
        let addr = self.registers.get_register16(reg16);
        let value = self.memory.read_byte(addr);

        let result = self.sub(reg, value);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn and_8_mem16(&mut self, dest: Register, reg16: Register) {
        let reg = self.registers.get_register8(dest);
        let addr = self.registers.get_register16(reg16);
        let value = self.memory.read_byte(addr);

        let result = self.and(reg, value);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn or_8_mem16(&mut self, dest: Register, reg16: Register) {
        let reg = self.registers.get_register8(dest);
        let addr = self.registers.get_register16(reg16);
        let value = self.memory.read_byte(addr);

        let result = self.or(reg, value);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn add_8_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register8(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = self.add(reg, next_byte);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn sub_8_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register8(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = self.sub(reg, next_byte);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn and_8_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register8(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = self.and(reg, next_byte);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn or_8_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register8(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = self.or(reg, next_byte);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn adc_8_8(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register8(dest);
        let reg2 = self.registers.get_register8(src);

        let result = self.adc(reg1, reg2);

        self.registers.set_register8(dest, result);

        self.handle_cycles(4);
    }

    pub fn sbc_8_8(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register8(dest);
        let reg2 = self.registers.get_register8(src);

        let result = self.sbc(reg1, reg2);

        self.registers.set_register8(dest, result);

        self.handle_cycles(4);
    }

    pub fn xor_8_8(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register8(dest);
        let reg2 = self.registers.get_register8(src);

        let result = self.xor(reg1, reg2);

        self.registers.set_register8(dest, result);

        self.handle_cycles(4);
    }

    pub fn cp_8_8(&mut self, dest: Register, src: Register) {
        let reg1 = self.registers.get_register8(dest);
        let reg2 = self.registers.get_register8(src);

        let result = self.cp(reg1, reg2);

        self.handle_cycles(4);
    }

    pub fn adc_8_mem16(&mut self, dest: Register, reg16: Register) {
        let reg = self.registers.get_register8(dest);
        let addr = self.registers.get_register16(reg16);
        let value = self.memory.read_byte(addr);

        let result = self.adc(reg, value);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn sbc_8_mem16(&mut self, dest: Register, reg16: Register) {
        let reg = self.registers.get_register8(dest);
        let addr = self.registers.get_register16(reg16);
        let value = self.memory.read_byte(addr);

        let result = self.sbc(reg, value);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn xor_8_mem16(&mut self, dest: Register, reg16: Register) {
        let reg = self.registers.get_register8(dest);
        let addr = self.registers.get_register16(reg16);
        let value = self.memory.read_byte(addr);

        let result = self.xor(reg, value);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn cp_8_mem16(&mut self, dest: Register, reg16: Register) {
        let reg = self.registers.get_register8(dest);
        let addr = self.registers.get_register16(reg16);
        let value = self.memory.read_byte(addr);

        let result = self.cp(reg, value);

        self.handle_cycles(8);
    }

    pub fn adc_8_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register8(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = self.adc(reg, next_byte);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn sbc_8_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register8(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = self.sbc(reg, next_byte);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn xor_8_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register8(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = self.xor(reg, next_byte);

        self.registers.set_register8(dest, result);

        self.handle_cycles(8);
    }

    pub fn cp_8_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register8(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = self.cp(reg, next_byte);

        self.handle_cycles(8);
    }

    pub fn add_16_imm1(&mut self, dest: Register) {
        let reg = self.registers.get_register16(dest);
        let next_byte = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let result = reg.wrapping_add(next_byte as u16);

        self.registers.set_z_flag(false);
        self.registers.set_s_flag(false);
        self.update_h_flag16(reg, next_byte as u16, false);
        self.update_c_flag16(reg, next_byte as u16, false);

        self.registers.set_register16(dest, result);

        self.handle_cycles(16);
    }

    pub fn ret_nz(&mut self) {
        if !self.registers.get_z_flag() {
            self.ret();
            self.handle_cycles(4);
        } else {
            self.handle_cycles(8);
        }
    }

    pub fn ret_nc(&mut self) {
        if !self.registers.get_c_flag() {
            self.ret();
            self.handle_cycles(4);
        } else {
            self.handle_cycles(8);
        }
    }

    pub fn ret_z(&mut self) {
        if self.registers.get_z_flag() {
            self.ret();
            self.handle_cycles(4);
        } else {
            self.handle_cycles(8);
        }
    }

    pub fn ret_c(&mut self) {
        if self.registers.get_c_flag() {
            self.ret();
            self.handle_cycles(4);
        } else {
            self.handle_cycles(8);
        }
    }

    pub fn ret(&mut self) {
        let next_byte1 = self.memory.read_byte(self.registers.sp) as u16;
        self.registers.sp += 1;
        let next_byte2 = self.memory.read_byte(self.registers.sp) as u16;
        self.registers.sp += 1;
        let addr = (next_byte2 << 8) | next_byte1;

        self.registers.pc = addr;
        self.handle_cycles(16);
    }

    pub fn pop_16(&mut self, reg16: Register) {
        let next_byte1 = self.memory.read_byte(self.registers.sp);
        self.registers.sp += 1;
        let next_byte2 = self.memory.read_byte(self.registers.sp);
        self.registers.sp += 1;

        let word = ((next_byte2 as u16) << 8) | (next_byte1 as u16);
        self.registers.set_register16(reg16, word);

        self.handle_cycles(12);
    }

    pub fn push_16(&mut self, reg16: Register) {
        let value = self.registers.get_register16(reg16);

        self.registers.sp -= 1;
        self.memory
            .write_byte(self.registers.sp, (value >> 8) as u8);
        self.registers.sp -= 1;
        self.memory.write_byte(self.registers.sp, value as u8);

        self.handle_cycles(16);
    }

    pub fn jp_nz_imm2(&mut self) {
        if !self.registers.get_z_flag() {
            self.jp_imm2();
        } else {
            self.handle_cycles(12);
        }
    }

    pub fn jp_nc_imm2(&mut self) {
        if !self.registers.get_c_flag() {
            self.jp_imm2();
        } else {
            self.handle_cycles(12);
        }
    }

    pub fn jp_z_imm2(&mut self) {
        if self.registers.get_z_flag() {
            self.jp_imm2();
        } else {
            self.handle_cycles(12);
        }
    }

    pub fn jp_c_imm2(&mut self) {
        if self.registers.get_c_flag() {
            self.jp_imm2();
        } else {
            self.handle_cycles(12);
        }
    }

    pub fn jp_imm2(&mut self) {
        let next_byte1 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        let next_byte2 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let addr = ((next_byte2 as u16) << 8) | (next_byte1 as u16);

        self.registers.pc = addr;
        self.handle_cycles(16);
    }

    pub fn jp_mem16(&mut self, dest: Register) {
        let addr = self.registers.get_register16(dest);

        self.registers.pc = addr;
        self.handle_cycles(4);
    }

    pub fn call_nz_imm2(&mut self) {
        if !self.registers.get_z_flag() {
            self.call_imm2();
        } else {
            self.handle_cycles(12);
        }
    }

    pub fn call_nc_imm2(&mut self) {
        if !self.registers.get_c_flag() {
            self.call_imm2();
        } else {
            self.handle_cycles(12);
        }
    }

    pub fn call_z_imm2(&mut self) {
        if self.registers.get_z_flag() {
            self.call_imm2();
        } else {
            self.handle_cycles(12);
        }
    }

    pub fn call_c_imm2(&mut self) {
        if self.registers.get_c_flag() {
            self.call_imm2();
        } else {
            self.handle_cycles(12);
        }
    }

    pub fn call_imm2(&mut self) {
        let next_byte1 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        let next_byte2 = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let addr = ((next_byte2 as u16) << 8) | (next_byte1 as u16);

        let pc = self.registers.pc;
        self.registers.sp -= 1;
        self.memory.write_byte(self.registers.sp, (pc >> 8) as u8);
        self.registers.sp -= 1;
        self.memory.write_byte(self.registers.sp, pc as u8);

        self.registers.pc = addr;

        self.handle_cycles(24);
    }

    pub fn reti(&mut self) {
        self.ret();
        self.ime = true;
    }

    pub fn rst(&mut self, target: u16) {
        let pc = self.registers.pc;
        self.registers.sp -= 1;
        self.memory.write_byte(self.registers.sp, (pc >> 8) as u8);
        self.registers.sp -= 1;
        self.memory.write_byte(self.registers.sp, pc as u8);

        self.registers.pc = target;

        self.handle_cycles(16);
    }

    pub fn di(&mut self) {
        self.ime = false;
        self.handle_cycles(4);
    }

    pub fn ei(&mut self) {
        self.ime = true;
        self.handle_cycles(4);
    }

    pub fn prefix_cb(&mut self) {
        let opcode = self.fetch_opcode();

        self.execute_cb_opcode(opcode);

        self.handle_cycles(4);
    }
}
