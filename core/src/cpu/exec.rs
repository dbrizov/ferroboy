use crate::bus::Bus;
use crate::cpu::opcodes::*;
use crate::cpu::{Cpu, FLAG_C, FLAG_Z};

impl Cpu {
    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        match opcode {
            NOP => 4,
            LD_BC_D16 => {
                let value = self.fetch16(bus);
                self.regs.set_bc(value);
                12
            }
            LD_MEM_BC_A => {
                bus.write(self.regs.bc(), self.regs.a);
                8
            }
            INC_BC => {
                self.regs.set_bc(self.regs.bc().wrapping_add(1));
                8
            }
            INC_B => {
                let value = self.inc8(self.regs.b);
                self.regs.b = value;
                4
            }
            DEC_B => {
                let value = self.dec8(self.regs.b);
                self.regs.b = value;
                4
            }
            LD_B_D8 => {
                let value = self.fetch8(bus);
                self.regs.b = value;
                8
            }
            RLCA => {
                self.rlca();
                4
            }
            LD_MEM_A16_SP => {
                let address = self.fetch16(bus);
                bus.write(address, self.sp as u8);
                bus.write(address.wrapping_add(1), (self.sp >> 8) as u8);
                20
            }
            ADD_HL_BC => {
                self.add16(self.regs.bc());
                8
            }
            LD_A_MEM_BC => {
                self.regs.a = bus.read(self.regs.bc());
                8
            }
            DEC_BC => {
                self.regs.set_bc(self.regs.bc().wrapping_sub(1));
                8
            }
            INC_C => {
                let value = self.inc8(self.regs.c);
                self.regs.c = value;
                4
            }
            DEC_C => {
                let value = self.dec8(self.regs.c);
                self.regs.c = value;
                4
            }
            LD_C_D8 => {
                let value = self.fetch8(bus);
                self.regs.c = value;
                8
            }
            RRCA => {
                self.rrca();
                4
            }
            STOP_0 => {
                self.fetch8(bus); // STOP is two bytes; the second is ignored
                4
            }
            LD_DE_D16 => {
                let value = self.fetch16(bus);
                self.regs.set_de(value);
                12
            }
            LD_MEM_DE_A => {
                bus.write(self.regs.de(), self.regs.a);
                8
            }
            INC_DE => {
                self.regs.set_de(self.regs.de().wrapping_add(1));
                8
            }
            INC_D => {
                let value = self.inc8(self.regs.d);
                self.regs.d = value;
                4
            }
            DEC_D => {
                let value = self.dec8(self.regs.d);
                self.regs.d = value;
                4
            }
            LD_D_D8 => {
                let value = self.fetch8(bus);
                self.regs.d = value;
                8
            }
            RLA => {
                self.rla();
                4
            }
            JR_R8 => {
                let offset = self.fetch8(bus) as i8;
                self.pc = self.pc.wrapping_add(offset as u16);
                12
            }
            ADD_HL_DE => {
                self.add16(self.regs.de());
                8
            }
            LD_A_MEM_DE => {
                self.regs.a = bus.read(self.regs.de());
                8
            }
            DEC_DE => {
                self.regs.set_de(self.regs.de().wrapping_sub(1));
                8
            }
            INC_E => {
                let value = self.inc8(self.regs.e);
                self.regs.e = value;
                4
            }
            DEC_E => {
                let value = self.dec8(self.regs.e);
                self.regs.e = value;
                4
            }
            LD_E_D8 => {
                let value = self.fetch8(bus);
                self.regs.e = value;
                8
            }
            RRA => {
                self.rra();
                4
            }
            JR_NZ_R8 => {
                let offset = self.fetch8(bus) as i8;
                if !self.regs.has_flags(FLAG_Z) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            LD_HL_D16 => {
                let value = self.fetch16(bus);
                self.regs.set_hl(value);
                12
            }
            LD_MEM_HLI_A => {
                bus.write(self.regs.hl(), self.regs.a);
                self.regs.set_hl(self.regs.hl().wrapping_add(1));
                8
            }
            INC_HL => {
                self.regs.set_hl(self.regs.hl().wrapping_add(1));
                8
            }
            INC_H => {
                let value = self.inc8(self.regs.h);
                self.regs.h = value;
                4
            }
            DEC_H => {
                let value = self.dec8(self.regs.h);
                self.regs.h = value;
                4
            }
            LD_H_D8 => {
                let value = self.fetch8(bus);
                self.regs.h = value;
                8
            }
            DAA => {
                self.daa();
                4
            }
            JR_Z_R8 => {
                let offset = self.fetch8(bus) as i8;
                if self.regs.has_flags(FLAG_Z) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            ADD_HL_HL => {
                self.add16(self.regs.hl());
                8
            }
            LD_A_MEM_HLI => {
                self.regs.a = bus.read(self.regs.hl());
                self.regs.set_hl(self.regs.hl().wrapping_add(1));
                8
            }
            DEC_HL => {
                self.regs.set_hl(self.regs.hl().wrapping_sub(1));
                8
            }
            INC_L => {
                let value = self.inc8(self.regs.l);
                self.regs.l = value;
                4
            }
            DEC_L => {
                let value = self.dec8(self.regs.l);
                self.regs.l = value;
                4
            }
            LD_L_D8 => {
                let value = self.fetch8(bus);
                self.regs.l = value;
                8
            }
            CPL => {
                self.cpl();
                4
            }
            JR_NC_R8 => {
                let offset = self.fetch8(bus) as i8;
                if !self.regs.has_flags(FLAG_C) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            LD_SP_D16 => {
                let value = self.fetch16(bus);
                self.sp = value;
                12
            }
            LD_MEM_HLD_A => {
                bus.write(self.regs.hl(), self.regs.a);
                self.regs.set_hl(self.regs.hl().wrapping_sub(1));
                8
            }
            INC_SP => {
                self.sp = self.sp.wrapping_add(1);
                8
            }
            INC_MEM_HL => {
                let value = self.inc8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                12
            }
            DEC_MEM_HL => {
                let value = self.dec8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                12
            }
            LD_MEM_HL_D8 => {
                let value = self.fetch8(bus);
                bus.write(self.regs.hl(), value);
                12
            }
            SCF => {
                self.scf();
                4
            }
            JR_C_R8 => {
                let offset = self.fetch8(bus) as i8;
                if self.regs.has_flags(FLAG_C) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            ADD_HL_SP => {
                self.add16(self.sp);
                8
            }
            LD_A_MEM_HLD => {
                self.regs.a = bus.read(self.regs.hl());
                self.regs.set_hl(self.regs.hl().wrapping_sub(1));
                8
            }
            DEC_SP => {
                self.sp = self.sp.wrapping_sub(1);
                8
            }
            INC_A => {
                let value = self.inc8(self.regs.a);
                self.regs.a = value;
                4
            }
            DEC_A => {
                let value = self.dec8(self.regs.a);
                self.regs.a = value;
                4
            }
            LD_A_D8 => {
                let value = self.fetch8(bus);
                self.regs.a = value;
                8
            }
            CCF => {
                self.ccf();
                4
            }
            LD_B_B => 4,
            LD_B_C => {
                self.regs.b = self.regs.c;
                4
            }
            LD_B_D => {
                self.regs.b = self.regs.d;
                4
            }
            LD_B_E => {
                self.regs.b = self.regs.e;
                4
            }
            LD_B_H => {
                self.regs.b = self.regs.h;
                4
            }
            LD_B_L => {
                self.regs.b = self.regs.l;
                4
            }
            LD_B_MEM_HL => {
                self.regs.b = bus.read(self.regs.hl());
                8
            }
            LD_B_A => {
                self.regs.b = self.regs.a;
                4
            }
            LD_C_B => {
                self.regs.c = self.regs.b;
                4
            }
            LD_C_C => 4,
            LD_C_D => {
                self.regs.c = self.regs.d;
                4
            }
            LD_C_E => {
                self.regs.c = self.regs.e;
                4
            }
            LD_C_H => {
                self.regs.c = self.regs.h;
                4
            }
            LD_C_L => {
                self.regs.c = self.regs.l;
                4
            }
            LD_C_MEM_HL => {
                self.regs.c = bus.read(self.regs.hl());
                8
            }
            LD_C_A => {
                self.regs.c = self.regs.a;
                4
            }
            LD_D_B => {
                self.regs.d = self.regs.b;
                4
            }
            LD_D_C => {
                self.regs.d = self.regs.c;
                4
            }
            LD_D_D => 4,
            LD_D_E => {
                self.regs.d = self.regs.e;
                4
            }
            LD_D_H => {
                self.regs.d = self.regs.h;
                4
            }
            LD_D_L => {
                self.regs.d = self.regs.l;
                4
            }
            LD_D_MEM_HL => {
                self.regs.d = bus.read(self.regs.hl());
                8
            }
            LD_D_A => {
                self.regs.d = self.regs.a;
                4
            }
            LD_E_B => {
                self.regs.e = self.regs.b;
                4
            }
            LD_E_C => {
                self.regs.e = self.regs.c;
                4
            }
            LD_E_D => {
                self.regs.e = self.regs.d;
                4
            }
            LD_E_E => 4,
            LD_E_H => {
                self.regs.e = self.regs.h;
                4
            }
            LD_E_L => {
                self.regs.e = self.regs.l;
                4
            }
            LD_E_MEM_HL => {
                self.regs.e = bus.read(self.regs.hl());
                8
            }
            LD_E_A => {
                self.regs.e = self.regs.a;
                4
            }
            LD_H_B => {
                self.regs.h = self.regs.b;
                4
            }
            LD_H_C => {
                self.regs.h = self.regs.c;
                4
            }
            LD_H_D => {
                self.regs.h = self.regs.d;
                4
            }
            LD_H_E => {
                self.regs.h = self.regs.e;
                4
            }
            LD_H_H => 4,
            LD_H_L => {
                self.regs.h = self.regs.l;
                4
            }
            LD_H_MEM_HL => {
                self.regs.h = bus.read(self.regs.hl());
                8
            }
            LD_H_A => {
                self.regs.h = self.regs.a;
                4
            }
            LD_L_B => {
                self.regs.l = self.regs.b;
                4
            }
            LD_L_C => {
                self.regs.l = self.regs.c;
                4
            }
            LD_L_D => {
                self.regs.l = self.regs.d;
                4
            }
            LD_L_E => {
                self.regs.l = self.regs.e;
                4
            }
            LD_L_H => {
                self.regs.l = self.regs.h;
                4
            }
            LD_L_L => 4,
            LD_L_MEM_HL => {
                self.regs.l = bus.read(self.regs.hl());
                8
            }
            LD_L_A => {
                self.regs.l = self.regs.a;
                4
            }
            LD_MEM_HL_B => {
                bus.write(self.regs.hl(), self.regs.b);
                8
            }
            LD_MEM_HL_C => {
                bus.write(self.regs.hl(), self.regs.c);
                8
            }
            LD_MEM_HL_D => {
                bus.write(self.regs.hl(), self.regs.d);
                8
            }
            LD_MEM_HL_E => {
                bus.write(self.regs.hl(), self.regs.e);
                8
            }
            LD_MEM_HL_H => {
                bus.write(self.regs.hl(), self.regs.h);
                8
            }
            LD_MEM_HL_L => {
                bus.write(self.regs.hl(), self.regs.l);
                8
            }
            HALT => {
                self.halted = true;
                4
            }
            LD_MEM_HL_A => {
                bus.write(self.regs.hl(), self.regs.a);
                8
            }
            LD_A_B => {
                self.regs.a = self.regs.b;
                4
            }
            LD_A_C => {
                self.regs.a = self.regs.c;
                4
            }
            LD_A_D => {
                self.regs.a = self.regs.d;
                4
            }
            LD_A_E => {
                self.regs.a = self.regs.e;
                4
            }
            LD_A_H => {
                self.regs.a = self.regs.h;
                4
            }
            LD_A_L => {
                self.regs.a = self.regs.l;
                4
            }
            LD_A_MEM_HL => {
                self.regs.a = bus.read(self.regs.hl());
                8
            }
            LD_A_A => 4,
            ADD_A_B => {
                self.add8(self.regs.b);
                4
            }
            ADD_A_C => {
                self.add8(self.regs.c);
                4
            }
            ADD_A_D => {
                self.add8(self.regs.d);
                4
            }
            ADD_A_E => {
                self.add8(self.regs.e);
                4
            }
            ADD_A_H => {
                self.add8(self.regs.h);
                4
            }
            ADD_A_L => {
                self.add8(self.regs.l);
                4
            }
            ADD_A_MEM_HL => {
                self.add8(bus.read(self.regs.hl()));
                8
            }
            ADD_A_A => {
                self.add8(self.regs.a);
                4
            }
            ADC_A_B => {
                self.adc8(self.regs.b);
                4
            }
            ADC_A_C => {
                self.adc8(self.regs.c);
                4
            }
            ADC_A_D => {
                self.adc8(self.regs.d);
                4
            }
            ADC_A_E => {
                self.adc8(self.regs.e);
                4
            }
            ADC_A_H => {
                self.adc8(self.regs.h);
                4
            }
            ADC_A_L => {
                self.adc8(self.regs.l);
                4
            }
            ADC_A_MEM_HL => {
                self.adc8(bus.read(self.regs.hl()));
                8
            }
            ADC_A_A => {
                self.adc8(self.regs.a);
                4
            }
            SUB_B => {
                self.sub8(self.regs.b);
                4
            }
            SUB_C => {
                self.sub8(self.regs.c);
                4
            }
            SUB_D => {
                self.sub8(self.regs.d);
                4
            }
            SUB_E => {
                self.sub8(self.regs.e);
                4
            }
            SUB_H => {
                self.sub8(self.regs.h);
                4
            }
            SUB_L => {
                self.sub8(self.regs.l);
                4
            }
            SUB_MEM_HL => {
                self.sub8(bus.read(self.regs.hl()));
                8
            }
            SUB_A => {
                self.sub8(self.regs.a);
                4
            }
            SBC_A_B => {
                self.sbc8(self.regs.b);
                4
            }
            SBC_A_C => {
                self.sbc8(self.regs.c);
                4
            }
            SBC_A_D => {
                self.sbc8(self.regs.d);
                4
            }
            SBC_A_E => {
                self.sbc8(self.regs.e);
                4
            }
            SBC_A_H => {
                self.sbc8(self.regs.h);
                4
            }
            SBC_A_L => {
                self.sbc8(self.regs.l);
                4
            }
            SBC_A_MEM_HL => {
                self.sbc8(bus.read(self.regs.hl()));
                8
            }
            SBC_A_A => {
                self.sbc8(self.regs.a);
                4
            }
            AND_B => {
                self.and8(self.regs.b);
                4
            }
            AND_C => {
                self.and8(self.regs.c);
                4
            }
            AND_D => {
                self.and8(self.regs.d);
                4
            }
            AND_E => {
                self.and8(self.regs.e);
                4
            }
            AND_H => {
                self.and8(self.regs.h);
                4
            }
            AND_L => {
                self.and8(self.regs.l);
                4
            }
            AND_MEM_HL => {
                self.and8(bus.read(self.regs.hl()));
                8
            }
            AND_A => {
                self.and8(self.regs.a);
                4
            }
            XOR_B => {
                self.xor8(self.regs.b);
                4
            }
            XOR_C => {
                self.xor8(self.regs.c);
                4
            }
            XOR_D => {
                self.xor8(self.regs.d);
                4
            }
            XOR_E => {
                self.xor8(self.regs.e);
                4
            }
            XOR_H => {
                self.xor8(self.regs.h);
                4
            }
            XOR_L => {
                self.xor8(self.regs.l);
                4
            }
            XOR_MEM_HL => {
                self.xor8(bus.read(self.regs.hl()));
                8
            }
            XOR_A => {
                self.xor8(self.regs.a);
                4
            }
            OR_B => {
                self.or8(self.regs.b);
                4
            }
            OR_C => {
                self.or8(self.regs.c);
                4
            }
            OR_D => {
                self.or8(self.regs.d);
                4
            }
            OR_E => {
                self.or8(self.regs.e);
                4
            }
            OR_H => {
                self.or8(self.regs.h);
                4
            }
            OR_L => {
                self.or8(self.regs.l);
                4
            }
            OR_MEM_HL => {
                self.or8(bus.read(self.regs.hl()));
                8
            }
            OR_A => {
                self.or8(self.regs.a);
                4
            }
            CP_B => {
                self.cp8(self.regs.b);
                4
            }
            CP_C => {
                self.cp8(self.regs.c);
                4
            }
            CP_D => {
                self.cp8(self.regs.d);
                4
            }
            CP_E => {
                self.cp8(self.regs.e);
                4
            }
            CP_H => {
                self.cp8(self.regs.h);
                4
            }
            CP_L => {
                self.cp8(self.regs.l);
                4
            }
            CP_MEM_HL => {
                self.cp8(bus.read(self.regs.hl()));
                8
            }
            CP_A => {
                self.cp8(self.regs.a);
                4
            }
            RET_NZ => {
                if !self.regs.has_flags(FLAG_Z) {
                    self.pc = self.pop16(bus);
                    20
                } else {
                    8
                }
            }
            POP_BC => {
                let value = self.pop16(bus);
                self.regs.set_bc(value);
                12
            }
            JP_NZ_A16 => {
                let address = self.fetch16(bus);
                if !self.regs.has_flags(FLAG_Z) {
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            JP_A16 => {
                self.pc = self.fetch16(bus);
                16
            }
            CALL_NZ_A16 => {
                let address = self.fetch16(bus);
                if !self.regs.has_flags(FLAG_Z) {
                    self.push16(bus, self.pc);
                    self.pc = address;
                    24
                } else {
                    12
                }
            }
            PUSH_BC => {
                self.push16(bus, self.regs.bc());
                16
            }
            ADD_A_D8 => {
                let value = self.fetch8(bus);
                self.add8(value);
                8
            }
            RST_00H => {
                self.push16(bus, self.pc);
                self.pc = 0x00;
                16
            }
            RET_Z => {
                if self.regs.has_flags(FLAG_Z) {
                    self.pc = self.pop16(bus);
                    20
                } else {
                    8
                }
            }
            RET => {
                self.pc = self.pop16(bus);
                16
            }
            JP_Z_A16 => {
                let address = self.fetch16(bus);
                if self.regs.has_flags(FLAG_Z) {
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            CALL_Z_A16 => {
                let address = self.fetch16(bus);
                if self.regs.has_flags(FLAG_Z) {
                    self.push16(bus, self.pc);
                    self.pc = address;
                    24
                } else {
                    12
                }
            }
            CALL_A16 => {
                let address = self.fetch16(bus);
                self.push16(bus, self.pc);
                self.pc = address;
                24
            }
            ADC_A_D8 => {
                let value = self.fetch8(bus);
                self.adc8(value);
                8
            }
            RST_08H => {
                self.push16(bus, self.pc);
                self.pc = 0x08;
                16
            }
            RET_NC => {
                if !self.regs.has_flags(FLAG_C) {
                    self.pc = self.pop16(bus);
                    20
                } else {
                    8
                }
            }
            POP_DE => {
                let value = self.pop16(bus);
                self.regs.set_de(value);
                12
            }
            JP_NC_A16 => {
                let address = self.fetch16(bus);
                if !self.regs.has_flags(FLAG_C) {
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            CALL_NC_A16 => {
                let address = self.fetch16(bus);
                if !self.regs.has_flags(FLAG_C) {
                    self.push16(bus, self.pc);
                    self.pc = address;
                    24
                } else {
                    12
                }
            }
            PUSH_DE => {
                self.push16(bus, self.regs.de());
                16
            }
            SUB_D8 => {
                let value = self.fetch8(bus);
                self.sub8(value);
                8
            }
            RST_10H => {
                self.push16(bus, self.pc);
                self.pc = 0x10;
                16
            }
            RET_C => {
                if self.regs.has_flags(FLAG_C) {
                    self.pc = self.pop16(bus);
                    20
                } else {
                    8
                }
            }
            RETI => {
                self.pc = self.pop16(bus);
                self.ime = true; // RETI re-enables at once, unlike EI
                16
            }
            JP_C_A16 => {
                let address = self.fetch16(bus);
                if self.regs.has_flags(FLAG_C) {
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            CALL_C_A16 => {
                let address = self.fetch16(bus);
                if self.regs.has_flags(FLAG_C) {
                    self.push16(bus, self.pc);
                    self.pc = address;
                    24
                } else {
                    12
                }
            }
            SBC_A_D8 => {
                let value = self.fetch8(bus);
                self.sbc8(value);
                8
            }
            RST_18H => {
                self.push16(bus, self.pc);
                self.pc = 0x18;
                16
            }
            LDH_MEM_A8_A => {
                let offset = self.fetch8(bus) as u16;
                bus.write(0xFF00 + offset, self.regs.a);
                12
            }
            POP_HL => {
                let value = self.pop16(bus);
                self.regs.set_hl(value);
                12
            }
            LD_MEM_C_A => {
                bus.write(0xFF00 + self.regs.c as u16, self.regs.a);
                8
            }
            PUSH_HL => {
                self.push16(bus, self.regs.hl());
                16
            }
            AND_D8 => {
                let value = self.fetch8(bus);
                self.and8(value);
                8
            }
            RST_20H => {
                self.push16(bus, self.pc);
                self.pc = 0x20;
                16
            }
            ADD_SP_R8 => {
                let offset = self.fetch8(bus) as i8;
                self.sp = self.add_sp_i8(offset);
                16
            }
            JP_MEM_HL => {
                self.pc = self.regs.hl(); // no memory read despite the mnemonic
                4
            }
            LD_MEM_A16_A => {
                let address = self.fetch16(bus);
                bus.write(address, self.regs.a);
                16
            }
            XOR_D8 => {
                let value = self.fetch8(bus);
                self.xor8(value);
                8
            }
            RST_28H => {
                self.push16(bus, self.pc);
                self.pc = 0x28;
                16
            }
            LDH_A_MEM_A8 => {
                let offset = self.fetch8(bus) as u16;
                self.regs.a = bus.read(0xFF00 + offset);
                12
            }
            POP_AF => {
                let value = self.pop16(bus);
                self.regs.set_af(value); // set_af masks the low nibble of F away
                12
            }
            LD_A_MEM_C => {
                self.regs.a = bus.read(0xFF00 + self.regs.c as u16);
                8
            }
            DI => {
                self.ime = false;
                self.ime_pending = false;
                4
            }
            PUSH_AF => {
                self.push16(bus, self.regs.af());
                16
            }
            OR_D8 => {
                let value = self.fetch8(bus);
                self.or8(value);
                8
            }
            RST_30H => {
                self.push16(bus, self.pc);
                self.pc = 0x30;
                16
            }
            LD_HL_SP_R8 => {
                let offset = self.fetch8(bus) as i8;
                let value = self.add_sp_i8(offset);
                self.regs.set_hl(value);
                12
            }
            LD_SP_HL => {
                self.sp = self.regs.hl();
                8
            }
            LD_A_MEM_A16 => {
                let address = self.fetch16(bus);
                self.regs.a = bus.read(address);
                16
            }
            EI => {
                self.ime_pending = true;
                4
            }
            CP_D8 => {
                let value = self.fetch8(bus);
                self.cp8(value);
                8
            }
            RST_38H => {
                self.push16(bus, self.pc);
                self.pc = 0x38;
                16
            }
            PREFIX_CB => {
                let cb_opcode = self.fetch8(bus);
                self.execute_cb(cb_opcode, bus)
            }
            _ => panic!(
                "undefined opcode {:#04X} at {:#06X}",
                opcode,
                self.pc.wrapping_sub(1)
            ),
        }
    }
}
