use super::*;

const HALT: u8 = 0x76;
const CGB_ENHANCED: u8 = 0x80;

const CART_TYPE_NONE: u8 = 0x00;
const CART_TYPE_MBC1: u8 = 0x01;

const ROM_SIZE_32_KIB: u8 = 0x00;
const ROM_SIZE_64_KIB: u8 = 0x01;
const ROM_SIZE_1_MIB: u8 = 0x05;

const RAM_SIZE_NONE: u8 = 0x00;
const RAM_SIZE_64_KIB: u8 = 0x05;

fn rom_with(cart_type: u8, rom_size: u8, ram_size: u8) -> Vec<u8> {
    let mut rom = vec![HALT; 0x8000];
    rom[addr::TITLE..=addr::CGB_FLAG].fill(0);
    rom[addr::CART_TYPE] = cart_type;
    rom[addr::ROM_SIZE] = rom_size;
    rom[addr::RAM_SIZE] = ram_size;
    rom[addr::CHECKSUM] = header_checksum(&rom);
    rom
}

#[test]
fn parses_a_32_kib_cartridge_with_no_ram() {
    let header = Header::parse(&rom_with(CART_TYPE_NONE, ROM_SIZE_32_KIB, RAM_SIZE_NONE));
    assert_eq!(header._title, "");
    assert_eq!(header.cart_type, CART_TYPE_NONE);
    assert_eq!(header.rom_size, 0x8000);
    assert_eq!(header.ram_size, 0);
}

#[test]
fn the_size_codes_are_not_the_sizes() {
    let header = Header::parse(&rom_with(CART_TYPE_MBC1, ROM_SIZE_1_MIB, RAM_SIZE_64_KIB));
    assert_eq!(header.rom_size, 1024 * 1024);
    assert_eq!(header.ram_size, 64 * 1024);
}

#[test]
fn the_title_stops_at_the_first_zero() {
    let mut rom = rom_with(CART_TYPE_NONE, ROM_SIZE_32_KIB, RAM_SIZE_NONE);
    rom[addr::TITLE..addr::TITLE + 6].copy_from_slice(b"TETRIS");
    rom[addr::CHECKSUM] = header_checksum(&rom);

    assert_eq!(Header::parse(&rom)._title, "TETRIS");
}

#[test]
fn a_cgb_flag_is_not_part_of_the_title() {
    let mut rom = rom_with(CART_TYPE_NONE, ROM_SIZE_32_KIB, RAM_SIZE_NONE);
    rom[addr::TITLE..=addr::CGB_FLAG].fill(b'A');
    rom[addr::CGB_FLAG] = CGB_ENHANCED;
    rom[addr::CHECKSUM] = header_checksum(&rom);

    assert_eq!(Header::parse(&rom)._title, "A".repeat(15));
}

#[test]
fn load_takes_a_32_kib_mbc1_cartridge() {
    // Banks 0 and 1 only, and bank 1 is selected at reset, so it behaves
    // exactly like no mapper. Every Blargg test ROM is one of these.
    let rom = rom_with(CART_TYPE_MBC1, ROM_SIZE_32_KIB, RAM_SIZE_NONE);

    assert_eq!(load(&rom).read_rom(addr::CART_TYPE as u16), CART_TYPE_MBC1);
}

#[test]
#[should_panic(expected = "unsupported cartridge type 0x01")]
fn load_refuses_an_mbc1_cartridge_that_needs_banking() {
    let mut rom = rom_with(CART_TYPE_MBC1, ROM_SIZE_64_KIB, RAM_SIZE_NONE);
    rom.resize(0x10000, HALT);

    load(&rom);
}

#[test]
#[should_panic(expected = "header checksum mismatch")]
fn a_wrong_checksum_is_rejected() {
    let mut rom = rom_with(CART_TYPE_NONE, ROM_SIZE_32_KIB, RAM_SIZE_NONE);
    rom[addr::CHECKSUM] ^= 0xFF;

    Header::parse(&rom);
}
