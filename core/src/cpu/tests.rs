use super::*;

fn post_boot_registers() -> Registers {
    Registers {
        a: 0x01,
        f: 0xB0,
        b: 0x00,
        c: 0x13,
        d: 0x00,
        e: 0xD8,
        h: 0x01,
        l: 0x4D,
    }
}

#[test]
fn b_is_the_high_byte_of_bc() {
    let mut regs = post_boot_registers();
    regs.set_bc(0x1234);
    assert_eq!(regs.b, 0x12);
    assert_eq!(regs.c, 0x34);
    assert_eq!(regs.bc(), 0x1234);
}

#[test]
fn the_low_nibble_of_f_does_not_exist() {
    let mut regs = post_boot_registers();
    regs.set_af(0xFFFF);
    assert_eq!(regs.f, 0xF0);
    assert_eq!(regs.af(), 0xFFF0);
}

#[test]
fn flags_round_trip() {
    let mut regs = post_boot_registers();
    regs.set_flags(true, false, false, true);
    assert!(regs.has_flags(FLAG_Z));
    assert!(regs.has_flags(FLAG_C));
    assert!(regs.has_flags(FLAG_Z | FLAG_C));
    assert!(!regs.has_flags(FLAG_N));
    assert_eq!(regs.f, FLAG_Z | FLAG_C);
}
