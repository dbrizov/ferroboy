use crate::apu::length::Length;

const RAM_BYTES: usize = 16;
const SAMPLES: usize = 32;
const MAX_PERIOD: u16 = 2047;

// The first fetch after a trigger runs 6 T-cycles late (binjgb's
// WAVE_TRIGGER_DELAY_TICKS); every later fetch is period-spaced.
const TRIGGER_DELAY: u16 = 6;

pub struct Wave {
    cgb: bool,
    active: bool,
    dac_enabled: bool,
    volume: u8,
    period: u16,
    timer: u16,
    position: usize,
    fetch_age: u32,
    length: Length,
    ram: [u8; RAM_BYTES],
}

impl Wave {
    pub fn new(cgb: bool) -> Self {
        Self {
            cgb,
            active: false,
            dac_enabled: false,
            volume: 0,
            period: 0,
            timer: 0,
            position: 0,
            fetch_age: u32::MAX,
            length: Length::new(256),
            ram: [0; RAM_BYTES],
        }
    }

    pub fn tick(&mut self) {
        if !self.active {
            return;
        }

        self.fetch_age = self.fetch_age.saturating_add(1);
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.timer = (MAX_PERIOD + 1 - self.period) * 2;
            self.position = (self.position + 1) % SAMPLES;
            self.fetch_age = 0;
        }
    }

    pub fn clock_length(&mut self) {
        if self.length.clock() {
            self.active = false;
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn sample(&self) -> f32 {
        if !self.active || !self.dac_enabled {
            return 0.0;
        }

        let byte = self.ram[self.position / 2];
        let nibble = if self.position.is_multiple_of(2) {
            byte >> 4
        } else {
            byte & 0x0F
        };
        let amplitude = match self.volume {
            0 => 0,
            volume => nibble >> (volume - 1),
        };

        amplitude as f32 / 7.5 - 1.0
    }

    pub fn write_length(&mut self, value: u8) {
        self.length.set(value);
    }

    pub fn power_off(&mut self) {
        let ram = self.ram;
        let counter = self.length.counter();
        *self = Self::new(self.cgb);
        self.ram = ram;
        self.length.restore(counter);
    }

    pub fn read(&self, register: u16) -> u8 {
        match register {
            0 => (self.dac_enabled as u8) << 7 | 0x7F,
            2 => self.volume << 5 | 0x9F,
            4 => (self.length.enabled() as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, register: u16, value: u8, next_clocks: bool) {
        match register {
            0 => {
                self.dac_enabled = value & 0x80 != 0;
                if !self.dac_enabled {
                    self.active = false;
                }
            }
            1 => self.length.set(value),
            2 => self.volume = value >> 5 & 0x03,
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
        // DMG bug: a trigger landing 2 T-cycles before a fetch corrupts wave
        // RAM with the byte about to be read - byte 0 alone if it comes from
        // the first four, the whole aligned four-byte block otherwise.
        if self.active && self.timer == 2 {
            self.corrupt_ram();
        }

        self.active = self.dac_enabled;
        self.length.trigger(next_clocks);
        self.timer = (MAX_PERIOD + 1 - self.period) * 2 + TRIGGER_DELAY;
        self.position = 0;
    }

    fn corrupt_ram(&mut self) {
        let next_byte = (self.position + 1) % SAMPLES / 2;
        if next_byte < 4 {
            self.ram[0] = self.ram[next_byte];
        } else {
            self.ram
                .copy_within(next_byte & !3..(next_byte & !3) + 4, 0);
        }
    }

    // The DMG exposes wave RAM only on the 2 T-cycles of the channel's own
    // fetch, and the access lands on the byte being fetched, not the one
    // addressed.
    fn access_window_open(&self) -> bool {
        self.cgb || self.fetch_age <= 1
    }

    pub fn read_ram(&self, offset: u16) -> u8 {
        if self.active {
            if self.access_window_open() {
                return self.ram[self.position / 2];
            }
            return 0xFF;
        }
        self.ram[offset as usize]
    }

    pub fn write_ram(&mut self, offset: u16, value: u8) {
        if self.active {
            if self.access_window_open() {
                self.ram[self.position / 2] = value;
            }
            return;
        }
        self.ram[offset as usize] = value;
    }
}
