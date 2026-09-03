use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    pub fn step(&mut self) -> u8 {
        let cycles = self.cpu.step(&mut self.bus);
        self.bus.tick(cycles);
        cycles
    }

    pub fn run_frame(&mut self) {
        while self.bus.ppu.take_frame_ready() {
            self.step();
        }
    }

    pub fn framebuffer(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.bus.ppu.framebuffer()
    }
}
