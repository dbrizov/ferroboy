#[cfg(test)]
mod tests;

use crate::cpu::{Cpu, FLAG_C, FLAG_H, FLAG_N, FLAG_Z};

impl Cpu {
    fn carry(&self) -> u8 {
        self.regs.has_flags(FLAG_C) as u8
    }

    // ---- 8-bit arithmetic, always targeting A -------------------------------

    pub fn add8(&mut self, value: u8) {
        let a = self.regs.a;
        let result = a.wrapping_add(value);
        self.regs.set_flags(
            result == 0,
            false,
            (a & 0xF) + (value & 0xF) > 0xF,
            a as u16 + value as u16 > 0xFF,
        );
        self.regs.a = result;
    }

    pub fn adc8(&mut self, value: u8) {
        let a = self.regs.a;
        let carry = self.carry();
        let result = a.wrapping_add(value).wrapping_add(carry);
        self.regs.set_flags(
            result == 0,
            false,
            (a & 0xF) + (value & 0xF) + carry > 0xF,
            a as u16 + value as u16 + carry as u16 > 0xFF,
        );
        self.regs.a = result;
    }

    pub fn sub8(&mut self, value: u8) {
        let a = self.regs.a;
        self.regs
            .set_flags(a == value, true, (a & 0xF) < (value & 0xF), a < value);
        self.regs.a = a.wrapping_sub(value);
    }

    pub fn sbc8(&mut self, value: u8) {
        let a = self.regs.a;
        let carry = self.carry();
        let result = a.wrapping_sub(value).wrapping_sub(carry);
        self.regs.set_flags(
            result == 0,
            true,
            (a & 0xF) < (value & 0xF) + carry,
            (a as u16) < value as u16 + carry as u16,
        );
        self.regs.a = result;
    }

    pub fn and8(&mut self, value: u8) {
        self.regs.a &= value;
        self.regs.set_flags(self.regs.a == 0, false, true, false);
    }

    pub fn or8(&mut self, value: u8) {
        self.regs.a |= value;
        self.regs.set_flags(self.regs.a == 0, false, false, false);
    }

    pub fn xor8(&mut self, value: u8) {
        self.regs.a ^= value;
        self.regs.set_flags(self.regs.a == 0, false, false, false);
    }

    pub fn cp8(&mut self, value: u8) {
        let a = self.regs.a;
        self.sub8(value);
        self.regs.a = a;
    }

    // ---- 8-bit arithmetic on any register or (HL) ---------------------------

    pub fn inc8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        let carry = self.regs.has_flags(FLAG_C);
        self.regs
            .set_flags(result == 0, false, (value & 0xF) == 0xF, carry);
        result
    }

    pub fn dec8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        let carry = self.regs.has_flags(FLAG_C);
        self.regs
            .set_flags(result == 0, true, (value & 0xF) == 0, carry);
        result
    }

    // ---- 16-bit -------------------------------------------------------------

    pub fn add16(&mut self, value: u16) {
        let hl = self.regs.hl();
        let zero = self.regs.has_flags(FLAG_Z);
        self.regs.set_flags(
            zero,
            false,
            (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF,
            hl as u32 + value as u32 > 0xFFFF,
        );
        self.regs.set_hl(hl.wrapping_add(value));
    }

    pub fn add_sp_i8(&mut self, offset: i8) -> u16 {
        let sp = self.sp;
        let value = offset as u16;
        self.regs.set_flags(
            false,
            false,
            (sp & 0xF) + (value & 0xF) > 0xF,
            (sp & 0xFF) + (value & 0xFF) > 0xFF,
        );
        sp.wrapping_add(value)
    }

    // ---- A-register oddments ------------------------------------------------

    pub fn daa(&mut self) {
        let a = self.regs.a;
        let subtract = self.regs.has_flags(FLAG_N);
        let mut adjust = 0;
        let mut carry = self.regs.has_flags(FLAG_C);

        if self.regs.has_flags(FLAG_H) || (!subtract && a & 0x0F > 0x09) {
            adjust |= 0x06;
        }
        if carry || (!subtract && a > 0x99) {
            adjust |= 0x60;
            carry = true;
        }

        let result = if subtract {
            a.wrapping_sub(adjust)
        } else {
            a.wrapping_add(adjust)
        };

        self.regs.set_flags(result == 0, subtract, false, carry);
        self.regs.a = result;
    }

    pub fn cpl(&mut self) {
        let zero = self.regs.has_flags(FLAG_Z);
        let carry = self.regs.has_flags(FLAG_C);
        self.regs.a = !self.regs.a;
        self.regs.set_flags(zero, true, true, carry);
    }

    pub fn scf(&mut self) {
        let zero = self.regs.has_flags(FLAG_Z);
        self.regs.set_flags(zero, false, false, true);
    }

    pub fn ccf(&mut self) {
        let zero = self.regs.has_flags(FLAG_Z);
        let carry = self.regs.has_flags(FLAG_C);
        self.regs.set_flags(zero, false, false, !carry);
    }

    // ---- Rotates on A, opcodes 0x07/0x0F/0x17/0x1F (these are NOT the CB rotates.) ----

    pub fn rlca(&mut self) {
        let value = self.regs.a;
        self.regs.a = value.rotate_left(1);
        self.regs.set_flags(false, false, false, value & 0x80 != 0);
    }

    pub fn rrca(&mut self) {
        let value = self.regs.a;
        self.regs.a = value.rotate_right(1);
        self.regs.set_flags(false, false, false, value & 0x01 != 0);
    }

    pub fn rla(&mut self) {
        let value = self.regs.a;
        self.regs.a = value << 1 | self.carry();
        self.regs.set_flags(false, false, false, value & 0x80 != 0);
    }

    pub fn rra(&mut self) {
        let value = self.regs.a;
        self.regs.a = value >> 1 | self.carry() << 7;
        self.regs.set_flags(false, false, false, value & 0x01 != 0);
    }

    // ---- CB rotates and shifts, on any register or (HL) ---------------------

    pub fn rlc8(&mut self, value: u8) -> u8 {
        let result = value.rotate_left(1);
        self.regs
            .set_flags(result == 0, false, false, value & 0x80 != 0);
        result
    }

    pub fn rrc8(&mut self, value: u8) -> u8 {
        let result = value.rotate_right(1);
        self.regs
            .set_flags(result == 0, false, false, value & 0x01 != 0);
        result
    }

    pub fn rl8(&mut self, value: u8) -> u8 {
        let result = value << 1 | self.carry();
        self.regs
            .set_flags(result == 0, false, false, value & 0x80 != 0);
        result
    }

    pub fn rr8(&mut self, value: u8) -> u8 {
        let result = value >> 1 | self.carry() << 7;
        self.regs
            .set_flags(result == 0, false, false, value & 0x01 != 0);
        result
    }

    pub fn sla8(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.regs
            .set_flags(result == 0, false, false, value & 0x80 != 0);
        result
    }

    pub fn sra8(&mut self, value: u8) -> u8 {
        let result = value >> 1 | value & 0x80;
        self.regs
            .set_flags(result == 0, false, false, value & 0x01 != 0);
        result
    }

    pub fn srl8(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.regs
            .set_flags(result == 0, false, false, value & 0x01 != 0);
        result
    }

    pub fn swap8(&mut self, value: u8) -> u8 {
        let result = value.rotate_left(4);
        self.regs.set_flags(result == 0, false, false, false);
        result
    }

    pub fn bit8(&mut self, bit: u8, value: u8) {
        let carry = self.regs.has_flags(FLAG_C);
        self.regs
            .set_flags(value & 1 << bit == 0, false, true, carry);
    }
}
