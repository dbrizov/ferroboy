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
    let mut ppu = Ppu::new(false);
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
    let mut ppu = Ppu::new(false);
    run(&mut ppu, 70_224);

    assert!(ppu.take_frame_ready());
    assert_eq!(ppu.ly, 0, "wrapped back to the top");
    assert_eq!(ppu.mode, Mode::OamScan);
    assert!(!ppu.take_frame_ready(), "taking it clears it");
}

#[test]
fn vblank_starts_at_line_144_and_lasts_ten_lines() {
    let mut ppu = Ppu::new(false);
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
    let mut ppu = Ppu::new(false);
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
    let mut ppu = Ppu::new(false);
    ppu.write_reg(addr::LYC, 2);

    let raised = run(&mut ppu, 2 * SCANLINE_DOTS);
    assert_eq!(ppu.ly, 2);
    assert!(ppu.read_reg(addr::STAT) & stat::COINCIDENCE != 0);
    assert_eq!(raised & interrupts::STAT, 0, "source not enabled");

    // Now enable it and come round again.
    let mut ppu = Ppu::new(false);
    ppu.write_reg(addr::LYC, 2);
    ppu.write_reg(addr::STAT, stat::LYC_EQUALS_LY);

    let raised = run(&mut ppu, 2 * SCANLINE_DOTS);
    assert!(raised & interrupts::STAT != 0);
}

#[test]
fn stat_reports_the_mode_and_refuses_to_have_it_written() {
    let mut ppu = Ppu::new(false);
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
    let mut ppu = Ppu::new(false);
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

fn write_solid_tile(ppu: &mut Ppu, index: u8) {
    let base = index as u16 * TILE_BYTES;
    for byte in 0..TILE_BYTES {
        ppu.write_vram(base + byte, 0xFF);
    }
}

fn identity_palette(ppu: &mut Ppu) {
    ppu.write_reg(addr::BGP, 0b11_10_01_00);
    ppu.write_reg(addr::OBP0, 0b11_10_01_00);
}

fn put_object(ppu: &mut Ppu, index: u8, y: u8, x: u8, tile: u8, attributes: u8) {
    let entry = index as u16 * 4;
    ppu.write_oam(entry, y);
    ppu.write_oam(entry + 1, x);
    ppu.write_oam(entry + 2, tile);
    ppu.write_oam(entry + 3, attributes);
}

fn rendered_line(ppu: &mut Ppu, line: u8) -> Vec<u16> {
    ppu.ly = line;
    ppu.render_scanline();
    let start = line as usize * SCREEN_WIDTH;
    ppu.framebuffer[start..start + SCREEN_WIDTH].to_vec()
}

#[test]
fn a_tile_maps_its_two_bitplanes_onto_four_colors() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON);
    write_striped_tile(&mut ppu, 0);

    assert_eq!(&rendered_line(&mut ppu, 0)[..8], &[0, 1, 2, 3, 0, 1, 2, 3]);
}

#[test]
fn the_palette_is_an_indirection_not_a_shade() {
    let mut ppu = Ppu::new(false);
    ppu.write_reg(addr::LCDC, ON);
    write_striped_tile(&mut ppu, 0);

    ppu.write_reg(addr::BGP, 0b00_01_10_11);
    assert_eq!(&rendered_line(&mut ppu, 0)[..8], &[3, 2, 1, 0, 3, 2, 1, 0]);
}

#[test]
fn scx_scrolls_the_viewport() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON);
    write_striped_tile(&mut ppu, 0);

    ppu.write_reg(addr::SCX, 2);
    assert_eq!(&rendered_line(&mut ppu, 0)[..6], &[2, 3, 0, 1, 2, 3]);
}

#[test]
fn scy_selects_which_row_of_the_map_a_scanline_reads() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON);
    write_striped_tile(&mut ppu, 1);
    ppu.write_vram(LOW_MAP + TILES_PER_MAP_ROW, 1);

    assert_eq!(rendered_line(&mut ppu, 0)[..4], [0, 0, 0, 0]);

    ppu.write_reg(addr::SCY, 8);
    assert_eq!(rendered_line(&mut ppu, 0)[..4], [0, 1, 2, 3]);
}

