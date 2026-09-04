use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::opcodes::*;

impl Cpu {
    pub fn execute_cb(&mut self, cb_opcode: u8, bus: &mut Bus) -> u8 {
        match cb_opcode {
            CB_RLC_B => {
                let value = self.rlc8(self.regs.b);
                self.regs.b = value;
                8
            }
            CB_RLC_C => {
                let value = self.rlc8(self.regs.c);
                self.regs.c = value;
                8
            }
            CB_RLC_D => {
                let value = self.rlc8(self.regs.d);
                self.regs.d = value;
                8
            }
            CB_RLC_E => {
                let value = self.rlc8(self.regs.e);
                self.regs.e = value;
                8
            }
            CB_RLC_H => {
                let value = self.rlc8(self.regs.h);
                self.regs.h = value;
                8
            }
            CB_RLC_L => {
                let value = self.rlc8(self.regs.l);
                self.regs.l = value;
                8
            }
            CB_RLC_MEM_HL => {
                let value = self.rlc8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RLC_A => {
                let value = self.rlc8(self.regs.a);
                self.regs.a = value;
                8
            }
            CB_RRC_B => {
                let value = self.rrc8(self.regs.b);
                self.regs.b = value;
                8
            }
            CB_RRC_C => {
                let value = self.rrc8(self.regs.c);
                self.regs.c = value;
                8
            }
            CB_RRC_D => {
                let value = self.rrc8(self.regs.d);
                self.regs.d = value;
                8
            }
            CB_RRC_E => {
                let value = self.rrc8(self.regs.e);
                self.regs.e = value;
                8
            }
            CB_RRC_H => {
                let value = self.rrc8(self.regs.h);
                self.regs.h = value;
                8
            }
            CB_RRC_L => {
                let value = self.rrc8(self.regs.l);
                self.regs.l = value;
                8
            }
            CB_RRC_MEM_HL => {
                let value = self.rrc8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RRC_A => {
                let value = self.rrc8(self.regs.a);
                self.regs.a = value;
                8
            }
            CB_RL_B => {
                let value = self.rl8(self.regs.b);
                self.regs.b = value;
                8
            }
            CB_RL_C => {
                let value = self.rl8(self.regs.c);
                self.regs.c = value;
                8
            }
            CB_RL_D => {
                let value = self.rl8(self.regs.d);
                self.regs.d = value;
                8
            }
            CB_RL_E => {
                let value = self.rl8(self.regs.e);
                self.regs.e = value;
                8
            }
            CB_RL_H => {
                let value = self.rl8(self.regs.h);
                self.regs.h = value;
                8
            }
            CB_RL_L => {
                let value = self.rl8(self.regs.l);
                self.regs.l = value;
                8
            }
            CB_RL_MEM_HL => {
                let value = self.rl8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RL_A => {
                let value = self.rl8(self.regs.a);
                self.regs.a = value;
                8
            }
            CB_RR_B => {
                let value = self.rr8(self.regs.b);
                self.regs.b = value;
                8
            }
            CB_RR_C => {
                let value = self.rr8(self.regs.c);
                self.regs.c = value;
                8
            }
            CB_RR_D => {
                let value = self.rr8(self.regs.d);
                self.regs.d = value;
                8
            }
            CB_RR_E => {
                let value = self.rr8(self.regs.e);
                self.regs.e = value;
                8
            }
            CB_RR_H => {
                let value = self.rr8(self.regs.h);
                self.regs.h = value;
                8
            }
            CB_RR_L => {
                let value = self.rr8(self.regs.l);
                self.regs.l = value;
                8
            }
            CB_RR_MEM_HL => {
                let value = self.rr8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RR_A => {
                let value = self.rr8(self.regs.a);
                self.regs.a = value;
                8
            }
            CB_SLA_B => {
                let value = self.sla8(self.regs.b);
                self.regs.b = value;
                8
            }
            CB_SLA_C => {
                let value = self.sla8(self.regs.c);
                self.regs.c = value;
                8
            }
            CB_SLA_D => {
                let value = self.sla8(self.regs.d);
                self.regs.d = value;
                8
            }
            CB_SLA_E => {
                let value = self.sla8(self.regs.e);
                self.regs.e = value;
                8
            }
            CB_SLA_H => {
                let value = self.sla8(self.regs.h);
                self.regs.h = value;
                8
            }
            CB_SLA_L => {
                let value = self.sla8(self.regs.l);
                self.regs.l = value;
                8
            }
            CB_SLA_MEM_HL => {
                let value = self.sla8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SLA_A => {
                let value = self.sla8(self.regs.a);
                self.regs.a = value;
                8
            }
            CB_SRA_B => {
                let value = self.sra8(self.regs.b);
                self.regs.b = value;
                8
            }
            CB_SRA_C => {
                let value = self.sra8(self.regs.c);
                self.regs.c = value;
                8
            }
            CB_SRA_D => {
                let value = self.sra8(self.regs.d);
                self.regs.d = value;
                8
            }
            CB_SRA_E => {
                let value = self.sra8(self.regs.e);
                self.regs.e = value;
                8
            }
            CB_SRA_H => {
                let value = self.sra8(self.regs.h);
                self.regs.h = value;
                8
            }
            CB_SRA_L => {
                let value = self.sra8(self.regs.l);
                self.regs.l = value;
                8
            }
            CB_SRA_MEM_HL => {
                let value = self.sra8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SRA_A => {
                let value = self.sra8(self.regs.a);
                self.regs.a = value;
                8
            }
            CB_SWAP_B => {
                let value = self.swap8(self.regs.b);
                self.regs.b = value;
                8
            }
            CB_SWAP_C => {
                let value = self.swap8(self.regs.c);
                self.regs.c = value;
                8
            }
            CB_SWAP_D => {
                let value = self.swap8(self.regs.d);
                self.regs.d = value;
                8
            }
            CB_SWAP_E => {
                let value = self.swap8(self.regs.e);
                self.regs.e = value;
                8
            }
            CB_SWAP_H => {
                let value = self.swap8(self.regs.h);
                self.regs.h = value;
                8
            }
            CB_SWAP_L => {
                let value = self.swap8(self.regs.l);
                self.regs.l = value;
                8
            }
            CB_SWAP_MEM_HL => {
                let value = self.swap8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SWAP_A => {
                let value = self.swap8(self.regs.a);
                self.regs.a = value;
                8
            }
            CB_SRL_B => {
                let value = self.srl8(self.regs.b);
                self.regs.b = value;
                8
            }
            CB_SRL_C => {
                let value = self.srl8(self.regs.c);
                self.regs.c = value;
                8
            }
            CB_SRL_D => {
                let value = self.srl8(self.regs.d);
                self.regs.d = value;
                8
            }
            CB_SRL_E => {
                let value = self.srl8(self.regs.e);
                self.regs.e = value;
                8
            }
            CB_SRL_H => {
                let value = self.srl8(self.regs.h);
                self.regs.h = value;
                8
            }
            CB_SRL_L => {
                let value = self.srl8(self.regs.l);
                self.regs.l = value;
                8
            }
            CB_SRL_MEM_HL => {
                let value = self.srl8(bus.read(self.regs.hl()));
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SRL_A => {
                let value = self.srl8(self.regs.a);
                self.regs.a = value;
                8
            }
            CB_BIT_0_B => {
                self.bit8(0, self.regs.b);
                8
            }
            CB_BIT_0_C => {
                self.bit8(0, self.regs.c);
                8
            }
            CB_BIT_0_D => {
                self.bit8(0, self.regs.d);
                8
            }
            CB_BIT_0_E => {
                self.bit8(0, self.regs.e);
                8
            }
            CB_BIT_0_H => {
                self.bit8(0, self.regs.h);
                8
            }
            CB_BIT_0_L => {
                self.bit8(0, self.regs.l);
                8
            }
            CB_BIT_0_MEM_HL => {
                self.bit8(0, bus.read(self.regs.hl()));
                12
            }
            CB_BIT_0_A => {
                self.bit8(0, self.regs.a);
                8
            }
            CB_BIT_1_B => {
                self.bit8(1, self.regs.b);
                8
            }
            CB_BIT_1_C => {
                self.bit8(1, self.regs.c);
                8
            }
            CB_BIT_1_D => {
                self.bit8(1, self.regs.d);
                8
            }
            CB_BIT_1_E => {
                self.bit8(1, self.regs.e);
                8
            }
            CB_BIT_1_H => {
                self.bit8(1, self.regs.h);
                8
            }
            CB_BIT_1_L => {
                self.bit8(1, self.regs.l);
                8
            }
            CB_BIT_1_MEM_HL => {
                self.bit8(1, bus.read(self.regs.hl()));
                12
            }
            CB_BIT_1_A => {
                self.bit8(1, self.regs.a);
                8
            }
            CB_BIT_2_B => {
                self.bit8(2, self.regs.b);
                8
            }
            CB_BIT_2_C => {
                self.bit8(2, self.regs.c);
                8
            }
            CB_BIT_2_D => {
                self.bit8(2, self.regs.d);
                8
            }
            CB_BIT_2_E => {
                self.bit8(2, self.regs.e);
                8
            }
            CB_BIT_2_H => {
                self.bit8(2, self.regs.h);
                8
            }
            CB_BIT_2_L => {
                self.bit8(2, self.regs.l);
                8
            }
            CB_BIT_2_MEM_HL => {
                self.bit8(2, bus.read(self.regs.hl()));
                12
            }
            CB_BIT_2_A => {
                self.bit8(2, self.regs.a);
                8
            }
            CB_BIT_3_B => {
                self.bit8(3, self.regs.b);
                8
            }
            CB_BIT_3_C => {
                self.bit8(3, self.regs.c);
                8
            }
            CB_BIT_3_D => {
                self.bit8(3, self.regs.d);
                8
            }
            CB_BIT_3_E => {
                self.bit8(3, self.regs.e);
                8
            }
            CB_BIT_3_H => {
                self.bit8(3, self.regs.h);
                8
            }
            CB_BIT_3_L => {
                self.bit8(3, self.regs.l);
                8
            }
            CB_BIT_3_MEM_HL => {
                self.bit8(3, bus.read(self.regs.hl()));
                12
            }
            CB_BIT_3_A => {
                self.bit8(3, self.regs.a);
                8
            }
            CB_BIT_4_B => {
                self.bit8(4, self.regs.b);
                8
            }
            CB_BIT_4_C => {
                self.bit8(4, self.regs.c);
                8
            }
            CB_BIT_4_D => {
                self.bit8(4, self.regs.d);
                8
            }
            CB_BIT_4_E => {
                self.bit8(4, self.regs.e);
                8
            }
            CB_BIT_4_H => {
                self.bit8(4, self.regs.h);
                8
            }
            CB_BIT_4_L => {
                self.bit8(4, self.regs.l);
                8
            }
            CB_BIT_4_MEM_HL => {
                self.bit8(4, bus.read(self.regs.hl()));
                12
            }
            CB_BIT_4_A => {
                self.bit8(4, self.regs.a);
                8
            }
            CB_BIT_5_B => {
                self.bit8(5, self.regs.b);
                8
            }
            CB_BIT_5_C => {
                self.bit8(5, self.regs.c);
                8
            }
            CB_BIT_5_D => {
                self.bit8(5, self.regs.d);
                8
            }
            CB_BIT_5_E => {
                self.bit8(5, self.regs.e);
                8
            }
            CB_BIT_5_H => {
                self.bit8(5, self.regs.h);
                8
            }
            CB_BIT_5_L => {
                self.bit8(5, self.regs.l);
                8
            }
            CB_BIT_5_MEM_HL => {
                self.bit8(5, bus.read(self.regs.hl()));
                12
            }
            CB_BIT_5_A => {
                self.bit8(5, self.regs.a);
                8
            }
            CB_BIT_6_B => {
                self.bit8(6, self.regs.b);
                8
            }
            CB_BIT_6_C => {
                self.bit8(6, self.regs.c);
                8
            }
            CB_BIT_6_D => {
                self.bit8(6, self.regs.d);
                8
            }
            CB_BIT_6_E => {
                self.bit8(6, self.regs.e);
                8
            }
            CB_BIT_6_H => {
                self.bit8(6, self.regs.h);
                8
            }
            CB_BIT_6_L => {
                self.bit8(6, self.regs.l);
                8
            }
            CB_BIT_6_MEM_HL => {
                self.bit8(6, bus.read(self.regs.hl()));
                12
            }
            CB_BIT_6_A => {
                self.bit8(6, self.regs.a);
                8
            }
            CB_BIT_7_B => {
                self.bit8(7, self.regs.b);
                8
            }
            CB_BIT_7_C => {
                self.bit8(7, self.regs.c);
                8
            }
            CB_BIT_7_D => {
                self.bit8(7, self.regs.d);
                8
            }
            CB_BIT_7_E => {
                self.bit8(7, self.regs.e);
                8
            }
            CB_BIT_7_H => {
                self.bit8(7, self.regs.h);
                8
            }
            CB_BIT_7_L => {
                self.bit8(7, self.regs.l);
                8
            }
            CB_BIT_7_MEM_HL => {
                self.bit8(7, bus.read(self.regs.hl()));
                12
            }
            CB_BIT_7_A => {
                self.bit8(7, self.regs.a);
                8
            }
            CB_RES_0_B => {
                let value = self.regs.b & !(1 << 0);
                self.regs.b = value;
                8
            }
            CB_RES_0_C => {
                let value = self.regs.c & !(1 << 0);
                self.regs.c = value;
                8
            }
            CB_RES_0_D => {
                let value = self.regs.d & !(1 << 0);
                self.regs.d = value;
                8
            }
            CB_RES_0_E => {
                let value = self.regs.e & !(1 << 0);
                self.regs.e = value;
                8
            }
            CB_RES_0_H => {
                let value = self.regs.h & !(1 << 0);
                self.regs.h = value;
                8
            }
            CB_RES_0_L => {
                let value = self.regs.l & !(1 << 0);
                self.regs.l = value;
                8
            }
            CB_RES_0_MEM_HL => {
                let value = bus.read(self.regs.hl()) & !(1 << 0);
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RES_0_A => {
                let value = self.regs.a & !(1 << 0);
                self.regs.a = value;
                8
            }
            CB_RES_1_B => {
                let value = self.regs.b & !(1 << 1);
                self.regs.b = value;
                8
            }
            CB_RES_1_C => {
                let value = self.regs.c & !(1 << 1);
                self.regs.c = value;
                8
            }
            CB_RES_1_D => {
                let value = self.regs.d & !(1 << 1);
                self.regs.d = value;
                8
            }
            CB_RES_1_E => {
                let value = self.regs.e & !(1 << 1);
                self.regs.e = value;
                8
            }
            CB_RES_1_H => {
                let value = self.regs.h & !(1 << 1);
                self.regs.h = value;
                8
            }
            CB_RES_1_L => {
                let value = self.regs.l & !(1 << 1);
                self.regs.l = value;
                8
            }
            CB_RES_1_MEM_HL => {
                let value = bus.read(self.regs.hl()) & !(1 << 1);
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RES_1_A => {
                let value = self.regs.a & !(1 << 1);
                self.regs.a = value;
                8
            }
            CB_RES_2_B => {
                let value = self.regs.b & !(1 << 2);
                self.regs.b = value;
                8
            }
            CB_RES_2_C => {
                let value = self.regs.c & !(1 << 2);
                self.regs.c = value;
                8
            }
            CB_RES_2_D => {
                let value = self.regs.d & !(1 << 2);
                self.regs.d = value;
                8
            }
            CB_RES_2_E => {
                let value = self.regs.e & !(1 << 2);
                self.regs.e = value;
                8
            }
            CB_RES_2_H => {
                let value = self.regs.h & !(1 << 2);
                self.regs.h = value;
                8
            }
            CB_RES_2_L => {
                let value = self.regs.l & !(1 << 2);
                self.regs.l = value;
                8
            }
            CB_RES_2_MEM_HL => {
                let value = bus.read(self.regs.hl()) & !(1 << 2);
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RES_2_A => {
                let value = self.regs.a & !(1 << 2);
                self.regs.a = value;
                8
            }
            CB_RES_3_B => {
                let value = self.regs.b & !(1 << 3);
                self.regs.b = value;
                8
            }
            CB_RES_3_C => {
                let value = self.regs.c & !(1 << 3);
                self.regs.c = value;
                8
            }
            CB_RES_3_D => {
                let value = self.regs.d & !(1 << 3);
                self.regs.d = value;
                8
            }
            CB_RES_3_E => {
                let value = self.regs.e & !(1 << 3);
                self.regs.e = value;
                8
            }
            CB_RES_3_H => {
                let value = self.regs.h & !(1 << 3);
                self.regs.h = value;
                8
            }
            CB_RES_3_L => {
                let value = self.regs.l & !(1 << 3);
                self.regs.l = value;
                8
            }
            CB_RES_3_MEM_HL => {
                let value = bus.read(self.regs.hl()) & !(1 << 3);
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RES_3_A => {
                let value = self.regs.a & !(1 << 3);
                self.regs.a = value;
                8
            }
            CB_RES_4_B => {
                let value = self.regs.b & !(1 << 4);
                self.regs.b = value;
                8
            }
            CB_RES_4_C => {
                let value = self.regs.c & !(1 << 4);
                self.regs.c = value;
                8
            }
            CB_RES_4_D => {
                let value = self.regs.d & !(1 << 4);
                self.regs.d = value;
                8
            }
            CB_RES_4_E => {
                let value = self.regs.e & !(1 << 4);
                self.regs.e = value;
                8
            }
            CB_RES_4_H => {
                let value = self.regs.h & !(1 << 4);
                self.regs.h = value;
                8
            }
            CB_RES_4_L => {
                let value = self.regs.l & !(1 << 4);
                self.regs.l = value;
                8
            }
            CB_RES_4_MEM_HL => {
                let value = bus.read(self.regs.hl()) & !(1 << 4);
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RES_4_A => {
                let value = self.regs.a & !(1 << 4);
                self.regs.a = value;
                8
            }
            CB_RES_5_B => {
                let value = self.regs.b & !(1 << 5);
                self.regs.b = value;
                8
            }
            CB_RES_5_C => {
                let value = self.regs.c & !(1 << 5);
                self.regs.c = value;
                8
            }
            CB_RES_5_D => {
                let value = self.regs.d & !(1 << 5);
                self.regs.d = value;
                8
            }
            CB_RES_5_E => {
                let value = self.regs.e & !(1 << 5);
                self.regs.e = value;
                8
            }
            CB_RES_5_H => {
                let value = self.regs.h & !(1 << 5);
                self.regs.h = value;
                8
            }
            CB_RES_5_L => {
                let value = self.regs.l & !(1 << 5);
                self.regs.l = value;
                8
            }
            CB_RES_5_MEM_HL => {
                let value = bus.read(self.regs.hl()) & !(1 << 5);
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RES_5_A => {
                let value = self.regs.a & !(1 << 5);
                self.regs.a = value;
                8
            }
            CB_RES_6_B => {
                let value = self.regs.b & !(1 << 6);
                self.regs.b = value;
                8
            }
            CB_RES_6_C => {
                let value = self.regs.c & !(1 << 6);
                self.regs.c = value;
                8
            }
            CB_RES_6_D => {
                let value = self.regs.d & !(1 << 6);
                self.regs.d = value;
                8
            }
            CB_RES_6_E => {
                let value = self.regs.e & !(1 << 6);
                self.regs.e = value;
                8
            }
            CB_RES_6_H => {
                let value = self.regs.h & !(1 << 6);
                self.regs.h = value;
                8
            }
            CB_RES_6_L => {
                let value = self.regs.l & !(1 << 6);
                self.regs.l = value;
                8
            }
            CB_RES_6_MEM_HL => {
                let value = bus.read(self.regs.hl()) & !(1 << 6);
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RES_6_A => {
                let value = self.regs.a & !(1 << 6);
                self.regs.a = value;
                8
            }
            CB_RES_7_B => {
                let value = self.regs.b & !(1 << 7);
                self.regs.b = value;
                8
            }
            CB_RES_7_C => {
                let value = self.regs.c & !(1 << 7);
                self.regs.c = value;
                8
            }
            CB_RES_7_D => {
                let value = self.regs.d & !(1 << 7);
                self.regs.d = value;
                8
            }
            CB_RES_7_E => {
                let value = self.regs.e & !(1 << 7);
                self.regs.e = value;
                8
            }
            CB_RES_7_H => {
                let value = self.regs.h & !(1 << 7);
                self.regs.h = value;
                8
            }
            CB_RES_7_L => {
                let value = self.regs.l & !(1 << 7);
                self.regs.l = value;
                8
            }
            CB_RES_7_MEM_HL => {
                let value = bus.read(self.regs.hl()) & !(1 << 7);
                bus.write(self.regs.hl(), value);
                16
            }
            CB_RES_7_A => {
                let value = self.regs.a & !(1 << 7);
                self.regs.a = value;
                8
            }
            CB_SET_0_B => {
                let value = self.regs.b | 1 << 0;
                self.regs.b = value;
                8
            }
            CB_SET_0_C => {
                let value = self.regs.c | 1 << 0;
                self.regs.c = value;
                8
            }
            CB_SET_0_D => {
                let value = self.regs.d | 1 << 0;
                self.regs.d = value;
                8
            }
            CB_SET_0_E => {
                let value = self.regs.e | 1 << 0;
                self.regs.e = value;
                8
            }
            CB_SET_0_H => {
                let value = self.regs.h | 1 << 0;
                self.regs.h = value;
                8
            }
            CB_SET_0_L => {
                let value = self.regs.l | 1 << 0;
                self.regs.l = value;
                8
            }
            CB_SET_0_MEM_HL => {
                let value = bus.read(self.regs.hl()) | 1 << 0;
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SET_0_A => {
                let value = self.regs.a | 1 << 0;
                self.regs.a = value;
                8
            }
            CB_SET_1_B => {
                let value = self.regs.b | 1 << 1;
                self.regs.b = value;
                8
            }
            CB_SET_1_C => {
                let value = self.regs.c | 1 << 1;
                self.regs.c = value;
                8
            }
            CB_SET_1_D => {
                let value = self.regs.d | 1 << 1;
                self.regs.d = value;
                8
            }
            CB_SET_1_E => {
                let value = self.regs.e | 1 << 1;
                self.regs.e = value;
                8
            }
            CB_SET_1_H => {
                let value = self.regs.h | 1 << 1;
                self.regs.h = value;
                8
            }
            CB_SET_1_L => {
                let value = self.regs.l | 1 << 1;
                self.regs.l = value;
                8
            }
            CB_SET_1_MEM_HL => {
                let value = bus.read(self.regs.hl()) | 1 << 1;
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SET_1_A => {
                let value = self.regs.a | 1 << 1;
                self.regs.a = value;
                8
            }
            CB_SET_2_B => {
                let value = self.regs.b | 1 << 2;
                self.regs.b = value;
                8
            }
            CB_SET_2_C => {
                let value = self.regs.c | 1 << 2;
                self.regs.c = value;
                8
            }
            CB_SET_2_D => {
                let value = self.regs.d | 1 << 2;
                self.regs.d = value;
                8
            }
            CB_SET_2_E => {
                let value = self.regs.e | 1 << 2;
                self.regs.e = value;
                8
            }
            CB_SET_2_H => {
                let value = self.regs.h | 1 << 2;
                self.regs.h = value;
                8
            }
            CB_SET_2_L => {
                let value = self.regs.l | 1 << 2;
                self.regs.l = value;
                8
            }
            CB_SET_2_MEM_HL => {
                let value = bus.read(self.regs.hl()) | 1 << 2;
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SET_2_A => {
                let value = self.regs.a | 1 << 2;
                self.regs.a = value;
                8
            }
            CB_SET_3_B => {
                let value = self.regs.b | 1 << 3;
                self.regs.b = value;
                8
            }
            CB_SET_3_C => {
                let value = self.regs.c | 1 << 3;
                self.regs.c = value;
                8
            }
            CB_SET_3_D => {
                let value = self.regs.d | 1 << 3;
                self.regs.d = value;
                8
            }
            CB_SET_3_E => {
                let value = self.regs.e | 1 << 3;
                self.regs.e = value;
                8
            }
            CB_SET_3_H => {
                let value = self.regs.h | 1 << 3;
                self.regs.h = value;
                8
            }
            CB_SET_3_L => {
                let value = self.regs.l | 1 << 3;
                self.regs.l = value;
                8
            }
            CB_SET_3_MEM_HL => {
                let value = bus.read(self.regs.hl()) | 1 << 3;
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SET_3_A => {
                let value = self.regs.a | 1 << 3;
                self.regs.a = value;
                8
            }
            CB_SET_4_B => {
                let value = self.regs.b | 1 << 4;
                self.regs.b = value;
                8
            }
            CB_SET_4_C => {
                let value = self.regs.c | 1 << 4;
                self.regs.c = value;
                8
            }
            CB_SET_4_D => {
                let value = self.regs.d | 1 << 4;
                self.regs.d = value;
                8
            }
            CB_SET_4_E => {
                let value = self.regs.e | 1 << 4;
                self.regs.e = value;
                8
            }
            CB_SET_4_H => {
                let value = self.regs.h | 1 << 4;
                self.regs.h = value;
                8
            }
            CB_SET_4_L => {
                let value = self.regs.l | 1 << 4;
                self.regs.l = value;
                8
            }
            CB_SET_4_MEM_HL => {
                let value = bus.read(self.regs.hl()) | 1 << 4;
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SET_4_A => {
                let value = self.regs.a | 1 << 4;
                self.regs.a = value;
                8
            }
            CB_SET_5_B => {
                let value = self.regs.b | 1 << 5;
                self.regs.b = value;
                8
            }
            CB_SET_5_C => {
                let value = self.regs.c | 1 << 5;
                self.regs.c = value;
                8
            }
            CB_SET_5_D => {
                let value = self.regs.d | 1 << 5;
                self.regs.d = value;
                8
            }
            CB_SET_5_E => {
                let value = self.regs.e | 1 << 5;
                self.regs.e = value;
                8
            }
            CB_SET_5_H => {
                let value = self.regs.h | 1 << 5;
                self.regs.h = value;
                8
            }
            CB_SET_5_L => {
                let value = self.regs.l | 1 << 5;
                self.regs.l = value;
                8
            }
            CB_SET_5_MEM_HL => {
                let value = bus.read(self.regs.hl()) | 1 << 5;
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SET_5_A => {
                let value = self.regs.a | 1 << 5;
                self.regs.a = value;
                8
            }
            CB_SET_6_B => {
                let value = self.regs.b | 1 << 6;
                self.regs.b = value;
                8
            }
            CB_SET_6_C => {
                let value = self.regs.c | 1 << 6;
                self.regs.c = value;
                8
            }
            CB_SET_6_D => {
                let value = self.regs.d | 1 << 6;
                self.regs.d = value;
                8
            }
            CB_SET_6_E => {
                let value = self.regs.e | 1 << 6;
                self.regs.e = value;
                8
            }
            CB_SET_6_H => {
                let value = self.regs.h | 1 << 6;
                self.regs.h = value;
                8
            }
            CB_SET_6_L => {
                let value = self.regs.l | 1 << 6;
                self.regs.l = value;
                8
            }
            CB_SET_6_MEM_HL => {
                let value = bus.read(self.regs.hl()) | 1 << 6;
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SET_6_A => {
                let value = self.regs.a | 1 << 6;
                self.regs.a = value;
                8
            }
            CB_SET_7_B => {
                let value = self.regs.b | 1 << 7;
                self.regs.b = value;
                8
            }
            CB_SET_7_C => {
                let value = self.regs.c | 1 << 7;
                self.regs.c = value;
                8
            }
            CB_SET_7_D => {
                let value = self.regs.d | 1 << 7;
                self.regs.d = value;
                8
            }
            CB_SET_7_E => {
                let value = self.regs.e | 1 << 7;
                self.regs.e = value;
                8
            }
            CB_SET_7_H => {
                let value = self.regs.h | 1 << 7;
                self.regs.h = value;
                8
            }
            CB_SET_7_L => {
                let value = self.regs.l | 1 << 7;
                self.regs.l = value;
                8
            }
            CB_SET_7_MEM_HL => {
                let value = bus.read(self.regs.hl()) | 1 << 7;
                bus.write(self.regs.hl(), value);
                16
            }
            CB_SET_7_A => {
                let value = self.regs.a | 1 << 7;
                self.regs.a = value;
                8
            }
        }
    }
}
