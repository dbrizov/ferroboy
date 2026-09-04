use super::*;
use crate::cartridge::RAM_BANK_BYTES;
use crate::cartridge::tests::rom_of_banks;

fn mbc3(banks: usize, ram_size: usize) -> Mbc3 {
    let rom = rom_of_banks(banks);
    let header = Header {
        _title: String::new(),
        cart_type: 0x13,
        _rom_size: rom.len(),
        ram_size,
        has_battery: true,
    };
    Mbc3::new(&rom, &header)
}

#[test]
fn seven_bits_of_bank_with_no_unreachable_ones() {
    let mut cartridge = mbc3(128, 0);

    for bank in [0x20u8, 0x40, 0x60, 0x7F] {
        cartridge.write_rom(0x2000, bank);
        assert_eq!(cartridge.read_rom(0x4000), bank);
    }
}

#[test]
fn bank_0_still_selects_1() {
    let mut cartridge = mbc3(4, 0);
    cartridge.write_rom(0x2000, 0);

    assert_eq!(cartridge.read_rom(0x4000), 1);
}

#[test]
fn ram_banks_switch() {
    let mut cartridge = mbc3(4, RAM_BANK_BYTES * 4);
    cartridge.write_rom(0x0000, 0x0A);

    cartridge.write_rom(0x4000, 0);
    cartridge.write_ram(0, 0xAA);
    cartridge.write_rom(0x4000, 3);
    cartridge.write_ram(0, 0xBB);

    cartridge.write_rom(0x4000, 0);
    assert_eq!(cartridge.read_ram(0), 0xAA);
}

#[test]
fn selecting_an_rtc_register_does_not_alias_onto_ram() {
    let mut cartridge = mbc3(4, RAM_BANK_BYTES);
    cartridge.write_rom(0x0000, 0x0A);
    cartridge.write_ram(0, 0x42);

    cartridge.write_rom(0x4000, 0x08);
    assert_eq!(cartridge.read_ram(0), 0xFF, "a stopped clock, not RAM");
    cartridge.write_ram(0, 0x99);

    cartridge.write_rom(0x4000, 0x00);
    assert_eq!(cartridge.read_ram(0), 0x42, "RAM was not written through");
}
