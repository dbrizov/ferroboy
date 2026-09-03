pub struct Joypad {
    select: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self { select: 0x30 }
    }

    pub fn read(&self) -> u8 {
        self.select | 0xCF // active low: a 1 in bits 0-3 means not held
    }

    pub fn write(&mut self, value: u8) {
        self.select = value & 0x30; // only the two row-select bits are writable
    }
}
