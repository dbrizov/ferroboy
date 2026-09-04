use crate::apu::Apu;
use crate::cartridge::Cartridge;
use crate::joypad::{Button, Joypad};
use crate::ppu::Ppu;
use crate::serial::Serial;
use crate::timer::Timer;

const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const CART_RAM_START: u16 = 0xA000;
const CART_RAM_END: u16 = 0xBFFF;
const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;
const ECHO_START: u16 = 0xE000;
const ECHO_END: u16 = 0xFDFF;
const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
const UNUSABLE_START: u16 = 0xFEA0;
const UNUSABLE_END: u16 = 0xFEFF;
const JOYPAD: u16 = 0xFF00;
const SERIAL_START: u16 = 0xFF01;
const SERIAL_END: u16 = 0xFF02;
const TIMER_START: u16 = 0xFF04;
const TIMER_END: u16 = 0xFF07;
const IF: u16 = 0xFF0F;
const APU_START: u16 = 0xFF10;
const APU_END: u16 = 0xFF3F;
const OAM_DMA: u16 = 0xFF46;
const PPU_REG_START: u16 = 0xFF40;
const PPU_REG_END: u16 = 0xFF4B;
const KEY1: u16 = 0xFF4D;
const VBK: u16 = 0xFF4F;
const HDMA_SOURCE_HIGH: u16 = 0xFF51;
const HDMA_SOURCE_LOW: u16 = 0xFF52;
const HDMA_DEST_HIGH: u16 = 0xFF53;
const HDMA_DEST_LOW: u16 = 0xFF54;
const HDMA_CONTROL: u16 = 0xFF55;
const PALETTES_START: u16 = 0xFF68;
const PALETTES_END: u16 = 0xFF6B;
const SVBK: u16 = 0xFF70;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const BOOT_ROM_DISABLE: u16 = 0xFF50;
const IE: u16 = 0xFFFF;

const WRAM_BANK_BYTES: usize = 0x1000;
const HDMA_BLOCK_BYTES: u16 = 0x10;

// The CGB boot ROM covers 0x0000-0x08FF but leaves the cartridge header
// window at 0x0100-0x01FF visible, so the entry point stays readable.
const HEADER_WINDOW_START: u16 = 0x0100;
const HEADER_WINDOW_END: u16 = 0x01FF;

#[cfg(test)]
mod tests;

pub struct Bus {
    wram: [u8; 0x8000],
    hram: [u8; 0x7F],
    access_cycles: u8,
    pub intf: u8, // IF register
    pub inte: u8, // IE register

    cgb: bool,
    svbk: u8,
    double_speed: bool,
    speed_switch_armed: bool,
    dot_debt: u8,
    hdma_source: u16,
    hdma_dest: u16,
    hdma_blocks: u8,
    hdma_hblank_active: bool,

    cartridge: Box<dyn Cartridge>,
    boot_rom: Vec<u8>,
    boot_rom_mapped: bool,
    pub apu: Apu,
    pub ppu: Ppu,
    pub timer: Timer,
    pub joypad: Joypad,
    pub serial: Serial,
}

impl Bus {
    pub fn new(cartridge: Box<dyn Cartridge>, boot_rom: &[u8], cgb: bool) -> Self {
        Self {
            wram: [0; 0x8000],
            hram: [0; 0x7F],
            access_cycles: 0,
            intf: 0,
            inte: 0,
            cgb,
            svbk: 0,
            double_speed: false,
            speed_switch_armed: false,
            dot_debt: 0,
            hdma_source: 0,
            hdma_dest: 0,
            hdma_blocks: 0,
            hdma_hblank_active: false,
            cartridge,
            boot_rom: boot_rom.to_vec(),
            boot_rom_mapped: true,
            apu: Apu::new(cgb),
            ppu: Ppu::new(cgb),
            timer: Timer::new(),
            joypad: Joypad::new(),
            serial: Serial::new(),
        }
    }

    pub fn is_cgb(&self) -> bool {
        self.cgb
    }

