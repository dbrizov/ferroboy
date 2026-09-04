use crate::cartridge::Cartridge;

pub struct Unplugged;

impl Cartridge for Unplugged {
    fn read_rom(&self, _address: u16) -> u8 {
        0xFF
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {}

    fn read_ram(&self, _offset: u16) -> u8 {
        0xFF
    }

    fn write_ram(&mut self, _offset: u16, _value: u8) {}
}
