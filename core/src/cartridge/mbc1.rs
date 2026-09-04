#[cfg(test)]
mod tests;

use crate::cartridge::{Cartridge, Header, ROM_BANK_BYTES, ram_offset, rom_offset};

const RAM_ENABLED: u8 = 0x0A;

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    has_battery: bool,
    ram_enabled: bool,
    bank: u8,
    upper: u8,
    in_ram_mode: bool,
}

impl Mbc1 {
    pub fn new(rom: &[u8], header: &Header) -> Self {
        Self {
            rom: rom.to_vec(),
            ram: vec![0; header.ram_size],
            has_battery: header.has_battery,
            ram_enabled: false,
            bank: 1,
            upper: 0,
            in_ram_mode: false,
        }
    }

    fn rom_bank(&self, address: u16) -> usize {
        if address < ROM_BANK_BYTES as u16 {
            if self.in_ram_mode {
                (self.upper as usize) << 5
            } else {
                0
            }
        } else {
            let low = if self.bank & 0x1F == 0 {
                1
            } else {
                self.bank & 0x1F
            };
            (self.upper as usize) << 5 | low as usize
        }
    }

    fn ram_bank(&self) -> usize {
        if self.in_ram_mode {
            self.upper as usize
        } else {
            0
        }
    }
}

impl Cartridge for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        let offset = rom_offset(self.rom_bank(address), address);
        self.rom.get(offset).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == RAM_ENABLED,
            0x2000..=0x3FFF => self.bank = value & 0x1F,
            0x4000..=0x5FFF => self.upper = value & 0x03,
            _ => self.in_ram_mode = value & 0x01 != 0,
        }
    }

    fn read_ram(&self, offset: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        let offset = ram_offset(self.ram_bank(), offset);
        self.ram.get(offset).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, offset: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }
        let offset = ram_offset(self.ram_bank(), offset);
        if let Some(byte) = self.ram.get_mut(offset) {
            *byte = value;
        }
    }

    fn battery_ram(&self) -> Option<&[u8]> {
        self.has_battery.then_some(self.ram.as_slice())
    }

    fn load_battery_ram(&mut self, saved: &[u8]) {
        let shared = self.ram.len().min(saved.len());
        self.ram[..shared].copy_from_slice(&saved[..shared]);
    }
}
