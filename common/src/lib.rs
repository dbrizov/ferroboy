mod audio;

use std::time::Duration;

use ferroboy::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};

pub use crate::audio::Audio;

pub const GB_WIDTH: u32 = SCREEN_WIDTH as u32;
pub const GB_HEIGHT: u32 = SCREEN_HEIGHT as u32;
pub const WINDOW_SCALE: u32 = 4;

// 70,224 T-cycles / 4,194,304 Hz. The frontend paces by wall clock against this.
pub const FRAME_DURATION: Duration = Duration::from_nanos(16_742_706);

/// The DMG's four shades, lightest to darkest, as RGBA.
pub const PALETTE: [[u8; 4]; 4] = [
    [0x9B, 0xBC, 0x0F, 0xFF],
    [0x8B, 0xAC, 0x0F, 0xFF],
    [0x30, 0x62, 0x30, 0xFF],
    [0x0F, 0x38, 0x0F, 0xFF],
];

pub fn to_rgba(emulator: &Emulator, surface: &mut [[u8; 4]]) {
    let framebuffer = emulator.framebuffer();
    if emulator.is_cgb() {
        for (pixel, &color) in surface.iter_mut().zip(framebuffer) {
            *pixel = rgba_from_555(color);
        }
    } else {
        for (pixel, &shade) in surface.iter_mut().zip(framebuffer) {
            *pixel = PALETTE[shade as usize];
        }
    }
}

fn rgba_from_555(color: u16) -> [u8; 4] {
    let expand = |channel: u16| (channel << 3 | channel >> 2) as u8;
    [
        expand(color & 0x1F),
        expand(color >> 5 & 0x1F),
        expand(color >> 10 & 0x1F),
        0xFF,
    ]
}

pub fn looks_like_a_rom(rom: &[u8]) -> bool {
    if rom.len() < 0x0150 {
        return false;
    }

    let mut checksum = 0u8;
    for byte in &rom[0x0134..0x014D] {
        checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    checksum == rom[0x014D]
}
