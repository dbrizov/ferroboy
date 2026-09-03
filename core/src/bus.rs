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
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const IE: u16 = 0xFFFF;

pub struct Bus {
    wram: [u8; 0x2000],
    hram: [u8; 0x7F],
    intf: u8, // IF register
    inte: u8, // IE register
}

impl Bus {
    pub fn new() -> Self {
        Self {
            wram: [0; 0x2000],
            hram: [0; 0x7F],
            intf: 0,
            inte: 0,
        }
    }

    pub fn tick(&mut self, _t_cycles: u8) {}

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            ROM_START..=ROM_END => 0x76, // HALT for now
            VRAM_START..=VRAM_END => 0xFF,
            CART_RAM_START..=CART_RAM_END => 0xFF,
            WRAM_START..=WRAM_END => self.wram[(addr - WRAM_START) as usize],
            ECHO_START..=ECHO_END => self.wram[(addr - ECHO_START) as usize],
            OAM_START..=OAM_END => 0xFF,
            UNUSABLE_START..=UNUSABLE_END => 0xFF,
            JOYPAD => 0xFF,
            SERIAL_START..=SERIAL_END => 0xFF,
            TIMER_START..=TIMER_END => 0xFF,
            IF => self.intf | 0xE0,
            APU_START..=APU_END => 0xFF,
            PPU_REG_START..=PPU_REG_END => 0xFF,
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize],
            IE => self.inte,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            ROM_START..=ROM_END => {}
            VRAM_START..=VRAM_END => {}
            CART_RAM_START..=CART_RAM_END => {}
            WRAM_START..=WRAM_END => self.wram[(addr - WRAM_START) as usize] = value,
            ECHO_START..=ECHO_END => self.wram[(addr - ECHO_START) as usize] = value,
            OAM_START..=OAM_END => {}
            UNUSABLE_START..=UNUSABLE_END => {}
            JOYPAD => {}
            SERIAL_START..=SERIAL_END => {}
            TIMER_START..=TIMER_END => {}
            IF => self.intf = value & 0x1F,
            APU_START..=APU_END => {}
            OAM_DMA => {}
            PPU_REG_START..=PPU_REG_END => {}
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize] = value,
            IE => self.inte = value,
            _ => {}
        }
    }
}
