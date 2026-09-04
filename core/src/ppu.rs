use crate::interrupts;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

const SCANLINES: u8 = 154;
const VISIBLE_SCANLINES: u8 = 144;
const OAM_SCAN_DOTS: u32 = 80;
const DRAWING_DOTS: u32 = 172;
const SCANLINE_DOTS: u32 = 456;
const FRAME_DOTS: u32 = SCANLINE_DOTS * SCANLINES as u32;

const LCDC_ENABLED: u8 = 1 << 7;
const TILE_BYTES: u16 = 16;
const TILE_SIZE: u8 = 8;
const TILES_PER_MAP_ROW: u16 = 32;

const LOW_MAP: u16 = 0x1800;
const HIGH_MAP: u16 = 0x1C00;
const SIGNED_TILE_BASE: u16 = 0x1000;

const LCDC_BG_ENABLED: u8 = 1 << 0;
const LCDC_BG_MAP_HIGH: u8 = 1 << 3;
const LCDC_TILE_DATA_UNSIGNED: u8 = 1 << 4;

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
                self.render_scanline();
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
    pub fn render_scanline(&mut self) {
        let mut colors = [0u8; SCREEN_WIDTH];

        if self.lcdc & LCDC_BG_ENABLED != 0 {
            let y = self.ly.wrapping_add(self.scy);

            for (x, color) in colors.iter_mut().enumerate() {
                let x = (x as u8).wrapping_add(self.scx);
                *color = self.background_color(x, y);
            }
        }

        let line = self.ly as usize * SCREEN_WIDTH;
        for (x, &color) in colors.iter().enumerate() {
            self.framebuffer[line + x] = shade(self.bgp, color);
        }
    }

    fn background_color(&self, x: u8, y: u8) -> u8 {
        let map = if self.lcdc & LCDC_BG_MAP_HIGH == 0 {
            LOW_MAP
        } else {
            HIGH_MAP
        };

        let entry = (y / TILE_SIZE) as u16 * TILES_PER_MAP_ROW + (x / TILE_SIZE) as u16;
        let tile = self.vram[(map + entry) as usize];

        let row = tile_address(self.lcdc, tile) + (y % TILE_SIZE) as u16 * 2;
        let low_plane = self.vram[row as usize];
        let high_plane = self.vram[row as usize + 1];
        let column = 7 - x % TILE_SIZE;

        (high_plane >> column & 1) << 1 | (low_plane >> column & 1)
    }
}

fn tile_address(lcdc: u8, tile: u8) -> u16 {
    if lcdc & LCDC_TILE_DATA_UNSIGNED != 0 {
        tile as u16 * TILE_BYTES
    } else {
        SIGNED_TILE_BASE.wrapping_add_signed(tile as i8 as i16 * TILE_BYTES as i16)
    }
}

