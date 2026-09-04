#[cfg(test)]
mod tests;

mod mbc1;
mod mbc3;
mod mbc5;
mod nombc;
mod unplugged;

use crate::cartridge::mbc1::Mbc1;
use crate::cartridge::mbc3::Mbc3;
use crate::cartridge::mbc5::Mbc5;
use crate::cartridge::nombc::NoMbc;
use crate::cartridge::unplugged::Unplugged;

pub const ROM_BANK_BYTES: usize = 0x4000;
pub const RAM_BANK_BYTES: usize = 0x2000;

fn rom_offset(bank: usize, address: u16) -> usize {
    bank * ROM_BANK_BYTES + address as usize % ROM_BANK_BYTES
}

fn ram_offset(bank: usize, offset: u16) -> usize {
    bank * RAM_BANK_BYTES + offset as usize
}

pub trait Cartridge {
    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self, offset: u16) -> u8;
    fn write_ram(&mut self, offset: u16, value: u8);

    fn battery_ram(&self) -> Option<&[u8]> {
        None
    }

    fn load_battery_ram(&mut self, _saved: &[u8]) {}
}

pub fn load(rom: &[u8]) -> Box<dyn Cartridge> {
    let header = Header::parse(rom);
    match header.cart_type {
        0x00 => Box::new(NoMbc::new(rom, &header)),
        0x01..=0x03 => Box::new(Mbc1::new(rom, &header)),
        0x0F..=0x13 => Box::new(Mbc3::new(rom, &header)),
        0x19..=0x1E => Box::new(Mbc5::new(rom, &header)),
        other => panic!("unsupported cartridge type {other:#04X}"),
    }
}

pub fn unplugged() -> Box<dyn Cartridge> {
    Box::new(Unplugged)
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
    pub _title: String,
    pub cart_type: u8,
    pub _rom_size: usize,
    pub ram_size: usize,
    pub has_battery: bool,
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
            _title: title(rom),
            cart_type: rom[addr::CART_TYPE],
            _rom_size: rom_size(rom[addr::ROM_SIZE]),
            ram_size: ram_size(rom[addr::RAM_SIZE]),
            has_battery: has_battery(rom[addr::CART_TYPE]),
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

fn has_battery(cart_type: u8) -> bool {
    matches!(
        cart_type,
        0x03 | 0x06 | 0x09 | 0x0D | 0x0F | 0x10 | 0x13 | 0x1B | 0x1E | 0x22 | 0xFF
    )
}

fn rom_size(code: u8) -> usize {
    assert!(code <= 0x08, "unknown ROM size code {code:#04X}");
    0x8000 << code
}

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
