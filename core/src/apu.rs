#[cfg(test)]
mod tests;

mod envelope;
mod length;
mod noise;
mod square;
mod wave;

use crate::apu::noise::Noise;
use crate::apu::square::Square;
use crate::apu::wave::Wave;

pub const SAMPLE_RATE: u32 = 48_000;

const CYCLES_PER_SECOND: u32 = 4_194_304;
const CYCLES_PER_FRAME_STEP: u32 = 8192;
// The DMG's output capacitors drain by 0.999958 per T-cycle; over the 87.4
// cycles between samples that compounds to this. Without it every channel
// parks at -1.0 once its envelope empties and the mixer carries the offset.
const CHARGE_FACTOR: f32 = 0.996;
const FRAME_STEPS: u8 = 8;

mod addr {
    pub const NR10: u16 = 0xFF10;
    pub const NR11: u16 = 0xFF11;
    pub const NR14: u16 = 0xFF14;
    pub const NR21: u16 = 0xFF16;
    pub const NR24: u16 = 0xFF19;
    pub const NR30: u16 = 0xFF1A;
    pub const NR31: u16 = 0xFF1B;
    pub const NR34: u16 = 0xFF1E;
    pub const NR41: u16 = 0xFF20;
    pub const NR44: u16 = 0xFF23;
    pub const NR50: u16 = 0xFF24;
    pub const NR51: u16 = 0xFF25;
    pub const NR52: u16 = 0xFF26;
    pub const WAVE_START: u16 = 0xFF30;
    pub const WAVE_END: u16 = 0xFF3F;
}

const NR52_ENABLED: u8 = 1 << 7;

pub struct Apu {
    enabled: bool,
    square1: Square,
    square2: Square,
    wave: Wave,
    noise: Noise,
    volumes: u8,
    panning: u8,
    frame_step: u8,
    frame_counter: u32,
    sample_counter: u32,
    capacitors: (f32, f32),
    samples: Vec<(f32, f32)>,
}

impl Apu {
    pub fn new(cgb: bool) -> Self {
        Self {
            enabled: false,
            square1: Square::new(true),
            square2: Square::new(false),
            wave: Wave::new(cgb),
            noise: Noise::new(),
            volumes: 0,
            panning: 0,
            frame_step: 0,
            frame_counter: 0,
            sample_counter: 0,
            capacitors: (0.0, 0.0),
            samples: Vec::new(),
        }
    }

    pub fn tick(&mut self, t_cycles: u8) -> u8 {
        for _ in 0..t_cycles {
            self.frame_counter += 1;
            if self.frame_counter >= CYCLES_PER_FRAME_STEP {
                self.frame_counter = 0;
                self.advance_frame_sequencer();
            }

            self.square1.tick();
            self.square2.tick();
            self.wave.tick();
            self.noise.tick();

            self.sample_counter += SAMPLE_RATE;
            if self.sample_counter >= CYCLES_PER_SECOND {
                self.sample_counter -= CYCLES_PER_SECOND;
                let sample = self.mix();
                self.samples.push(sample);
            }
        }

        0
    }

    fn advance_frame_sequencer(&mut self) {
        if self.frame_step.is_multiple_of(2) {
            self.square1.clock_length();
            self.square2.clock_length();
            self.wave.clock_length();
            self.noise.clock_length();
        }
        if self.frame_step == 2 || self.frame_step == 6 {
            self.square1.clock_sweep();
        }
        if self.frame_step == 7 {
            self.square1.clock_envelope();
            self.square2.clock_envelope();
            self.noise.clock_envelope();
        }

        self.frame_step = (self.frame_step + 1) % FRAME_STEPS;
    }

