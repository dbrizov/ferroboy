use ferroboy::Emulator;

fn cgb_halt_rom() -> Vec<u8> {
    let mut rom = vec![0x76u8; 0x8000];
    for byte in &mut rom[0x0134..0x014D] {
        *byte = 0;
    }
    rom[0x0143] = 0xC0;

    let mut checksum = 0u8;
    for byte in &rom[0x0134..0x014D] {
        checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    rom[0x014D] = checksum;
    rom
}

#[test]
fn it_boots_in_color() {
    let mut emulator = Emulator::new(&cgb_halt_rom());
    assert!(emulator.is_cgb());

    for _ in 0..300 {
        emulator.run_frame();
    }

    let framebuffer = emulator.framebuffer();
    let white = framebuffer.iter().filter(|&&c| c == 0x7FFF).count();
    let black = framebuffer.iter().filter(|&&c| c == 0x0000).count();
    println!("white={white} black={black}");
    assert!(white > 15000, "background should be RGB555 white");
    assert!(black > 300, "the logo should be RGB555 black");
}
