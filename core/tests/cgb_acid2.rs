use ferroboy::Emulator;

// The framebuffer was compared pixel by pixel against the reference image in
// the cgb-acid2 repository; this hash freezes that verified render.
const REFERENCE_HASH: u64 = 0x78CE869D9B004A6F;

fn fnv1a(framebuffer: &[u16]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for &color in framebuffer {
        for byte in [color as u8, (color >> 8) as u8] {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash
}

#[test]
fn it_matches_the_reference_image() {
    let rom = std::fs::read("tests/roms/cgb_acid2.gbc").unwrap();
    let mut emulator = Emulator::new(&rom);
    for _ in 0..600 {
        emulator.run_frame();
    }

    assert_eq!(fnv1a(emulator.framebuffer()), REFERENCE_HASH);
}
