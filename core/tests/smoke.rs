use ferroboy::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};

const TITLE: usize = 0x0134;
const CART_TYPE: usize = 0x0147;
const ROM_SIZE: usize = 0x0148;
const RAM_SIZE: usize = 0x0149;
const HEADER_CHECKSUM: usize = 0x014D;

const CART_TYPE_NONE: u8 = 0x00;
const ROM_SIZE_32_KIB: u8 = 0x00;
const RAM_SIZE_NONE: u8 = 0x00;

const HALT: u8 = 0x76;

/// 32 KiB of HALT behind a valid header. The CPU parks on its first fetch, so
/// every step costs 4 T-cycles and the frame length is exactly predictable.
fn halt_rom() -> Vec<u8> {
    let mut rom = vec![HALT; 0x8000];
    rom[CART_TYPE] = CART_TYPE_NONE;
    rom[ROM_SIZE] = ROM_SIZE_32_KIB;
    rom[RAM_SIZE] = RAM_SIZE_NONE;

    let mut checksum = 0u8;
    for byte in &rom[TITLE..HEADER_CHECKSUM] {
        checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    rom[HEADER_CHECKSUM] = checksum;

    rom
}

#[test]
#[should_panic(expected = "header checksum mismatch")]
fn a_rom_that_is_not_a_rom_is_rejected() {
    let mut rom = halt_rom();
    rom[HEADER_CHECKSUM] ^= 0xFF;

    Emulator::new(&rom);
}

#[test]
fn a_frame_is_70224_t_cycles() {
    let mut emu = Emulator::new(&halt_rom());
    let mut cycles = 0u32;
    for _ in 0..100_000 {
        cycles += emu.step() as u32;
        if cycles >= 70_224 {
            break;
        }
    }

    assert_eq!(cycles, 70_224);
    assert_eq!(emu.framebuffer().len(), SCREEN_WIDTH * SCREEN_HEIGHT);
}
