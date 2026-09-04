use super::*;
use crate::cartridge::RAM_BANK_BYTES;
use crate::cartridge::tests::rom_of_banks;

fn mbc1(banks: usize, ram_size: usize) -> Mbc1 {
    let rom = rom_of_banks(banks);
    let header = Header {
        _title: String::new(),
        cart_type: 0x03,
        _rom_size: rom.len(),
        ram_size,
        has_battery: true,
    };
    Mbc1::new(&rom, &header)
}

#[test]
fn bank_1_is_mapped_at_reset() {
    let cartridge = mbc1(4, 0);

    assert_eq!(
        cartridge.read_rom(0x0000),
        0,
        "bank 0 is fixed at the bottom"
    );
    assert_eq!(cartridge.read_rom(0x4000), 1);
}

#[test]
fn writing_the_bank_register_moves_the_upper_window() {
    let mut cartridge = mbc1(8, 0);
    cartridge.write_rom(0x2000, 5);

    assert_eq!(cartridge.read_rom(0x4000), 5);
    assert_eq!(
        cartridge.read_rom(0x0000),
        0,
        "the lower half does not move"
    );
}

#[test]
fn bank_0_is_not_selectable() {
    let mut cartridge = mbc1(4, 0);
    cartridge.write_rom(0x2000, 0);

    assert_eq!(cartridge.read_rom(0x4000), 1, "0 selects 1");
}

#[test]
fn banks_20_40_and_60_are_unreachable() {
    let mut cartridge = mbc1(128, 0);

    for unreachable in [0x20u8, 0x40, 0x60] {
        cartridge.write_rom(0x2000, unreachable & 0x1F);
        cartridge.write_rom(0x4000, unreachable >> 5);

        assert_eq!(
            cartridge.read_rom(0x4000),
            unreachable + 1,
            "{unreachable:#04X} reads as the bank above it"
        );
    }
}

#[test]
fn ram_is_disabled_until_the_magic_value_is_written() {
    let mut cartridge = mbc1(4, RAM_BANK_BYTES);

    cartridge.write_ram(0, 0x42);
    assert_eq!(cartridge.read_ram(0), 0xFF, "disabled reads as open bus");

    cartridge.write_rom(0x0000, 0x0A);
    cartridge.write_ram(0, 0x42);
    assert_eq!(cartridge.read_ram(0), 0x42);

    cartridge.write_rom(0x0000, 0x00);
    assert_eq!(cartridge.read_ram(0), 0xFF, "and disabling hides it again");
}

#[test]
fn the_mode_bit_decides_what_the_upper_bits_mean() {
    let mut cartridge = mbc1(128, RAM_BANK_BYTES * 4);
    cartridge.write_rom(0x0000, 0x0A);
    cartridge.write_rom(0x4000, 2);

    cartridge.write_rom(0x2000, 1);
    assert_eq!(cartridge.read_rom(0x4000), 0x41);
    cartridge.write_ram(0, 0xAA);

    cartridge.write_rom(0x6000, 1);
    cartridge.write_ram(0, 0xBB);
    assert_eq!(cartridge.read_ram(0), 0xBB, "bank 2");

    cartridge.write_rom(0x6000, 0);
    assert_eq!(cartridge.read_ram(0), 0xAA, "back to bank 0");
}

#[test]
fn battery_ram_round_trips() {
    let mut cartridge = mbc1(4, RAM_BANK_BYTES);
    cartridge.write_rom(0x0000, 0x0A);
    cartridge.write_ram(0, 0x99);

    let saved = cartridge.battery_ram().unwrap().to_vec();

    let mut reloaded = mbc1(4, RAM_BANK_BYTES);
    reloaded.load_battery_ram(&saved);
    reloaded.write_rom(0x0000, 0x0A);

    assert_eq!(reloaded.read_ram(0), 0x99);
}
