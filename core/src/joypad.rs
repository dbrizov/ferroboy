#[cfg(test)]
mod tests;

use crate::interrupts;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Button {
    Right = 1 << 0,
    Left = 1 << 1,
    Up = 1 << 2,
    Down = 1 << 3,
    A = 1 << 4,
    B = 1 << 5,
    Select = 1 << 6,
    Start = 1 << 7,
}

const SELECT_ACTIONS: u8 = 1 << 5;
const SELECT_DPAD: u8 = 1 << 4;
const SELECTABLE: u8 = SELECT_ACTIONS | SELECT_DPAD;

pub struct Joypad {
    select: u8,
    buttons: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            select: SELECTABLE,
            buttons: 0,
        }
    }

    pub fn read(&self) -> u8 {
        let mut row = 0x0F;

        if self.select & SELECT_DPAD == 0 {
            row &= !(self.buttons & 0x0F);
        }
        if self.select & SELECT_ACTIONS == 0 {
            row &= !(self.buttons >> 4);
        }

        self.select | 0xC0 | row
    }

    pub fn write(&mut self, value: u8) {
        self.select = value & SELECTABLE;
    }

    pub fn set(&mut self, button: Button, pressed: bool) -> u8 {
        let before = self.read();

        if pressed {
            self.buttons |= button as u8;
        } else {
            self.buttons &= !(button as u8);
        }

        let selected_button_pressed = before & !self.read() & 0x0F != 0;
        if selected_button_pressed {
            interrupts::JOYPAD
        } else {
            0
        }
    }
}
