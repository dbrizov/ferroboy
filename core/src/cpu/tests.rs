use super::*;

#[test]
fn b_is_the_high_byte_of_bc() {
    let mut regs = Registers::new_post_boot();
    regs.set_bc(0x1234);
    assert_eq!(regs.b, 0x12);
    assert_eq!(regs.c, 0x34);
    assert_eq!(regs.bc(), 0x1234);
}

#[test]
fn the_low_nibble_of_f_does_not_exist() {
    let mut regs = Registers::new_post_boot();
    regs.set_af(0xFFFF);
    assert_eq!(regs.f, 0xF0);
    assert_eq!(regs.af(), 0xFFF0);
}

#[test]
fn flags_round_trip() {
    let mut regs = Registers::new_post_boot();
    regs.set_flags(true, false, false, true);
    assert!(regs.has_flags(FLAG_Z));
    assert!(regs.has_flags(FLAG_C));
    assert!(regs.has_flags(FLAG_Z | FLAG_C));
    assert!(!regs.has_flags(FLAG_N));
    assert_eq!(regs.f, FLAG_Z | FLAG_C);
}
