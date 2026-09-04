use ferroboy::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};

const FRAMES: usize = 150; // Long enough for the scroll, the hold, and the handoff.

fn halt_rom() -> Vec<u8> {
    let mut rom = vec![0x76u8; 0x8000];
    rom[0x0147] = 0x00;
    rom[0x0148] = 0x00;
    rom[0x0149] = 0x00;

    let mut checksum = 0u8;
    for byte in &rom[0x0134..=0x014C] {
        checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    rom[0x014D] = checksum;

    rom
}

fn screen(emulator: &Emulator) -> String {
    let shades = [' ', '.', '+', '#'];

    (0..SCREEN_HEIGHT)
        .map(|row| {
            (0..SCREEN_WIDTH)
                .map(|x| shades[emulator.framebuffer()[row * SCREEN_WIDTH + x] as usize])
                .collect::<String>()
        })
        .filter(|line| !line.trim().is_empty())
        .map(|line| format!("{}\n", line.trim_end()))
        .collect()
}

#[test]
fn it_draws_the_logo_and_hands_the_machine_over() {
    let mut emulator = Emulator::new(&halt_rom());
    for _ in 0..FRAMES {
        emulator.run_frame();
    }

    println!("{}", screen(&emulator));

    assert!(!screen(&emulator).is_empty(), "something is on screen");
    assert_eq!(emulator.read(0x0000), 0x76, "the cartridge is mapped again");
    assert_eq!(emulator.read(0xFF47), 0xFC, "the palette it leaves behind");
}