    fn access_cycle(&mut self) {
        self.tick(4);
        self.access_cycles += 4;
    }

    pub fn take_access_cycles(&mut self) -> u8 {
        std::mem::take(&mut self.access_cycles)
    }

    pub fn tick(&mut self, t_cycles: u8) {
        let dots = self.dot_cycles(t_cycles);
        self.intf |= self.apu.tick(dots);
        self.intf |= self.ppu.tick(dots);
        if self.ppu.take_hblank_start() && self.hdma_hblank_active {
            self.copy_hdma_block();
        }
        self.intf |= self.timer.tick(t_cycles);
        self.intf |= self.serial.tick(t_cycles);
    }

    // In double speed the CPU clock doubles but the PPU and APU do not: they
    // receive half the cycles, with odd remainders carried to the next call.
    fn dot_cycles(&mut self, t_cycles: u8) -> u8 {
        if !self.double_speed {
            return t_cycles;
        }

        let total = self.dot_debt + t_cycles;
        self.dot_debt = total & 1;
        total >> 1
    }

    pub fn switch_speed(&mut self) {
        if self.speed_switch_armed {
            self.double_speed = !self.double_speed;
            self.speed_switch_armed = false;
        }
    }

    fn oam_dma(&mut self, source_high: u8) {
        let source = (source_high as u16) << 8;
        for offset in 0..OAM_END - OAM_START + 1 {
            let byte = self.peek(source + offset);
            self.ppu.write_oam(offset, byte);
        }
    }

    // Instant, like OAM DMA; real hardware takes 8 M-cycles per block.
    fn copy_hdma_block(&mut self) {
        for _ in 0..HDMA_BLOCK_BYTES {
            let byte = self.peek(self.hdma_source);
            self.ppu.write_vram(self.hdma_dest & 0x1FFF, byte);
            self.hdma_source = self.hdma_source.wrapping_add(1);
            self.hdma_dest += 1;
        }

        self.hdma_blocks -= 1;
        if self.hdma_blocks == 0 {
            self.hdma_hblank_active = false;
        }
    }

    fn write_hdma_control(&mut self, value: u8) {
        if self.hdma_hblank_active && value & 0x80 == 0 {
            self.hdma_hblank_active = false;
            return;
        }

        self.hdma_blocks = (value & 0x7F) + 1;
        if value & 0x80 == 0 {
            while self.hdma_blocks > 0 {
                self.copy_hdma_block();
            }
        } else {
            self.hdma_hblank_active = true;
        }
    }

    fn read_hdma_control(&self) -> u8 {
        if self.hdma_hblank_active {
            self.hdma_blocks - 1
        } else if self.hdma_blocks == 0 {
            0xFF
        } else {
            0x80 | (self.hdma_blocks - 1)
        }
    }

    fn wram_index(&self, offset: u16) -> usize {
        let offset = offset as usize;
        if offset < WRAM_BANK_BYTES {
            offset
        } else {
            self.wram_bank() * WRAM_BANK_BYTES + offset - WRAM_BANK_BYTES
        }
    }

    fn wram_bank(&self) -> usize {
        match self.svbk as usize & 0x07 {
            0 => 1,
            bank => bank,
        }
    }

    pub fn set_button(&mut self, button: Button, pressed: bool) {
        self.intf |= self.joypad.set(button, pressed);
    }

    pub fn take_samples(&mut self) -> Vec<(f32, f32)> {
        self.apu.take_samples()
    }

    pub fn battery_ram(&self) -> Option<&[u8]> {
        self.cartridge.battery_ram()
    }

    pub fn load_battery_ram(&mut self, saved: &[u8]) {
        self.cartridge.load_battery_ram(saved);
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.access_cycle();
        self.peek(address)
    }

