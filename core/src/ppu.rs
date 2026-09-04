use crate::interrupts;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

const SCANLINES: u8 = 154;
const VISIBLE_SCANLINES: u8 = 144;
const OAM_SCAN_DOTS: u32 = 80;
const DRAWING_DOTS: u32 = 172;
const SCANLINE_DOTS: u32 = 456;
const FRAME_DOTS: u32 = SCANLINE_DOTS * SCANLINES as u32;

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

const LCDC_ENABLED: u8 = 1 << 7;

mod stat {
    pub const LYC_EQUALS_LY: u8 = 1 << 6;
    pub const OAM_SCAN: u8 = 1 << 5;
    pub const VBLANK: u8 = 1 << 4;
    pub const HBLANK: u8 = 1 << 3;
    pub const COINCIDENCE: u8 = 1 << 2;
    pub const WRITABLE: u8 = 0x78;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

pub struct Ppu {
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    mode: Mode,
    dots: u32,
    frame_ready: bool,
    framebuffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],

    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            mode: Mode::OamScan,
            dots: 0,
            frame_ready: false,
            framebuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],

            lcdc: 0x91, // what the boot ROM leaves behind: LCD on, BG on
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
        }
    }

    pub fn tick(&mut self, t_cycles: u8) -> u8 {
        self.dots += t_cycles as u32;

        // No VBlank while the LCD is off, but the frontend still needs a frame
        // boundary to present on, or run_frame never returns.
        if self.lcdc & LCDC_ENABLED == 0 {
            if self.dots >= FRAME_DOTS {
                self.dots -= FRAME_DOTS;
                self.frame_ready = true;
            }
            return 0;
        }

        match self.mode {
            Mode::OamScan if self.dots >= OAM_SCAN_DOTS => {
                self.dots -= OAM_SCAN_DOTS;
                self.mode = Mode::Drawing;
                0
            }
            Mode::Drawing if self.dots >= DRAWING_DOTS => {
                self.dots -= DRAWING_DOTS;
                self.mode = Mode::HBlank;
                self.stat_interrupt(stat::HBLANK)
            }
            Mode::HBlank if self.dots >= SCANLINE_DOTS - OAM_SCAN_DOTS - DRAWING_DOTS => {
                self.dots -= SCANLINE_DOTS - OAM_SCAN_DOTS - DRAWING_DOTS;
                self.next_line()
            }
            Mode::VBlank if self.dots >= SCANLINE_DOTS => {
                self.dots -= SCANLINE_DOTS;
                self.next_line()
            }
            _ => 0,
        }
    }

    fn next_line(&mut self) -> u8 {
        self.ly += 1;

        let mut raised = 0;
        if self.ly == VISIBLE_SCANLINES {
            self.mode = Mode::VBlank;
            self.frame_ready = true;
            raised |= interrupts::VBLANK | self.stat_interrupt(stat::VBLANK);
        } else if self.ly >= SCANLINES {
            self.ly = 0;
            self.mode = Mode::OamScan;
            raised |= self.stat_interrupt(stat::OAM_SCAN);
        } else if self.mode == Mode::HBlank {
            self.mode = Mode::OamScan;
            raised |= self.stat_interrupt(stat::OAM_SCAN);
        }

        raised | self.check_coincidence()
    }

    fn stat_interrupt(&self, source: u8) -> u8 {
        if self.stat & source != 0 {
            interrupts::STAT
        } else {
            0
        }
    }

    fn check_coincidence(&mut self) -> u8 {
        if self.ly == self.lyc {
            self.stat |= stat::COINCIDENCE;
            self.stat_interrupt(stat::LYC_EQUALS_LY)
        } else {
            self.stat &= !stat::COINCIDENCE;
            0
        }
    }

    pub fn take_frame_ready(&mut self) -> bool {
        std::mem::take(&mut self.frame_ready)
    }

    pub fn framebuffer(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.framebuffer
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
            addr::LCDC => self.lcdc,
            addr::STAT => self.stat | 0x80 | self.mode as u8,
            addr::SCY => self.scy,
            addr::SCX => self.scx,
            addr::LY => self.ly,
            addr::LYC => self.lyc,
            addr::BGP => self.bgp,
            addr::OBP0 => self.obp0,
            addr::OBP1 => self.obp1,
            addr::WY => self.wy,
            addr::WX => self.wx,
            _ => 0xFF,
        }
    }

    pub fn write_reg(&mut self, address: u16, value: u8) {
        match address {
            addr::LCDC => {
                let was_on = self.lcdc & LCDC_ENABLED != 0;
                let is_on = value & LCDC_ENABLED != 0;
                let toggled = was_on != is_on;
                self.lcdc = value;

                if toggled {
                    self.ly = 0;
                    self.dots = 0;
                    self.mode = Mode::HBlank;
                }
            }
            addr::STAT => self.stat = (self.stat & !stat::WRITABLE) | (value & stat::WRITABLE),
            addr::SCY => self.scy = value,
            addr::SCX => self.scx = value,
            addr::LY => {}
            addr::LYC => self.lyc = value,
            addr::DMA => {}
            addr::BGP => self.bgp = value,
            addr::OBP0 => self.obp0 = value,
            addr::OBP1 => self.obp1 = value,
            addr::WY => self.wy = value,
            addr::WX => self.wx = value,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Advances one T-cycle at a time so a mode boundary cannot be stepped over,
    /// collecting every interrupt raised along the way.
    fn run(ppu: &mut Ppu, dots: u32) -> u8 {
        let mut raised = 0;
        for _ in 0..dots {
            raised |= ppu.tick(1);
        }
        raised
    }

    #[test]
    fn a_visible_line_is_oam_scan_then_drawing_then_hblank() {
        let mut ppu = Ppu::new();
        assert_eq!(ppu.mode, Mode::OamScan);

        run(&mut ppu, 79);
        assert_eq!(ppu.mode, Mode::OamScan, "80 dots of OAM scan");
        run(&mut ppu, 1);
        assert_eq!(ppu.mode, Mode::Drawing);

        run(&mut ppu, 171);
        assert_eq!(ppu.mode, Mode::Drawing, "172 dots of drawing");
        run(&mut ppu, 1);
        assert_eq!(ppu.mode, Mode::HBlank);

        run(&mut ppu, 203);
        assert_eq!(ppu.ly, 0, "still on line 0 after 455 dots");
        run(&mut ppu, 1);
        assert_eq!(ppu.ly, 1, "456 dots is exactly one scanline");
        assert_eq!(ppu.mode, Mode::OamScan);
    }

    #[test]
    fn a_frame_is_70224_dots() {
        let mut ppu = Ppu::new();
        run(&mut ppu, 70_224);

        assert!(ppu.take_frame_ready());
        assert_eq!(ppu.ly, 0, "wrapped back to the top");
        assert_eq!(ppu.mode, Mode::OamScan);
        assert!(!ppu.take_frame_ready(), "taking it clears it");
    }

    #[test]
    fn vblank_starts_at_line_144_and_lasts_ten_lines() {
        let mut ppu = Ppu::new();
        let raised = run(&mut ppu, 144 * SCANLINE_DOTS);

        assert_eq!(ppu.ly, 144);
        assert_eq!(ppu.mode, Mode::VBlank);
        assert!(raised & interrupts::VBLANK != 0);
        assert!(ppu.take_frame_ready());

        run(&mut ppu, 9 * SCANLINE_DOTS);
        assert_eq!(ppu.ly, 153, "ten lines of VBlank, 144 through 153");
        assert_eq!(ppu.mode, Mode::VBlank);
    }

    #[test]
    fn the_lcd_being_off_freezes_the_state_machine_but_not_the_clock() {
        let mut ppu = Ppu::new();
        run(&mut ppu, 1000);
        assert_ne!(ppu.ly, 0);

        ppu.write_reg(addr::LCDC, 0x00);
        assert_eq!(ppu.ly, 0, "switching off restarts at the top of a frame");

        let raised = run(&mut ppu, FRAME_DOTS);
        assert_eq!(ppu.ly, 0, "no scanline advances while it is off");
        assert_eq!(ppu.mode, Mode::HBlank);
        assert_eq!(raised, 0, "and nothing interrupts");
        assert!(
            ppu.take_frame_ready(),
            "but frames still come, or run_frame never returns"
        );
    }

    #[test]
    fn coincidence_is_reported_whether_or_not_it_interrupts() {
        let mut ppu = Ppu::new();
        ppu.write_reg(addr::LYC, 2);

        let raised = run(&mut ppu, 2 * SCANLINE_DOTS);
        assert_eq!(ppu.ly, 2);
        assert!(ppu.read_reg(addr::STAT) & stat::COINCIDENCE != 0);
        assert_eq!(raised & interrupts::STAT, 0, "source not enabled");

        // Now enable it and come round again.
        let mut ppu = Ppu::new();
        ppu.write_reg(addr::LYC, 2);
        ppu.write_reg(addr::STAT, stat::LYC_EQUALS_LY);

        let raised = run(&mut ppu, 2 * SCANLINE_DOTS);
        assert!(raised & interrupts::STAT != 0);
    }

    #[test]
    fn stat_reports_the_mode_and_refuses_to_have_it_written() {
        let mut ppu = Ppu::new();
        assert_eq!(ppu.read_reg(addr::STAT) & 0x03, Mode::OamScan as u8);

        ppu.write_reg(addr::STAT, 0xFF);
        assert_eq!(
            ppu.read_reg(addr::STAT) & 0x03,
            Mode::OamScan as u8,
            "the mode is the PPU's to report, not the game's to set"
        );
        assert!(ppu.read_reg(addr::STAT) & 0x80 != 0, "bit 7 reads as 1");
    }

    #[test]
    fn ly_is_read_only() {
        let mut ppu = Ppu::new();
        run(&mut ppu, 3 * SCANLINE_DOTS);
        ppu.write_reg(addr::LY, 100);

        assert_eq!(ppu.ly, 3);
    }
}
