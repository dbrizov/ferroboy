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
const OBJ_BANK: u8 = 1 << 3;
const OBJ_CGB_PALETTE: u8 = 0x07;

const BG_PRIORITY: u8 = 1 << 7;
const BG_FLIP_Y: u8 = 1 << 6;
const BG_FLIP_X: u8 = 1 << 5;
const BG_BANK: u8 = 1 << 3;
const BG_CGB_PALETTE: u8 = 0x07;

const VRAM_BANK_BYTES: usize = 0x2000;
const PALETTE_RAM_BYTES: usize = 64;

const OBJECTS_PER_LINE: usize = 10;

const OBJ_X_BIAS: i16 = 8;
const OBJ_Y_BIAS: i16 = 16;

const WINDOW_X_BIAS: u8 = 7;

#[derive(Clone, Copy, Default)]
struct BgPixel {
    color: u8,
    attributes: u8,
}

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
    pub const VBK: u16 = 0xFF4F;
    pub const BCPS: u16 = 0xFF68;
    pub const BCPD: u16 = 0xFF69;
    pub const OCPS: u16 = 0xFF6A;
    pub const OCPD: u16 = 0xFF6B;
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
    cgb: bool,
    vram: [u8; 2 * VRAM_BANK_BYTES],
    oam: [u8; 0xA0],
    mode: Mode,
    dots: u32,
    window_line: u8,
    frame_ready: bool,
    hblank_started: bool,
    framebuffer: [u16; SCREEN_WIDTH * SCREEN_HEIGHT],

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
    vbk: u8,
    bcps: u8,
    ocps: u8,
    bg_palette: [u8; PALETTE_RAM_BYTES],
    ob_palette: [u8; PALETTE_RAM_BYTES],
}

