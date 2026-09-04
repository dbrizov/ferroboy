use super::*;
use crate::cartridge;

fn dmg_bus() -> Bus {
    Bus::new(cartridge::unplugged(), &[], false)
}

fn cgb_bus() -> Bus {
    Bus::new(cartridge::unplugged(), &[], true)
}

#[test]
fn svbk_switches_the_high_wram_half() {
    let mut bus = cgb_bus();
    bus.write(0xFF70, 2);
    bus.write(0xD000, 0xAA);
    bus.write(0xC000, 0x11);

    bus.write(0xFF70, 3);
    assert_ne!(bus.peek(0xD000), 0xAA);
    assert_eq!(bus.peek(0xC000), 0x11);

    bus.write(0xFF70, 2);
    assert_eq!(bus.peek(0xD000), 0xAA);
    assert_eq!(bus.peek(0xFF70), 0xFA);
}

#[test]
fn svbk_zero_selects_bank_one() {
    let mut bus = cgb_bus();
    bus.write(0xFF70, 0);
    bus.write(0xD000, 0x55);

    bus.write(0xFF70, 1);
    assert_eq!(bus.peek(0xD000), 0x55);
}

#[test]
fn vbk_switches_vram_banks() {
    let mut bus = cgb_bus();
    bus.write(0xFF4F, 1);
    bus.write(0x8000, 5);

    bus.write(0xFF4F, 0);
    assert_eq!(bus.peek(0x8000), 0);

    bus.write(0xFF4F, 1);
    assert_eq!(bus.peek(0x8000), 5);
    assert_eq!(bus.peek(0xFF4F), 0xFF);
}

#[test]
fn cgb_registers_do_not_exist_on_dmg() {
    let mut bus = dmg_bus();
    bus.write(0xFF70, 3);
    bus.write(0xFF4F, 1);
    bus.write(0xD000, 0x77);

    assert_eq!(bus.peek(0xFF70), 0xFF);
    assert_eq!(bus.peek(0xFF4D), 0xFF);
    assert_eq!(bus.peek(0xFF4F), 0xFF);
    assert_eq!(bus.peek(0xFF55), 0xFF);
    assert_eq!(bus.peek(0xD000), 0x77);
}

#[test]
fn stop_switches_speed_only_when_armed() {
    let mut bus = cgb_bus();
    bus.switch_speed();
    assert_eq!(bus.peek(0xFF4D), 0x7E);

    bus.write(0xFF4D, 1);
    assert_eq!(bus.peek(0xFF4D), 0x7F);

    bus.switch_speed();
    assert_eq!(bus.peek(0xFF4D), 0xFE);

    bus.write(0xFF4D, 1);
    bus.switch_speed();
    assert_eq!(bus.peek(0xFF4D), 0x7E);
}

#[test]
fn palette_ram_auto_increments() {
    let mut bus = cgb_bus();
    bus.write(0xFF68, 0x80);
    bus.write(0xFF69, 0x12);
    bus.write(0xFF69, 0x34);

    bus.write(0xFF68, 0x00);
    assert_eq!(bus.peek(0xFF69), 0x12);
    bus.write(0xFF68, 0x01);
    assert_eq!(bus.peek(0xFF69), 0x34);
    assert_eq!(bus.peek(0xFF68), 0x41);
}

#[test]
fn general_dma_copies_to_vram_at_once() {
    let mut bus = cgb_bus();
    for offset in 0..32 {
        bus.write(0xC000 + offset, offset as u8);
    }

    bus.write(0xFF51, 0xC0);
    bus.write(0xFF52, 0x00);
    bus.write(0xFF53, 0x04);
    bus.write(0xFF54, 0x00);
    bus.write(0xFF55, 0x01);

    for offset in 0..32 {
        assert_eq!(bus.peek(0x8400 + offset), offset as u8);
    }
    assert_eq!(bus.peek(0xFF55), 0xFF);
}

#[test]
fn hblank_dma_copies_one_block_per_hblank() {
    let mut bus = cgb_bus();
    for offset in 0..32 {
        bus.write(0xC000 + offset, 0xAB);
    }

    bus.write(0xFF51, 0xC0);
    bus.write(0xFF52, 0x00);
    bus.write(0xFF53, 0x00);
    bus.write(0xFF54, 0x00);
    bus.write(0xFF55, 0x81);
    assert_eq!(bus.peek(0xFF55), 0x01);
    assert_eq!(bus.peek(0x8000), 0x00);

    let mut budget = 500;
    while bus.peek(0x8000) != 0xAB && budget > 0 {
        bus.tick(4);
        budget -= 1;
    }
    assert!(budget > 0, "first block never copied");
    assert_eq!(bus.peek(0x800F), 0xAB);
    assert_eq!(bus.peek(0x8010), 0x00);
    assert_eq!(bus.peek(0xFF55), 0x00);

    let mut budget = 500;
    while bus.peek(0x8010) != 0xAB && budget > 0 {
        bus.tick(4);
        budget -= 1;
    }
    assert!(budget > 0, "second block never copied");
    assert_eq!(bus.peek(0xFF55), 0xFF);
}

#[test]
fn cancelling_hblank_dma_keeps_the_remaining_count() {
    let mut bus = cgb_bus();
    bus.write(0xFF51, 0xC0);
    bus.write(0xFF52, 0x00);
    bus.write(0xFF53, 0x00);
    bus.write(0xFF54, 0x00);
    bus.write(0xFF55, 0x83);

    bus.write(0xFF55, 0x00);
    assert_eq!(bus.peek(0xFF55), 0x83);
}
