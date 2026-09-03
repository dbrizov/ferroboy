mod opcodes;

use crate::bus::Bus;
use crate::cpu::opcodes::*;

pub const FLAG_Z: u8 = 1 << 7;
pub const FLAG_N: u8 = 1 << 6;
pub const FLAG_H: u8 = 1 << 5;
pub const FLAG_C: u8 = 1 << 4;

pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    /// The state the DMG boot ROM leaves behind. M2 may run the real boot ROM
    /// instead, in which case these become zeros and PC starts at 0x0000.
    fn post_boot() -> Self {
        Self {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
        }
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value as u8) & 0xF0; // the low nibble of F does not exist on hardware
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn has_flags(&self, flags: u8) -> bool {
        self.f & flags != 0
    }

    pub fn set_flags(&mut self, flags: u8, on: bool) {
        if on {
            self.f |= flags;
        } else {
            self.f &= !flags;
        }
    }
}

pub struct Cpu {
    regs: Registers,
    sp: u16,
    pc: u16,
    ime: bool,
    ime_pending: bool,
    halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::post_boot(),
            sp: 0xFFFE,
            pc: 0x0100,
            ime: false,
            ime_pending: false,
            halted: false,
        }
    }

    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        if self.halted {
            return 4; // idle, but the clock still runs
        }

        let opcode = self.fetch(bus);
        self.execute(opcode, bus)
    }

    fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn execute(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        match opcode {
            NOP => 4,
            HALT => {
                self.halted = true;
                4
            }
            PREFIX_CB => {
                let cb_opcode = self.fetch(bus);
                self.execute_cb(cb_opcode, bus)
            }
            _ => panic!(
                "unimplemented opcode {:#04X} ({}) at {:#06X}",
                opcode,
                MNEMONIC[opcode as usize],
                self.pc.wrapping_sub(1)
            ),
        }
    }

    fn execute_cb(&mut self, cb_opcode: u8, _bus: &mut Bus) -> u8 {
        panic!(
            "unimplemented CB opcode {:#04X} ({}) at {:#06X}",
            cb_opcode,
            CB_MNEMONIC[cb_opcode as usize],
            self.pc.wrapping_sub(2)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_is_the_high_byte_of_bc() {
        let mut regs = Registers::post_boot();
        regs.set_bc(0x1234);
        assert_eq!(regs.b, 0x12);
        assert_eq!(regs.c, 0x34);
        assert_eq!(regs.bc(), 0x1234);
    }

    #[test]
    fn the_low_nibble_of_f_does_not_exist() {
        let mut regs = Registers::post_boot();
        regs.set_af(0xFFFF);
        assert_eq!(regs.f, 0xF0);
        assert_eq!(regs.af(), 0xFFF0);
    }

    #[test]
    fn flags_round_trip() {
        let mut regs = Registers::post_boot();
        regs.f = 0;
        regs.set_flags(FLAG_Z | FLAG_C, true);
        assert!(regs.has_flags(FLAG_Z));
        assert!(regs.has_flags(FLAG_C));
        assert!(!regs.has_flags(FLAG_N));
        regs.set_flags(FLAG_Z, false);
        assert!(!regs.has_flags(FLAG_Z));
        assert_eq!(regs.f, FLAG_C);
    }
}