#[test]
fn the_signed_addressing_mode_reaches_the_other_block() {
    assert_eq!(tile_address(0x00, 0x00), 0x1000);
    assert_eq!(tile_address(0x00, 0x7F), 0x1000 + 0x7F * 16);
    assert_eq!(tile_address(0x00, 0xFF), 0x1000 - 16);
    assert_eq!(tile_address(0x00, 0x80), 0x0800);

    assert_eq!(tile_address(LCDC_TILE_DATA_LOW, 0x00), 0x0000);
    assert_eq!(tile_address(LCDC_TILE_DATA_LOW, 0xFF), 0xFF * 16);
}

#[test]
fn the_high_map_is_a_different_1024_bytes() {
    let mut ppu = Ppu::new(false);
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
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON);
    write_striped_tile(&mut ppu, 0);
    assert_ne!(rendered_line(&mut ppu, 0)[1], 0);

    ppu.write_reg(addr::LCDC, ON & !LCDC_BG_ENABLED);
    assert_eq!(rendered_line(&mut ppu, 0), vec![0; SCREEN_WIDTH]);
}

#[test]
fn a_whole_frame_of_scanlines_lands_in_the_right_rows() {
    let mut ppu = Ppu::new(false);
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

#[test]
fn the_window_covers_the_background_from_wx_onward() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_WINDOW_ENABLED | LCDC_WINDOW_MAP_HIGH);

    write_solid_tile(&mut ppu, 1);
    ppu.write_vram(HIGH_MAP, 1);

    ppu.write_reg(addr::WY, 0);
    ppu.write_reg(addr::WX, WINDOW_X_BIAS + 40);

    let line = rendered_line(&mut ppu, 0);
    assert_eq!(line[39], 0, "still background here");
    assert_eq!(line[40], 3, "the window starts at WX - 7");
    assert_eq!(line[47], 3, "and covers its whole first tile");
}

#[test]
fn the_window_does_not_start_before_wy() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_WINDOW_ENABLED);
    write_solid_tile(&mut ppu, 7);
    ppu.write_vram(LOW_MAP, 7);
    ppu.write_reg(addr::WY, 5);
    ppu.write_reg(addr::WX, WINDOW_X_BIAS);

    rendered_line(&mut ppu, 4);
    assert_eq!(ppu.window_line, 0, "not yet");

    rendered_line(&mut ppu, 5);
    assert_eq!(ppu.window_line, 1, "the counter moves only when drawn");
}

#[test]
fn a_sprite_is_drawn_at_its_biased_coordinates() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_OBJ_ENABLED);
    write_solid_tile(&mut ppu, 2);
    put_object(&mut ppu, 0, 16, 8, 2, 0);

    let line = rendered_line(&mut ppu, 0);
    assert_eq!(&line[..8], &[3; 8]);
    assert_eq!(line[8], 0, "eight pixels wide");
}

#[test]
fn color_zero_is_transparent() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_OBJ_ENABLED);
    write_striped_tile(&mut ppu, 3);
    put_object(&mut ppu, 0, 16, 8, 3, 0);

    let line = rendered_line(&mut ppu, 0);
    assert_eq!(&line[..4], &[0, 1, 2, 3], "color 0 shows the background");
}

#[test]
fn the_priority_bit_puts_a_sprite_behind_non_zero_background() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_OBJ_ENABLED);
    write_solid_tile(&mut ppu, 4);
    write_striped_tile(&mut ppu, 0);
    put_object(&mut ppu, 0, 16, 8, 4, OBJ_BEHIND_BACKGROUND);

    let line = rendered_line(&mut ppu, 0);
    assert_eq!(line[0], 3, "background color 0, so the sprite wins");
    assert_eq!(line[1], 1, "background color 1, so the sprite loses");
    assert_eq!(line[2], 2);
}

#[test]
fn flipping_reverses_the_tile() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_OBJ_ENABLED);
    write_striped_tile(&mut ppu, 3);

    put_object(&mut ppu, 0, 16, 8, 3, 0);
    assert_eq!(&rendered_line(&mut ppu, 0)[..4], &[0, 1, 2, 3]);

    put_object(&mut ppu, 0, 16, 8, 3, OBJ_FLIP_X);
    assert_eq!(&rendered_line(&mut ppu, 0)[..4], &[3, 2, 1, 0]);
}

