use crate::interrupts;

mod addr {
    pub const DIV: u16 = 0xFF04;
    pub const TIMA: u16 = 0xFF05;
    pub const TMA: u16 = 0xFF06;
    pub const TAC: u16 = 0xFF07;
}

const TAC_ENABLE: u8 = 1 << 2;
const TIMA_BIT: [u16; 4] = [9, 3, 5, 7];

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn tick(&mut self, t_cycles: u8) -> u8 {
        let mut interrupts = 0;
        for _ in 0..t_cycles {
            let previous = self.div;
            self.div = self.div.wrapping_add(1);
            if self.is_falling_edge(previous) {
                interrupts |= self.increment_tima();
            }
        }

        interrupts
    }

    fn selected_bit(&self) -> u16 {
        1 << TIMA_BIT[(self.tac & 0x03) as usize]
    }

    fn is_falling_edge(&self, previous: u16) -> bool {
        let bit = self.selected_bit();
        self.tac & TAC_ENABLE != 0 && previous & bit != 0 && self.div & bit == 0
    }

    fn increment_tima(&mut self) -> u8 {
        let (result, overflowed) = self.tima.overflowing_add(1);
        self.tima = if overflowed { self.tma } else { result };

        if overflowed { interrupts::TIMER } else { 0 }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            addr::DIV => (self.div >> 8) as u8,
            addr::TIMA => self.tima,
            addr::TMA => self.tma,
            addr::TAC => self.tac | 0xF8, // bits 3-7 do not exist and read as 1
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            addr::DIV => self.div = 0,
            addr::TIMA => self.tima = value,
            addr::TMA => self.tma = value,
            addr::TAC => self.tac = value & 0x07,
            _ => {}
        }
    }
}