impl Ppu {
    pub fn new(cgb: bool) -> Self {
        Self {
            cgb,
            vram: [0; 2 * VRAM_BANK_BYTES],
            oam: [0; 0xA0],
            mode: Mode::OamScan,
            dots: 0,
            window_line: 0,
            frame_ready: false,
            hblank_started: false,
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
            vbk: 0,
            bcps: 0,
            ocps: 0,
            bg_palette: [0xFF; PALETTE_RAM_BYTES],
            ob_palette: [0xFF; PALETTE_RAM_BYTES],
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
                self.hblank_started = true;
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

    pub fn take_hblank_start(&mut self) -> bool {
        std::mem::take(&mut self.hblank_started)
    }

    pub fn framebuffer(&self) -> &[u16; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.framebuffer
    }

    pub fn read_vram(&self, offset: u16) -> u8 {
        self.vram[self.vbk as usize * VRAM_BANK_BYTES + offset as usize]
    }

    pub fn write_vram(&mut self, offset: u16, value: u8) {
        self.vram[self.vbk as usize * VRAM_BANK_BYTES + offset as usize] = value;
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
            addr::VBK => 0xFE | self.vbk,
            addr::BCPS => self.bcps | 0x40,
            addr::BCPD => self.bg_palette[(self.bcps & 0x3F) as usize],
            addr::OCPS => self.ocps | 0x40,
            addr::OCPD => self.ob_palette[(self.ocps & 0x3F) as usize],
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
            addr::VBK => self.vbk = value & 1,
            addr::BCPS => self.bcps = value & 0xBF,
            addr::BCPD => {
                self.bg_palette[(self.bcps & 0x3F) as usize] = value;
                if self.bcps & 0x80 != 0 {
                    self.bcps = 0x80 | (self.bcps + 1) & 0x3F;
                }
            }
            addr::OCPS => self.ocps = value & 0xBF,
            addr::OCPD => {
                self.ob_palette[(self.ocps & 0x3F) as usize] = value;
                if self.ocps & 0x80 != 0 {
                    self.ocps = 0x80 | (self.ocps + 1) & 0x3F;
                }
            }
            _ => {}
        }
    }

    pub fn render_scanline(&mut self) {
        let mut background = [BgPixel::default(); SCREEN_WIDTH];

        // On the CGB, LCDC bit 0 no longer blanks the background - it only
        // demotes its priority against sprites.
        if self.cgb || self.lcdc & LCDC_BG_ENABLED != 0 {
            self.render_background(&mut background);
            self.render_window(&mut background);
        }

        let line = self.ly as usize * SCREEN_WIDTH;
        for (x, pixel) in background.iter().enumerate() {
            self.framebuffer[line + x] = if self.cgb {
                color_555(
                    &self.bg_palette,
                    pixel.attributes & BG_CGB_PALETTE,
                    pixel.color,
                )
            } else {
                shade(self.bgp, pixel.color) as u16
            };
        }

        if self.lcdc & LCDC_OBJ_ENABLED != 0 {
            self.render_objects(&background);
        }
    }

    fn render_background(&self, background: &mut [BgPixel; SCREEN_WIDTH]) {
        let map = if self.lcdc & LCDC_BG_MAP_HIGH == 0 {
            LOW_MAP
        } else {
            HIGH_MAP
        };
        let y = self.ly.wrapping_add(self.scy);

        for (x, pixel) in background.iter_mut().enumerate() {
            *pixel = self.tile_pixel(map, (x as u8).wrapping_add(self.scx), y);
        }
    }

    fn render_window(&mut self, background: &mut [BgPixel; SCREEN_WIDTH]) {
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

        for (x, pixel) in background.iter_mut().enumerate().skip(start) {
            let window_x = (x - start) as u8;
            *pixel = self.tile_pixel(map, window_x, self.window_line);
        }

        self.window_line = self.window_line.wrapping_add(1);
    }

    fn render_objects(&mut self, background: &[BgPixel; SCREEN_WIDTH]) {
        let height = if self.lcdc & LCDC_OBJ_TALL != 0 {
            16
        } else {
            8
        };
        let line = self.ly as usize * SCREEN_WIDTH;

        let mut visible = self.objects_on_line(height);
        // The CGB ranks sprites by OAM index alone; the DMG ranks by X first.
        if self.cgb {
            visible.sort_by_key(|object| std::cmp::Reverse(object.index));
        } else {
            visible.sort_by_key(|object| std::cmp::Reverse((object.x, object.index)));
        }

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
                if self.background_wins(&object, background[screen_x]) {
                    continue;
                }

                self.framebuffer[line + screen_x] = if self.cgb {
                    color_555(&self.ob_palette, object.attributes & OBJ_CGB_PALETTE, color)
                } else {
                    let palette = if object.attributes & OBJ_PALETTE_1 == 0 {
                        self.obp0
                    } else {
                        self.obp1
                    };
                    shade(palette, color) as u16
                };
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

    fn background_wins(&self, object: &Object, background: BgPixel) -> bool {
        if background.color == 0 {
            return false;
        }
        if self.cgb {
            // LCDC bit 0 is the master switch: cleared, sprites always win.
            self.lcdc & LCDC_BG_ENABLED != 0
                && (background.attributes & BG_PRIORITY != 0
                    || object.attributes & OBJ_BEHIND_BACKGROUND != 0)
        } else {
            object.attributes & OBJ_BEHIND_BACKGROUND != 0
        }
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

        let bank = if self.cgb && object.attributes & OBJ_BANK != 0 {
            VRAM_BANK_BYTES
        } else {
            0
        };
        let base = (tile as u16 * TILE_BYTES + row as u16 * 2) as usize + bank;
        let low = self.vram[base];
        let high = self.vram[base + 1];
        let bit = if object.attributes & OBJ_FLIP_X != 0 {
            offset
        } else {
            7 - offset
        };

        (high >> bit & 1) << 1 | (low >> bit & 1)
    }

    fn tile_pixel(&self, map: u16, x: u8, y: u8) -> BgPixel {
        let entry =
            ((y / TILE_SIZE) as u16 * TILES_PER_MAP_ROW + (x / TILE_SIZE) as u16 + map) as usize;
        let tile = self.vram[entry];
        // The attribute plane shadows the tile map from VRAM bank 1.
        let attributes = if self.cgb {
            self.vram[entry + VRAM_BANK_BYTES]
        } else {
            0
        };

        let mut row = y % TILE_SIZE;
        if attributes & BG_FLIP_Y != 0 {
            row = TILE_SIZE - 1 - row;
        }
        let mut bit = 7 - (x % TILE_SIZE);
        if attributes & BG_FLIP_X != 0 {
            bit = 7 - bit;
        }

        let bank = if attributes & BG_BANK != 0 {
            VRAM_BANK_BYTES
        } else {
            0
        };
        let base = tile_address(self.lcdc, tile) as usize + row as usize * 2 + bank;
        let low = self.vram[base];
        let high = self.vram[base + 1];

        BgPixel {
            color: (high >> bit & 1) << 1 | (low >> bit & 1),
            attributes,
        }
    }
}

fn color_555(palette_ram: &[u8; PALETTE_RAM_BYTES], palette: u8, color: u8) -> u16 {
    let index = palette as usize * 8 + color as usize * 2;
    ((palette_ram[index + 1] as u16) << 8 | palette_ram[index] as u16) & 0x7FFF
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
