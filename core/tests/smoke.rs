use ferroboy::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};

#[test]
fn a_frame_is_70224_t_cycles() {
    let mut emu = Emulator::new();
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