#[test]
fn the_lower_x_wins_and_ties_go_to_oam_order() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_OBJ_ENABLED);
    write_solid_tile(&mut ppu, 1);
    write_striped_tile(&mut ppu, 2);

    put_object(&mut ppu, 0, 16, 9, 2, 0);
    put_object(&mut ppu, 1, 16, 8, 1, 0);
    assert_eq!(
        rendered_line(&mut ppu, 0)[1],
        3,
        "the solid one at x=8 wins"
    );

    put_object(&mut ppu, 0, 16, 8, 2, 0);
    put_object(&mut ppu, 1, 16, 8, 1, 0);
    assert_eq!(rendered_line(&mut ppu, 0)[1], 1, "index 0 wins the tie");
}

#[test]
fn only_ten_sprites_are_drawn_on_a_line() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_OBJ_ENABLED);
    write_solid_tile(&mut ppu, 1);

    for index in 0..11u8 {
        put_object(&mut ppu, index, 16, 8 + index * 8, 1, 0);
    }

    let line = rendered_line(&mut ppu, 0);
    assert_eq!(line[79], 3, "the tenth sprite is drawn");
    assert_eq!(line[80], 0, "the eleventh is not");
}

#[test]
fn a_tall_sprite_is_two_stacked_tiles_and_ignores_bit_0() {
    let mut ppu = Ppu::new(false);
    identity_palette(&mut ppu);
    ppu.write_reg(addr::LCDC, ON | LCDC_OBJ_ENABLED | LCDC_OBJ_TALL);
    write_solid_tile(&mut ppu, 4);
    write_striped_tile(&mut ppu, 5);

    put_object(&mut ppu, 0, 16, 8, 5, 0);

    assert_eq!(
        rendered_line(&mut ppu, 0)[..4],
        [3, 3, 3, 3],
        "tile 4 on top"
    );
    assert_eq!(
        rendered_line(&mut ppu, 8)[..4],
        [0, 1, 2, 3],
        "tile 5 below"
    );
}

#[test]
fn cgb_tiles_use_their_attribute_palette() {
    let mut ppu = Ppu::new(true);
    ppu.write_reg(addr::LCDC, ON);
    write_solid_tile(&mut ppu, 1);
    ppu.write_vram(LOW_MAP, 1);

    ppu.write_reg(addr::VBK, 1);
    ppu.write_vram(LOW_MAP, 0x01);
    ppu.write_reg(addr::VBK, 0);

    ppu.write_reg(addr::BCPS, 0x80 | 14);
    ppu.write_reg(addr::BCPD, 0x34);
    ppu.write_reg(addr::BCPD, 0x12);

    assert_eq!(rendered_line(&mut ppu, 0)[0], 0x1234);
}

#[test]
fn cgb_tiles_can_flip_and_fetch_from_bank_one() {
    let mut ppu = Ppu::new(true);
    ppu.write_reg(addr::LCDC, ON);

    ppu.write_reg(addr::VBK, 1);
    for row in 0..8u16 {
        ppu.write_vram(TILE_BYTES + row * 2, 0x80);
        ppu.write_vram(TILE_BYTES + row * 2 + 1, 0x80);
    }
    ppu.write_vram(LOW_MAP, BG_FLIP_X | BG_BANK);
    ppu.write_reg(addr::VBK, 0);
    ppu.write_vram(LOW_MAP, 1);

    ppu.write_reg(addr::BCPS, 0x80 | 6);
    ppu.write_reg(addr::BCPD, 0xCD);
    ppu.write_reg(addr::BCPD, 0x2A);

    let line = rendered_line(&mut ppu, 0);
    assert_eq!(line[7], 0x2ACD, "the leftmost pixel should flip to x=7");
    assert_ne!(line[0], 0x2ACD);
}

#[test]
fn cgb_sprites_rank_by_oam_index_not_x() {
    let mut ppu = Ppu::new(true);
    ppu.write_reg(addr::LCDC, ON | LCDC_OBJ_ENABLED);
    write_solid_tile(&mut ppu, 1);
    put_object(&mut ppu, 0, 16, 12, 1, 0x00);
    put_object(&mut ppu, 1, 16, 8, 1, 0x01);

    ppu.write_reg(addr::OCPS, 0x80 | 6);
    ppu.write_reg(addr::OCPD, 0x11);
    ppu.write_reg(addr::OCPD, 0x11);
    ppu.write_reg(addr::OCPS, 0x80 | 14);
    ppu.write_reg(addr::OCPD, 0x22);
    ppu.write_reg(addr::OCPD, 0x22);

    let line = rendered_line(&mut ppu, 0);
    assert_eq!(line[4], 0x1111, "index 0 wins the overlap despite higher x");
    assert_eq!(line[0], 0x2222);
}
