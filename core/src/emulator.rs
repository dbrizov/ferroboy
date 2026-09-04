use crate::boot_rom::{BOOT_ROM, CGB_BOOT_ROM};
use crate::bus::Bus;
use crate::cartridge::{self, Cartridge};
use crate::cpu::Cpu;
use crate::joypad::Button;
use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
}

impl Emulator {
    pub fn new(rom: &[u8]) -> Self {
        Self::from_cartridge(cartridge::load(rom), cartridge::is_cgb(rom))
    }

    pub fn unplugged() -> Self {
        Self::from_cartridge(cartridge::unplugged(), false)
    }

    fn from_cartridge(cartridge: Box<dyn Cartridge>, cgb: bool) -> Self {
        let boot_rom: &[u8] = if cgb { &CGB_BOOT_ROM } else { &BOOT_ROM };
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(cartridge, boot_rom, cgb),
        }
    }

    pub fn is_cgb(&self) -> bool {
        self.bus.is_cgb()
    }

    pub fn set_button(&mut self, button: Button, pressed: bool) {
        self.bus.set_button(button, pressed);
    }

    pub fn take_samples(&mut self) -> Vec<(f32, f32)> {
        self.bus.take_samples()
    }

    pub fn battery_ram(&self) -> Option<&[u8]> {
        self.bus.battery_ram()
    }

    pub fn load_battery_ram(&mut self, saved: &[u8]) {
        self.bus.load_battery_ram(saved);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.bus.peek(address)
    }

    pub fn step(&mut self) -> u8 {
        let cycles = self.cpu.step(&mut self.bus);
        let spent = self.bus.take_access_cycles();
        debug_assert!(
            spent <= cycles,
            "{spent} cycles of bus access in a {cycles} cycle instruction"
        );
        self.bus.tick(cycles.saturating_sub(spent));
        cycles
    }

    pub fn run_frame(&mut self) {
        while !self.bus.ppu.take_frame_ready() {
            self.step();
        }
    }

    pub fn take_serial_byte(&mut self) -> Option<u8> {
        self.bus.serial.take_byte()
    }

    pub fn framebuffer(&self) -> &[u16; SCREEN_WIDTH * SCREEN_HEIGHT] {
        self.bus.ppu.framebuffer()
    }
}
