pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

const SCANLINES: u32 = 154; // 144 visible + 10 of VBlank
const SCANLINE_CYCLES: u32 = 456;
const FRAME_CYCLES: u32 = SCANLINES * SCANLINE_CYCLES;

mod addr {
    pub const LCDC: u16 = 0xFF40;
    pub const STAT: u16 = 0xFF41;
    pub const SCY: u16 = 0xFF42;
    pub const SCX: u16 = 0xFF43;
    pub const LY: u16 = 0xFF44;
    pub const LYC: u16 = 0xFF45;
    pub const DMA: u16 = 0xFF46;
    pub const BGP: u16 = 0xFF47;
    pub const OBP0: u16 = 0xFF48;
    pub const OBP1: u16 = 0xFF49;
    pub const WY: u16 = 0xFF4A;
    pub const WX: u16 = 0xFF4B;
}

pub struct Ppu {
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    ly: u8, // LCD Y-coordinate register
    cycles: u32,
    framebuffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    frame_ready: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            ly: 0,
            cycles: 0,
            framebuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            frame_ready: false,
        }
    }

    pub fn tick(&mut self, t_cycles: u8) -> u8 {
        self.cycles += t_cycles as u32;
        if self.cycles >= FRAME_CYCLES {
            self.cycles -= FRAME_CYCLES;
            self.frame_ready = true;
        }

        self.ly = (self.cycles / SCANLINE_CYCLES) as u8;
        0
    }

    pub fn framebuffer(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.framebuffer
    }

    pub fn take_frame_ready(&mut self) -> bool {
        std::mem::take(&mut self.frame_ready)
    }

    pub fn read_vram(&self, offset: u16) -> u8 {
        self.vram[offset as usize]
    }

    pub fn write_vram(&mut self, offset: u16, value: u8) {
        self.vram[offset as usize] = value;
    }

    pub fn read_oam(&self, offset: u16) -> u8 {
        self.oam[offset as usize]
    }

    pub fn write_oam(&mut self, offset: u16, value: u8) {
        self.oam[offset as usize] = value;
    }

    pub fn read_reg(&self, address: u16) -> u8 {
        match address {
            addr::LY => self.ly,
            _ => 0xFF,
        }
    }

    pub fn write_reg(&mut self, _address: u16, _value: u8) {}
}
