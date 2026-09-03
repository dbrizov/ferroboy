#![allow(dead_code)] // TODO remove

mod bus;
mod cpu;
mod emulator;
mod interrupts;
mod ppu;

pub use crate::emulator::Emulator;
pub use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};