    fn mix(&mut self) -> (f32, f32) {
        if !self.enabled {
            return (0.0, 0.0);
        }

        let channels = [
            self.square1.sample(),
            self.square2.sample(),
            self.wave.sample(),
            self.noise.sample(),
        ];

        let mut left = 0.0;
        let mut right = 0.0;
        for (index, amplitude) in channels.iter().enumerate() {
            if self.panning >> (index + 4) & 1 != 0 {
                left += amplitude;
            }
            if self.panning >> index & 1 != 0 {
                right += amplitude;
            }
        }

        let left_volume = (self.volumes >> 4 & 0x07) as f32 + 1.0;
        let right_volume = (self.volumes & 0x07) as f32 + 1.0;

        (
            high_pass(left / 4.0 * left_volume / 8.0, &mut self.capacitors.0),
            high_pass(right / 4.0 * right_volume / 8.0, &mut self.capacitors.1),
        )
    }

    pub fn take_samples(&mut self) -> Vec<(f32, f32)> {
        std::mem::take(&mut self.samples)
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            addr::NR10..=addr::NR14 => self.square1.read(address - addr::NR10),
            addr::NR21..=addr::NR24 => self.square2.read(address - addr::NR21 + 1),
            addr::NR30..=addr::NR34 => self.wave.read(address - addr::NR30),
            addr::NR41..=addr::NR44 => self.noise.read(address - addr::NR41 + 1),
            addr::NR50 => self.volumes,
            addr::NR51 => self.panning,
            addr::NR52 => self.status(),
            addr::WAVE_START..=addr::WAVE_END => self.wave.read_ram(address - addr::WAVE_START),
            _ => 0xFF,
        }
    }

    fn status(&self) -> u8 {
        let mut status = 0x70;
        if self.enabled {
            status |= NR52_ENABLED;
        }
        status |= self.square1.is_active() as u8;
        status |= (self.square2.is_active() as u8) << 1;
        status |= (self.wave.is_active() as u8) << 2;
        status |= (self.noise.is_active() as u8) << 3;
        status
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address == addr::NR52 {
            self.set_enabled(value & NR52_ENABLED != 0);
            return;
        }

        // Wave RAM is always reachable, and the DMG keeps its length counters
        // loadable with the APU off even though the rest of the registers are
        // inert until it comes back on.
        if !self.enabled {
            match address {
                addr::NR11 => self.square1.write_length(value),
                addr::NR21 => self.square2.write_length(value),
                addr::NR31 => self.wave.write_length(value),
                addr::NR41 => self.noise.write_length(value),
                addr::WAVE_START..=addr::WAVE_END => {
                    self.wave.write_ram(address - addr::WAVE_START, value)
                }
                _ => {}
            }
            return;
        }

        let next_clocks = self.next_clocks_length();
        match address {
            addr::NR10..=addr::NR14 => self.square1.write(address - addr::NR10, value, next_clocks),
            addr::NR21..=addr::NR24 => {
                self.square2
                    .write(address - addr::NR21 + 1, value, next_clocks)
            }
            addr::NR30..=addr::NR34 => self.wave.write(address - addr::NR30, value, next_clocks),
            addr::NR41..=addr::NR44 => {
                self.noise
                    .write(address - addr::NR41 + 1, value, next_clocks)
            }
            addr::NR50 => self.volumes = value,
            addr::NR51 => self.panning = value,
            addr::WAVE_START..=addr::WAVE_END => {
                self.wave.write_ram(address - addr::WAVE_START, value)
            }
            _ => {}
        }
    }

    // The sequencer holds the step it will run next, and the even steps are the
    // ones that clock length.
    fn next_clocks_length(&self) -> bool {
        self.frame_step.is_multiple_of(2)
    }

    fn set_enabled(&mut self, enabled: bool) {
        if self.enabled && !enabled {
            self.square1.power_off();
            self.square2.power_off();
            self.wave.power_off();
            self.noise.power_off();
            self.volumes = 0;
            self.panning = 0;
        }
        if !self.enabled && enabled {
            self.frame_step = 0;
        }

        self.enabled = enabled;
    }
}

fn high_pass(sample: f32, capacitor: &mut f32) -> f32 {
    let filtered = sample - *capacitor;
    *capacitor = sample - filtered * CHARGE_FACTOR;
    filtered
}
