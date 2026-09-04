pub struct Envelope {
    initial: u8,
    increasing: bool,
    period: u8,
    volume: u8,
    timer: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            initial: 0,
            increasing: false,
            period: 0,
            volume: 0,
            timer: 0,
        }
    }

    pub fn dac_enabled(&self) -> bool {
        self.initial > 0 || self.increasing
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }

    pub fn read(&self) -> u8 {
        self.initial << 4 | (self.increasing as u8) << 3 | self.period
    }

    pub fn write(&mut self, value: u8) {
        self.initial = value >> 4;
        self.increasing = value & 0x08 != 0;
        self.period = value & 0x07;
    }

    pub fn trigger(&mut self) {
        self.volume = self.initial;
        self.timer = if self.period == 0 { 8 } else { self.period };
    }

    pub fn clock(&mut self) {
        if self.period == 0 {
            return;
        }
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer != 0 {
            return;
        }

        self.timer = self.period;
        if self.increasing && self.volume < 15 {
            self.volume += 1;
        } else if !self.increasing && self.volume > 0 {
            self.volume -= 1;
        }
    }
}
