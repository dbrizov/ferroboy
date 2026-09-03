mod nombc;

use crate::cartridge::nombc::NoMbc;

pub trait Cartridge {
    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self, offset: u16) -> u8;
    fn write_ram(&mut self, offset: u16, value: u8);
}

pub fn load(rom: &[u8]) -> Box<dyn Cartridge> {
    let header = Header::parse(rom);
    match header.cart_type {
        0x00 => Box::new(NoMbc::new(rom, &header)),
        // A 32 KiB MBC1 cartridge has only banks 0 and 1, and bank 1 is what
        // the mapper selects at reset - so with no bank switching it behaves
        // exactly like no mapper at all. Every Blargg test ROM is one of these.
        0x01..=0x03 if header.rom_size == 0x8000 => Box::new(NoMbc::new(rom, &header)),
        other => panic!("unsupported cartridge type {other:#04X} - M9 adds MBC1, MBC3 and MBC5"),
    }
}

mod addr {
    pub const TITLE: usize = 0x0134;
    pub const CGB_FLAG: usize = 0x0143;
    pub const CART_TYPE: usize = 0x0147;
    pub const ROM_SIZE: usize = 0x0148;
    pub const RAM_SIZE: usize = 0x0149;
    pub const CHECKSUM: usize = 0x014D;
}

pub struct Header {
    pub title: String,
    pub cart_type: u8,
    pub rom_size: usize,
    pub ram_size: usize,
}

impl Header {
    pub fn parse(rom: &[u8]) -> Self {
        assert!(
            rom.len() > addr::CHECKSUM,
            "ROM is {} bytes; the header alone runs to 0x014F",
            rom.len()
        );
        assert_eq!(
            rom[addr::CHECKSUM],
            header_checksum(rom),
            "header checksum mismatch - this is probably not a Game Boy ROM"
        );

        Self {
            title: title(rom),
            cart_type: rom[addr::CART_TYPE],
            rom_size: rom_size(rom[addr::ROM_SIZE]),
            ram_size: ram_size(rom[addr::RAM_SIZE]),
        }
    }
}

/// Subtract every byte of 0x0134..=0x014C, and one more each time. The boot ROM
/// compares this against 0x014D and refuses to start the cartridge if it differs.
fn header_checksum(rom: &[u8]) -> u8 {
    let mut checksum = 0u8;
    for byte in &rom[addr::TITLE..addr::CHECKSUM] {
        checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    checksum
}

fn title(rom: &[u8]) -> String {
    // 0x0143 was the last byte of the title until the CGB claimed it for a
    // compatibility flag, so a cartridge that sets its top bit has 15, not 16.
    let cgb_enhanced = 0x80;
    let end = if rom[addr::CGB_FLAG] & cgb_enhanced == 0 {
        addr::CGB_FLAG + 1
    } else {
        addr::CGB_FLAG
    };

    rom[addr::TITLE..end]
        .iter()
        .take_while(|&&byte| byte != 0)
        .map(|&byte| byte as char)
        .collect()
}

fn rom_size(code: u8) -> usize {
    assert!(code <= 0x08, "unknown ROM size code {code:#04X}");
    0x8000 << code
}

/// Not a shift like the ROM size, and not monotonic - 0x05 is 64 KiB, which is
/// less than 0x04's 128 KiB.
fn ram_size(code: u8) -> usize {
    match code {
        0x00 => 0,
        0x01 => 2 * 1024,
        0x02 => 8 * 1024,
        0x03 => 32 * 1024,
        0x04 => 128 * 1024,
        0x05 => 64 * 1024,
        other => panic!("unknown RAM size code {other:#04X}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HALT: u8 = 0x76;
    const CGB_ENHANCED: u8 = 0x80;

    const CART_TYPE_NONE: u8 = 0x00;
    const CART_TYPE_MBC1: u8 = 0x01;

    const ROM_SIZE_32_KIB: u8 = 0x00;
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
        assert_eq!(header.title, "");
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

        assert_eq!(Header::parse(&rom).title, "TETRIS");
    }

    #[test]
    fn a_cgb_flag_is_not_part_of_the_title() {
        let mut rom = rom_with(CART_TYPE_NONE, ROM_SIZE_32_KIB, RAM_SIZE_NONE);
        rom[addr::TITLE..=addr::CGB_FLAG].fill(b'A');
        rom[addr::CGB_FLAG] = CGB_ENHANCED;
        rom[addr::CHECKSUM] = header_checksum(&rom);

        assert_eq!(Header::parse(&rom).title, "A".repeat(15));
    }

    #[test]
    #[should_panic(expected = "header checksum mismatch")]
    fn a_wrong_checksum_is_rejected() {
        let mut rom = rom_with(CART_TYPE_NONE, ROM_SIZE_32_KIB, RAM_SIZE_NONE);
        rom[addr::CHECKSUM] ^= 0xFF;

        Header::parse(&rom);
    }
}
