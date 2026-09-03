use crate::bus::Bus;
use crate::cpu::Cpu;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const FRAME_CYCLES: u32 = 70_224;

pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
    framebuffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    frames: u64,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(),
            framebuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            frames: 0,
        }
    }

    pub fn step(&mut self) -> u8 {
        let cycles = self.cpu.step(&mut self.bus);
        self.bus.tick(cycles);
        cycles
    }

    pub fn run_frame(&mut self) {
        let mut cycles: u32 = 0;
        while cycles < FRAME_CYCLES {
            cycles += self.step() as u32;
        }

        self.frames += 1;
        let shade = (self.frames / 30 % 4) as u8;
        self.framebuffer = [shade; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn framebuffer(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.framebuffer
    }
}