fn shade(palette: u8, color: u8) -> u8 {
    palette >> (color * 2) & 0x03
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

    const ON: u8 = 0x91;

    fn write_striped_tile(ppu: &mut Ppu, index: u8) {
        let base = index as u16 * TILE_BYTES;
        for row in 0..8 {
            ppu.write_vram(base + row * 2, 0b0101_0101);
            ppu.write_vram(base + row * 2 + 1, 0b0011_0011);
        }
    }

    fn identity_palette(ppu: &mut Ppu) {
        ppu.write_reg(addr::BGP, 0b11_10_01_00);
    }

    fn rendered_line(ppu: &mut Ppu, line: u8) -> Vec<u8> {
        ppu.ly = line;
        ppu.render_scanline();
        let start = line as usize * SCREEN_WIDTH;
        ppu.framebuffer[start..start + SCREEN_WIDTH].to_vec()
    }

    #[test]
    fn a_tile_maps_its_two_bitplanes_onto_four_colors() {
        let mut ppu = Ppu::new();
        identity_palette(&mut ppu);
        ppu.write_reg(addr::LCDC, ON);
        write_striped_tile(&mut ppu, 0);

        assert_eq!(&rendered_line(&mut ppu, 0)[..8], &[0, 1, 2, 3, 0, 1, 2, 3]);
    }

    #[test]
    fn the_palette_is_an_indirection_not_a_shade() {
        let mut ppu = Ppu::new();
        ppu.write_reg(addr::LCDC, ON);
        write_striped_tile(&mut ppu, 0);
        ppu.write_reg(addr::BGP, 0b00_01_10_11);

        assert_eq!(&rendered_line(&mut ppu, 0)[..8], &[3, 2, 1, 0, 3, 2, 1, 0]);
    }

    #[test]
    fn scx_scrolls_the_viewport() {
        let mut ppu = Ppu::new();
        identity_palette(&mut ppu);
        ppu.write_reg(addr::LCDC, ON);
        write_striped_tile(&mut ppu, 0);
        ppu.write_reg(addr::SCX, 2);

        assert_eq!(&rendered_line(&mut ppu, 0)[..6], &[2, 3, 0, 1, 2, 3]);
    }

    #[test]
    fn scy_selects_which_row_of_the_map_a_scanline_reads() {
        let mut ppu = Ppu::new();
        identity_palette(&mut ppu);
        ppu.write_reg(addr::LCDC, ON);
        write_striped_tile(&mut ppu, 1);
        ppu.write_vram(LOW_MAP + TILES_PER_MAP_ROW, 1);

        assert_eq!(rendered_line(&mut ppu, 0)[..4], [0, 0, 0, 0]);

        ppu.write_reg(addr::SCY, 8);
        assert_eq!(rendered_line(&mut ppu, 0)[..4], [0, 1, 2, 3]);
    }

    #[test]
    fn the_signed_addressing_mode_reaches_the_block_below_0x9000() {
        assert_eq!(tile_address(0x00, 0x00), 0x1000);
        assert_eq!(tile_address(0x00, 0x7F), 0x1000 + 0x7F * 16);
        assert_eq!(tile_address(0x00, 0xFF), 0x1000 - 16);
        assert_eq!(tile_address(0x00, 0x80), 0x0800);

        assert_eq!(tile_address(LCDC_TILE_DATA_UNSIGNED, 0x00), 0x0000);
        assert_eq!(tile_address(LCDC_TILE_DATA_UNSIGNED, 0xFF), 0xFF * 16);
    }

    #[test]
    fn the_high_map_is_a_different_1024_bytes() {
        let mut ppu = Ppu::new();
        identity_palette(&mut ppu);
        write_striped_tile(&mut ppu, 5);
        ppu.write_vram(HIGH_MAP, 5);

        ppu.write_reg(addr::LCDC, ON);
        assert_eq!(rendered_line(&mut ppu, 0)[..4], [0, 0, 0, 0]);

        ppu.write_reg(addr::LCDC, ON | LCDC_BG_MAP_HIGH);
        assert_eq!(rendered_line(&mut ppu, 0)[..4], [0, 1, 2, 3]);
    }

    #[test]
    fn clearing_lcdc_bit_0_blanks_the_background() {
        let mut ppu = Ppu::new();
        identity_palette(&mut ppu);
        ppu.write_reg(addr::LCDC, ON);
        write_striped_tile(&mut ppu, 0);
        assert_ne!(rendered_line(&mut ppu, 0)[1], 0);

        ppu.write_reg(addr::LCDC, ON & !LCDC_BG_ENABLED);
        assert_eq!(rendered_line(&mut ppu, 0), vec![0; SCREEN_WIDTH]);
    }

    #[test]
    fn a_whole_frame_of_scanlines_lands_in_the_right_rows() {
        let mut ppu = Ppu::new();
        identity_palette(&mut ppu);
        ppu.write_reg(addr::LCDC, ON);
        write_striped_tile(&mut ppu, 0);

        for line in 0..SCREEN_HEIGHT as u8 {
            rendered_line(&mut ppu, line);
        }
        for line in 0..SCREEN_HEIGHT {
            assert_eq!(ppu.framebuffer[line * SCREEN_WIDTH + 1], 1, "line {line}");
        }
    }
}
