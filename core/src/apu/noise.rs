use crate::apu::envelope::Envelope;
use crate::apu::length::Length;

const DIVISORS: [u16; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

pub struct Noise {
    active: bool,
    timer: u16,
    lfsr: u16,
    shift: u8,
    short_width: bool,
    divisor: u8,
    length: Length,
    envelope: Envelope,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            active: false,
            timer: 0,
            lfsr: 0x7FFF,
            shift: 0,
            short_width: false,
            divisor: 0,
            length: Length::new(64),
            envelope: Envelope::new(),
        }
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer != 0 {
            return;
        }

        self.timer = DIVISORS[self.divisor as usize] << self.shift;

        let feedback = (self.lfsr ^ self.lfsr >> 1) & 1;
        self.lfsr = self.lfsr >> 1 | feedback << 14;
        if self.short_width {
            self.lfsr = self.lfsr & !(1 << 6) | feedback << 6;
        }
    }

    pub fn clock_length(&mut self) {
        if self.length.clock() {
            self.active = false;
        }
    }

    pub fn clock_envelope(&mut self) {
        self.envelope.clock();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn sample(&self) -> f32 {
        if !self.active || !self.envelope.dac_enabled() {
            return 0.0;
        }

        let amplitude = (!self.lfsr & 1) as u8 * self.envelope.volume();
        amplitude as f32 / 7.5 - 1.0
    }

    pub fn read(&self, register: u16) -> u8 {
        match register {
            2 => self.envelope.read(),
            3 => self.shift << 4 | (self.short_width as u8) << 3 | self.divisor,
            4 => (self.length.enabled() as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    pub fn write_length(&mut self, value: u8) {
        self.length.set(value & 0x3F);
    }

    pub fn power_off(&mut self) {
        let counter = self.length.counter();
        *self = Self::new();
        self.length.restore(counter);
    }

    pub fn write(&mut self, register: u16, value: u8, next_clocks: bool) {
        match register {
            1 => self.length.set(value & 0x3F),
            2 => {
                self.envelope.write(value);
                if !self.envelope.dac_enabled() {
                    self.active = false;
                }
            }
            3 => {
                self.shift = value >> 4;
                self.short_width = value & 0x08 != 0;
                self.divisor = value & 0x07;
            }
            4 => {
                let trigger = value & 0x80 != 0;
                if self
                    .length
                    .write_control(value & 0x40 != 0, trigger, next_clocks)
                {
                    self.active = false;
                }
                if trigger {
                    self.trigger(next_clocks);
                }
            }
            _ => {}
        }
    }

    fn trigger(&mut self, next_clocks: bool) {
        self.active = self.envelope.dac_enabled();
        self.length.trigger(next_clocks);
        self.timer = DIVISORS[self.divisor as usize] << self.shift;
        self.lfsr = 0x7FFF;
        self.envelope.trigger();
    }
}
