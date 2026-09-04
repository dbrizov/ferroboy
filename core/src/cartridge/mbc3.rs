#[cfg(test)]
mod tests;

use crate::cartridge::{Cartridge, Header, ROM_BANK_BYTES, ram_offset, rom_offset};

const RAM_ENABLED: u8 = 0x0A;

pub struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    battery: bool,
    ram_enabled: bool,
    rom_bank: u8,
    ram_bank: u8,
}

impl Mbc3 {
    pub fn new(rom: &[u8], header: &Header) -> Self {
        Self {
            rom: rom.to_vec(),
            ram: vec![0; header.ram_size],
            battery: header.has_battery,
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
        }
    }

    fn rtc_selected(&self) -> bool {
        self.ram_bank >= 0x08
    }
}

impl Cartridge for Mbc3 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = if address < ROM_BANK_BYTES as u16 {
            0
        } else {
            self.rom_bank as usize
        };
        self.rom
            .get(rom_offset(bank, address))
            .copied()
            .unwrap_or(0xFF)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == RAM_ENABLED,
            0x2000..=0x3FFF => {
                self.rom_bank = match value & 0x7F {
                    0 => 1,
                    bank => bank,
                }
            }
            0x4000..=0x5FFF => self.ram_bank = value,
            _ => {}
        }
    }

    fn read_ram(&self, offset: u16) -> u8 {
        if !self.ram_enabled || self.rtc_selected() {
            return 0xFF;
        }
        let offset = ram_offset(self.ram_bank as usize, offset);
        self.ram.get(offset).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, offset: u16, value: u8) {
        if !self.ram_enabled || self.rtc_selected() {
            return;
        }
        let offset = ram_offset(self.ram_bank as usize, offset);
        if let Some(byte) = self.ram.get_mut(offset) {
            *byte = value;
        }
    }

    fn battery_ram(&self) -> Option<&[u8]> {
        self.battery.then_some(self.ram.as_slice())
    }

    fn load_battery_ram(&mut self, saved: &[u8]) {
        let shared = self.ram.len().min(saved.len());
        self.ram[..shared].copy_from_slice(&saved[..shared]);
    }
}
