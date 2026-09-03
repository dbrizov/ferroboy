use crate::cartridge::{Cartridge, Header};

pub struct NoMbc {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl NoMbc {
    pub fn new(rom: &[u8], header: &Header) -> Self {
        Self {
            rom: rom.to_vec(),
            ram: vec![0; header.ram_size],
        }
    }
}

impl Cartridge for NoMbc {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom.get(address as usize).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {
        // Do nothing - there is no memory bank controller on the cartridge
    }

    fn read_ram(&self, offset: u16) -> u8 {
        self.ram.get(offset as usize).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, offset: u16, value: u8) {
        if let Some(byte) = self.ram.get_mut(offset as usize) {
            *byte = value;
        }
    }
}