    pub fn peek(&self, address: u16) -> u8 {
        match address {
            ROM_START..=ROM_END
                if self.boot_rom_mapped
                    && (address as usize) < self.boot_rom.len()
                    && !(HEADER_WINDOW_START..=HEADER_WINDOW_END).contains(&address) =>
            {
                self.boot_rom[address as usize]
            }
            ROM_START..=ROM_END => self.cartridge.read_rom(address),
            VRAM_START..=VRAM_END => self.ppu.read_vram(address - VRAM_START),
            CART_RAM_START..=CART_RAM_END => self.cartridge.read_ram(address - CART_RAM_START),
            WRAM_START..=WRAM_END => self.wram[self.wram_index(address - WRAM_START)],
            ECHO_START..=ECHO_END => self.wram[self.wram_index(address - ECHO_START)],
            OAM_START..=OAM_END => self.ppu.read_oam(address - OAM_START),
            UNUSABLE_START..=UNUSABLE_END => 0xFF,
            JOYPAD => self.joypad.read(),
            SERIAL_START..=SERIAL_END => self.serial.read(address),
            TIMER_START..=TIMER_END => self.timer.read(address),
            IF => self.intf | 0xE0,
            APU_START..=APU_END => self.apu.read(address),
            KEY1 if self.cgb => {
                (self.double_speed as u8) << 7 | 0x7E | self.speed_switch_armed as u8
            }
            VBK | PALETTES_START..=PALETTES_END if self.cgb => self.ppu.read_reg(address),
            HDMA_CONTROL if self.cgb => self.read_hdma_control(),
            SVBK if self.cgb => self.svbk | 0xF8,
            PPU_REG_START..=PPU_REG_END => self.ppu.read_reg(address),
            HRAM_START..=HRAM_END => self.hram[(address - HRAM_START) as usize],
            IE => self.inte,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.access_cycle();
        match address {
            ROM_START..=ROM_END => self.cartridge.write_rom(address, value),
            VRAM_START..=VRAM_END => self.ppu.write_vram(address - VRAM_START, value),
            CART_RAM_START..=CART_RAM_END => {
                self.cartridge.write_ram(address - CART_RAM_START, value)
            }
            WRAM_START..=WRAM_END => self.wram[self.wram_index(address - WRAM_START)] = value,
            ECHO_START..=ECHO_END => self.wram[self.wram_index(address - ECHO_START)] = value,
            OAM_START..=OAM_END => self.ppu.write_oam(address - OAM_START, value),
            UNUSABLE_START..=UNUSABLE_END => {}
            JOYPAD => self.joypad.write(value),
            SERIAL_START..=SERIAL_END => self.serial.write(address, value),
            TIMER_START..=TIMER_END => self.timer.write(address, value),
            IF => self.intf = value & 0x1F,
            APU_START..=APU_END => self.apu.write(address, value),
            OAM_DMA => self.oam_dma(value),
            KEY1 if self.cgb => self.speed_switch_armed = value & 1 != 0,
            VBK | PALETTES_START..=PALETTES_END if self.cgb => self.ppu.write_reg(address, value),
            HDMA_SOURCE_HIGH if self.cgb => {
                self.hdma_source = self.hdma_source & 0x00FF | (value as u16) << 8
            }
            HDMA_SOURCE_LOW if self.cgb => {
                self.hdma_source = self.hdma_source & 0xFF00 | (value & 0xF0) as u16
            }
            HDMA_DEST_HIGH if self.cgb => {
                self.hdma_dest = self.hdma_dest & 0x00FF | ((value & 0x1F) as u16) << 8
            }
            HDMA_DEST_LOW if self.cgb => {
                self.hdma_dest = self.hdma_dest & 0xFF00 | (value & 0xF0) as u16
            }
            HDMA_CONTROL if self.cgb => self.write_hdma_control(value),
            SVBK if self.cgb => self.svbk = value & 0x07,
            PPU_REG_START..=PPU_REG_END => self.ppu.write_reg(address, value),
            HRAM_START..=HRAM_END => self.hram[(address - HRAM_START) as usize] = value,
            BOOT_ROM_DISABLE => self.boot_rom_mapped = false,
            IE => self.inte = value,
            _ => {}
        }
    }
}
