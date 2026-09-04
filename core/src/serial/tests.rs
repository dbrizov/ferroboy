use super::*;

#[test]
fn starting_a_transfer_captures_the_byte_and_finishes_at_once() {
    let mut serial = Serial::new();
    serial.write(addr::SB, b'A');
    serial.write(addr::SC, 0x81);

    assert_eq!(serial.take_byte(), Some(b'A'));
    assert_eq!(serial.take_byte(), None);
    assert_eq!(serial.read(addr::SC) & 0x80, 0);
}

#[test]
fn writing_sb_alone_transfers_nothing() {
    let mut serial = Serial::new();
    serial.write(addr::SB, b'A');
    assert_eq!(serial.take_byte(), None);
}
