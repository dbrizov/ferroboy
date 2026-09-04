use super::*;

const DPAD: u8 = 0x20;
const ACTIONS: u8 = 0x10;

#[test]
fn nothing_held_reads_all_ones() {
    let mut joypad = Joypad::new();
    joypad.write(DPAD);

    assert_eq!(joypad.read(), 0xEF);
}

#[test]
fn a_held_button_reads_as_zero_in_its_row() {
    let mut joypad = Joypad::new();
    joypad.write(DPAD);
    joypad.set(Button::Left, true);

    assert_eq!(joypad.read() & 0x0F, 0b1101, "bit 1 goes low");
}

#[test]
fn the_rows_are_multiplexed_not_masked() {
    let mut joypad = Joypad::new();
    joypad.set(Button::Left, true);
    joypad.set(Button::A, true);

    joypad.write(DPAD);
    assert_eq!(joypad.read() & 0x0F, 0b1101, "Left, and A is not visible");

    joypad.write(ACTIONS);
    assert_eq!(joypad.read() & 0x0F, 0b1110, "A, and Left is not visible");
}

#[test]
fn selecting_neither_row_shows_nothing() {
    let mut joypad = Joypad::new();
    joypad.set(Button::Left, true);
    joypad.write(SELECTABLE);

    assert_eq!(joypad.read() & 0x0F, 0x0F, "held, but nothing is connected");
}

#[test]
fn selecting_both_rows_ands_them_together() {
    let mut joypad = Joypad::new();
    joypad.set(Button::Left, true);
    joypad.set(Button::Start, true);
    joypad.write(0x00);

    assert_eq!(joypad.read() & 0x0F, 0b0101);
}

#[test]
fn only_the_row_select_bits_are_writable() {
    let mut joypad = Joypad::new();
    joypad.write(0x00);

    assert_eq!(joypad.read() & 0xC0, 0xC0, "bits 6-7 do not exist");
    assert_eq!(joypad.read() & SELECTABLE, 0x00);
}

#[test]
fn pressing_a_selected_button_interrupts() {
    let mut joypad = Joypad::new();
    joypad.write(DPAD);

    assert_eq!(joypad.set(Button::Left, true), interrupts::JOYPAD);
    assert_eq!(joypad.set(Button::Left, false), 0, "releasing does not");
}

#[test]
fn a_button_in_an_unselected_row_does_not_interrupt() {
    let mut joypad = Joypad::new();
    joypad.write(DPAD);

    assert_eq!(joypad.set(Button::A, true), 0, "wrong row");

    joypad.write(SELECTABLE);
    assert_eq!(joypad.set(Button::Start, true), 0, "no row selected at all");
}
