use super::*;
use crate::cartridge::RAM_BANK_BYTES;
use crate::cartridge::tests::rom_of_banks;

fn mbc5(banks: usize, ram_size: usize) -> Mbc5 {
    let rom = rom_of_banks(banks);
    let header = Header {
        _title: String::new(),
        cart_type: 0x1B,
        _rom_size: rom.len(),
        ram_size,
        has_battery: true,
    };
    Mbc5::new(&rom, &header)
}

#[test]
fn bank_0_is_selectable_unlike_mbc1() {
    let mut cartridge = mbc5(4, 0);
    cartridge.write_rom(0x2000, 0);

    assert_eq!(cartridge.read_rom(0x4000), 0, "0 means 0 here");
}

#[test]
fn the_bank_register_is_nine_bits_across_two_ranges() {
    let mut cartridge = mbc5(512, 0);

    cartridge.write_rom(0x2000, 0x11);
    assert_eq!(cartridge.read_rom(0x4000), 0x11);

    cartridge.write_rom(0x3000, 0x01);
    assert_eq!(cartridge.read_rom(0x4000), 0x11, "bank 0x111 wraps to 0x11");

    cartridge.write_rom(0x2000, 0x05);
    assert_eq!(cartridge.read_rom(0x4000), 0x05, "bank 0x105 wraps to 0x05");
}

#[test]
fn ram_banks_are_four_bits() {
    let mut cartridge = mbc5(4, RAM_BANK_BYTES * 4);
    cartridge.write_rom(0x0000, 0x0A);

    for bank in 0..4u8 {
        cartridge.write_rom(0x4000, bank);
        cartridge.write_ram(0, 0xB0 | bank);
    }
    for bank in 0..4u8 {
        cartridge.write_rom(0x4000, bank);
        assert_eq!(cartridge.read_ram(0), 0xB0 | bank);
    }
}
