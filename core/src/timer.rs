mod addr {
    pub const DIV: u16 = 0xFF04;
    pub const TIMA: u16 = 0xFF05;
    pub const TMA: u16 = 0xFF06;
    pub const TAC: u16 = 0xFF07;
}

pub struct Timer {
    div: u8,
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

    pub fn tick(&mut self, _t_cycles: u8) -> u8 {
        0
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            addr::DIV => self.div,
            addr::TIMA => self.tima,
            addr::TMA => self.tma,
            addr::TAC => self.tac | 0xF8, // bits 3-7 do not exist and read as 1
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            addr::DIV => self.div = 0, // any write resets DIV, whatever the value
            addr::TIMA => self.tima = value,
            addr::TMA => self.tma = value,
            addr::TAC => self.tac = value & 0x07,
            _ => {}
        }
    }
}
