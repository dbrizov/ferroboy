use ferroboy::Emulator;

#[test]
fn writing_ff46_copies_160_bytes_into_oam() {
    let mut rom = vec![0x76u8; 0x8000];
    let program = [
        0x21, 0x00, 0xC1, 0x3E, 0x5A, 0x06, 0xA0, 0x22, 0x05, 0x20, 0xFC, 0x3E, 0xC1, 0xE0, 0x46,
        0x76,
    ];
    rom[0x0100..0x0100 + program.len()].copy_from_slice(&program);

    rom[0x0147] = 0x00;
    rom[0x0148] = 0x00;
    rom[0x0149] = 0x00;

    let mut checksum = 0u8;
    for byte in &rom[0x0134..=0x014C] {
        checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    rom[0x014D] = checksum;

    let mut emulator = Emulator::new(&rom);

    for _ in 0..150 {
        emulator.run_frame();
    }

    assert_eq!(emulator.read(0xFE00), 0x5A, "first OAM byte");
    assert_eq!(emulator.read(0xFE9F), 0x5A, "last OAM byte");
}
