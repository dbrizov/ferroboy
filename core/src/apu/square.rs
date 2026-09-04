use crate::apu::envelope::Envelope;
use crate::apu::length::Length;

const DUTY: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

const MAX_PERIOD: u16 = 2047;

pub struct Square {
    has_sweep: bool,
    active: bool,
    duty: usize,
    duty_step: usize,
    period: u16,
    timer: u16,
    length: Length,
    envelope: Envelope,

    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,
    sweep_timer: u8,
    sweep_enabled: bool,
    sweep_shadow: u16,
    sweep_negate_used: bool,
}

impl Square {
    pub fn new(has_sweep: bool) -> Self {
        Self {
            has_sweep,
            active: false,
            duty: 0,
            duty_step: 0,
            period: 0,
            timer: 0,
            length: Length::new(64),
            envelope: Envelope::new(),
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            sweep_timer: 0,
            sweep_enabled: false,
            sweep_shadow: 0,
            sweep_negate_used: false,
        }
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.timer = (MAX_PERIOD + 1 - self.period) * 4;
            self.duty_step = (self.duty_step + 1) % 8;
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

    pub fn clock_sweep(&mut self) {
        if !self.has_sweep {
            return;
        }
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }
        if self.sweep_timer != 0 {
            return;
        }

        self.sweep_timer = if self.sweep_period == 0 {
            8
        } else {
            self.sweep_period
        };
        if !self.sweep_enabled || self.sweep_period == 0 {
            return;
        }

        let next = self.next_sweep_period();
        if next <= MAX_PERIOD && self.sweep_shift > 0 {
            self.sweep_shadow = next;
            self.period = next;
            if self.next_sweep_period() > MAX_PERIOD {
                self.active = false;
            }
        } else if next > MAX_PERIOD {
            self.active = false;
        }
    }

    fn next_sweep_period(&mut self) -> u16 {
        let delta = self.sweep_shadow >> self.sweep_shift;
        if self.sweep_negate {
            self.sweep_negate_used = true;
            self.sweep_shadow.wrapping_sub(delta)
        } else {
            self.sweep_shadow + delta
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn sample(&self) -> f32 {
        if !self.active || !self.envelope.dac_enabled() {
            return 0.0;
        }

        let amplitude = DUTY[self.duty][self.duty_step] * self.envelope.volume();
        amplitude as f32 / 7.5 - 1.0
    }

    pub fn read(&self, register: u16) -> u8 {
        match register {
            0 => self.sweep_period << 4 | (self.sweep_negate as u8) << 3 | self.sweep_shift | 0x80,
            1 => (self.duty as u8) << 6 | 0x3F,
            2 => self.envelope.read(),
            4 => (self.length.enabled() as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    pub fn write_length(&mut self, value: u8) {
        self.length.set(value & 0x3F);
    }

    pub fn power_off(&mut self) {
        let counter = self.length.counter();
        *self = Self::new(self.has_sweep);
        self.length.restore(counter);
    }

    pub fn write(&mut self, register: u16, value: u8, next_clocks: bool) {
        match register {
            0 => {
                let leaving_negate = self.sweep_negate && value & 0x08 == 0;
                self.sweep_period = value >> 4 & 0x07;
                self.sweep_negate = value & 0x08 != 0;
                self.sweep_shift = value & 0x07;
                if leaving_negate && self.sweep_negate_used {
                    self.active = false;
                }
            }
            1 => {
                self.duty = (value >> 6) as usize;
                self.length.set(value & 0x3F);
            }
            2 => {
                self.envelope.write(value);
                if !self.envelope.dac_enabled() {
                    self.active = false;
                }
            }
            3 => self.period = self.period & 0x0700 | value as u16,
            4 => {
                self.period = self.period & 0x00FF | (value as u16 & 0x07) << 8;
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
        self.timer = (MAX_PERIOD + 1 - self.period) * 4;
        self.envelope.trigger();

        if self.has_sweep {
            self.sweep_negate_used = false;
            self.sweep_shadow = self.period;
            self.sweep_timer = if self.sweep_period == 0 {
                8
            } else {
                self.sweep_period
            };
            self.sweep_enabled = self.sweep_period > 0 || self.sweep_shift > 0;
            if self.sweep_shift > 0 && self.next_sweep_period() > MAX_PERIOD {
                self.active = false;
            }
        }
    }
}
