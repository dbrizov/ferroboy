#[cfg(test)]
mod tests;

mod addr {
    pub const SB: u16 = 0xFF01;
    pub const SC: u16 = 0xFF02;
}

pub struct Serial {
    sb: u8,
    sc: u8,
    output: Option<u8>,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0,
            output: None,
        }
    }

    pub fn tick(&mut self, _t_cycles: u8) -> u8 {
        0
    }

    pub fn take_byte(&mut self) -> Option<u8> {
        self.output.take()
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            addr::SB => self.sb,
            addr::SC => self.sc | 0x7E, // bits 1-6 do not exist and read as 1
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            addr::SB => self.sb = value,
            addr::SC => {
                self.sc = value;
                // 0x81 starts a transfer clocked by us.
                // With no cable attached the byte goes nowhere,
                // so capture it and report completion at once by clearing the transfer bit.
                if value & 0x81 == 0x81 {
                    self.output = Some(self.sb);
                    self.sc &= !0x80;
                }
            }
            _ => {}
        }
    }
}
