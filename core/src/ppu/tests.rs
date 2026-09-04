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
