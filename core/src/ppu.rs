#[cfg(test)]
mod tests;

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
const TILES_PER_MAP_ROW: u16 = 32;
const TILE_SIZE: u8 = 8;

const LOW_MAP: u16 = 0x1800;
const HIGH_MAP: u16 = 0x1C00;

const LCDC_BG_ENABLED: u8 = 1 << 0;
const LCDC_OBJ_ENABLED: u8 = 1 << 1;
const LCDC_OBJ_TALL: u8 = 1 << 2;
const LCDC_BG_MAP_HIGH: u8 = 1 << 3;
const LCDC_TILE_DATA_LOW: u8 = 1 << 4;
const LCDC_WINDOW_ENABLED: u8 = 1 << 5;
const LCDC_WINDOW_MAP_HIGH: u8 = 1 << 6;

const OBJ_BEHIND_BACKGROUND: u8 = 1 << 7;
const OBJ_FLIP_Y: u8 = 1 << 6;
const OBJ_FLIP_X: u8 = 1 << 5;
const OBJ_PALETTE_1: u8 = 1 << 4;

const OBJECTS_PER_LINE: usize = 10;

const OBJ_X_BIAS: i16 = 8;
const OBJ_Y_BIAS: i16 = 16;

const WINDOW_X_BIAS: u8 = 7;

struct Object {
    index: u8,
    x: u8,
    tile: u8,
    attributes: u8,
    top: i16,
}

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
    window_line: u8,
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
            window_line: 0,
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
            self.window_line = 0;
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
        let mut background = [0u8; SCREEN_WIDTH];

        if self.lcdc & LCDC_BG_ENABLED != 0 {
            self.render_background(&mut background);
            self.render_window(&mut background);
        }

        let line = self.ly as usize * SCREEN_WIDTH;
        for (x, &color) in background.iter().enumerate() {
            self.framebuffer[line + x] = shade(self.bgp, color);
        }

        if self.lcdc & LCDC_OBJ_ENABLED != 0 {
            self.render_objects(&background);
        }
    }

    fn render_background(&self, background: &mut [u8; SCREEN_WIDTH]) {
        let map = if self.lcdc & LCDC_BG_MAP_HIGH == 0 {
            LOW_MAP
        } else {
            HIGH_MAP
        };
        let y = self.ly.wrapping_add(self.scy);

        for (x, color) in background.iter_mut().enumerate() {
            *color = self.tile_color(map, (x as u8).wrapping_add(self.scx), y);
        }
    }

    fn render_window(&mut self, background: &mut [u8; SCREEN_WIDTH]) {
        if self.lcdc & LCDC_WINDOW_ENABLED == 0 || self.ly < self.wy {
            return;
        }

        let map = if self.lcdc & LCDC_WINDOW_MAP_HIGH == 0 {
            LOW_MAP
        } else {
            HIGH_MAP
        };
        let start = self.wx.saturating_sub(WINDOW_X_BIAS) as usize;
        if start >= SCREEN_WIDTH {
            return;
        }

        for (x, color) in background.iter_mut().enumerate().skip(start) {
            let window_x = (x - start) as u8;
            *color = self.tile_color(map, window_x, self.window_line);
        }

        self.window_line = self.window_line.wrapping_add(1);
    }

    fn render_objects(&mut self, background: &[u8; SCREEN_WIDTH]) {
        let height = if self.lcdc & LCDC_OBJ_TALL != 0 {
            16
        } else {
            8
        };
        let line = self.ly as usize * SCREEN_WIDTH;

        let mut visible = self.objects_on_line(height);
        visible.sort_by_key(|object| std::cmp::Reverse((object.x, object.index)));

        for object in visible {
            for offset in 0..TILE_SIZE {
                let screen_x = object.x as i16 - OBJ_X_BIAS + offset as i16;
                if screen_x < 0 || screen_x >= SCREEN_WIDTH as i16 {
                    continue;
                }
                let screen_x = screen_x as usize;

                let color = self.object_color(&object, offset, height);
                if color == 0 {
                    continue;
                }
                if object.attributes & OBJ_BEHIND_BACKGROUND != 0 && background[screen_x] != 0 {
                    continue;
                }

                let palette = if object.attributes & OBJ_PALETTE_1 == 0 {
                    self.obp0
                } else {
                    self.obp1
                };
                self.framebuffer[line + screen_x] = shade(palette, color);
            }
        }
    }

    fn objects_on_line(&self, height: u8) -> Vec<Object> {
        let mut found = Vec::with_capacity(OBJECTS_PER_LINE);

        for index in 0..40u8 {
            let entry = index as usize * 4;
            let top = self.oam[entry] as i16 - OBJ_Y_BIAS;
            if (self.ly as i16) < top || (self.ly as i16) >= top + height as i16 {
                continue;
            }

            found.push(Object {
                index,
                x: self.oam[entry + 1],
                tile: self.oam[entry + 2],
                attributes: self.oam[entry + 3],
                top,
            });

            if found.len() == OBJECTS_PER_LINE {
                break;
            }
        }

        found
    }

    fn object_color(&self, object: &Object, offset: u8, height: u8) -> u8 {
        let mut row = (self.ly as i16 - object.top) as u8;
        if object.attributes & OBJ_FLIP_Y != 0 {
            row = height - 1 - row;
        }

        let tile = if height == 16 {
            object.tile & 0xFE
        } else {
            object.tile
        };

        let base = (tile as u16 * TILE_BYTES + row as u16 * 2) as usize;
        let low = self.vram[base];
        let high = self.vram[base + 1];
        let bit = if object.attributes & OBJ_FLIP_X != 0 {
            offset
        } else {
            7 - offset
        };

        (high >> bit & 1) << 1 | (low >> bit & 1)
    }

    fn tile_color(&self, map: u16, x: u8, y: u8) -> u8 {
        let entry = (y / TILE_SIZE) as u16 * TILES_PER_MAP_ROW + (x / TILE_SIZE) as u16;
        let tile = self.vram[(map + entry) as usize];
        let base = tile_address(self.lcdc, tile);

        let row = (y % TILE_SIZE) as u16 * 2;
        let low = self.vram[(base + row) as usize];
        let high = self.vram[(base + row + 1) as usize];
        let bit = 7 - (x % TILE_SIZE);

        (high >> bit & 1) << 1 | (low >> bit & 1)
    }
}

fn tile_address(lcdc: u8, tile: u8) -> u16 {
    if lcdc & LCDC_TILE_DATA_LOW != 0 {
        tile as u16 * TILE_BYTES
    } else {
        (0x1000 + (tile as i8 as i16 * TILE_BYTES as i16)) as u16
    }
}

fn shade(palette: u8, color: u8) -> u8 {
    palette >> (color * 2) & 0x03
}
