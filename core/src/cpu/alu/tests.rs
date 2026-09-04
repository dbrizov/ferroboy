use super::*;

fn with(a: u8, f: u8) -> Cpu {
    let mut cpu = Cpu::new();
    cpu.regs.a = a;
    cpu.regs.f = f;
    cpu
}

fn flags(cpu: &Cpu) -> String {
    [FLAG_Z, FLAG_N, FLAG_H, FLAG_C]
        .iter()
        .zip("ZNHC".chars())
        .map(|(&mask, name)| if cpu.regs.has_flags(mask) { name } else { '-' })
        .collect()
}

#[test]
fn add8_half_carry_is_out_of_bit_3() {
    let cases = [
        (0x00, 0x00, 0x00, "Z---"),
        (0x0E, 0x01, 0x0F, "----"),
        (0x0F, 0x01, 0x10, "--H-"),
        (0x08, 0x08, 0x10, "--H-"),
        (0x80, 0x80, 0x00, "Z--C"),
        (0xFF, 0x01, 0x00, "Z-HC"),
        (0xFF, 0xFF, 0xFE, "--HC"),
    ];
    for (a, value, result, expected) in cases {
        let mut cpu = with(a, 0);
        cpu.add8(value);
        assert_eq!(cpu.regs.a, result, "{a:#04X} + {value:#04X}");
        assert_eq!(flags(&cpu), expected, "{a:#04X} + {value:#04X}");
    }
}

#[test]
fn adc8_folds_the_carry_into_both_tests() {
    let mut cpu = with(0x00, FLAG_C);
    cpu.adc8(0xFF);
    assert_eq!(cpu.regs.a, 0x00);
    assert_eq!(flags(&cpu), "Z-HC");

    let mut cpu = with(0x0F, FLAG_C);
    cpu.adc8(0x00);
    assert_eq!(cpu.regs.a, 0x10);
    assert_eq!(flags(&cpu), "--H-");
}

#[test]
fn sub8_borrows_the_same_way() {
    let cases = [
        (0x00, 0x00, 0x00, "ZN--"),
        (0x10, 0x01, 0x0F, "ZN--"),
        (0x00, 0x01, 0xFF, "ZNHC"),
        (0x3E, 0x0F, 0x2F, "ZNH-"),
    ];
    for (a, value, result, _) in cases {
        let mut cpu = with(a, 0);
        cpu.sub8(value);
        assert_eq!(cpu.regs.a, result, "{a:#04X} - {value:#04X}");
    }

    let mut cpu = with(0x10, 0);
    cpu.sub8(0x01);
    assert_eq!(flags(&cpu), "-NH-");
}

#[test]
fn sbc8_borrows_the_carry_too() {
    let mut cpu = with(0x00, FLAG_C);
    cpu.sbc8(0x00);
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(flags(&cpu), "-NHC");
}

#[test]
fn cp8_sets_the_flags_but_keeps_a() {
    let mut cpu = with(0x3C, 0);
    cpu.cp8(0x3C);
    assert_eq!(cpu.regs.a, 0x3C);
    assert_eq!(flags(&cpu), "ZN--");
}

#[test]
fn and_sets_h_and_the_others_clear_it() {
    let mut cpu = with(0x5A, FLAG_C);
    cpu.and8(0x3F);
    assert_eq!(flags(&cpu), "--H-");

    let mut cpu = with(0x5A, FLAG_C);
    cpu.or8(0x00);
    assert_eq!(flags(&cpu), "----");

    let mut cpu = with(0xFF, FLAG_C);
    cpu.xor8(0xFF);
    assert_eq!(flags(&cpu), "Z---");
}

#[test]
fn inc_and_dec_leave_the_carry_alone() {
    let mut cpu = with(0, FLAG_C);
    assert_eq!(cpu.inc8(0x0F), 0x10);
    assert_eq!(flags(&cpu), "--HC");

    let mut cpu = with(0, FLAG_C);
    assert_eq!(cpu.dec8(0x00), 0xFF);
    assert_eq!(flags(&cpu), "-NHC");

    let mut cpu = with(0, 0);
    assert_eq!(cpu.inc8(0xFF), 0x00);
    assert_eq!(flags(&cpu), "Z-H-");
}

#[test]
fn add16_half_carry_is_out_of_bit_11() {
    let mut cpu = with(0, FLAG_Z);
    cpu.regs.set_hl(0x0FFF);
    cpu.add16(0x0001);
    assert_eq!(cpu.regs.hl(), 0x1000);
    assert_eq!(flags(&cpu), "Z-H-", "Z is preserved by ADD HL,rr");

    let mut cpu = with(0, 0);
    cpu.regs.set_hl(0xFFFF);
    cpu.add16(0x0001);
    assert_eq!(cpu.regs.hl(), 0x0000);
    assert_eq!(flags(&cpu), "--HC");
}

#[test]
fn add_sp_i8_takes_its_flags_from_the_low_byte() {
    let mut cpu = with(0, FLAG_Z);
    cpu.sp = 0x000F;
    assert_eq!(cpu.add_sp_i8(1), 0x0010);
    assert_eq!(
        flags(&cpu),
        "--H-",
        "Z is cleared even though nothing is zero"
    );

    let mut cpu = with(0, 0);
    cpu.sp = 0x00FF;
    assert_eq!(cpu.add_sp_i8(1), 0x0100);
    assert_eq!(flags(&cpu), "--HC");

    let mut cpu = with(0, 0);
    cpu.sp = 0x0005;
    assert_eq!(cpu.add_sp_i8(-1), 0x0004);
    assert_eq!(flags(&cpu), "--HC");
}

#[test]
fn daa_reads_the_original_a_for_both_adjustments() {
    let mut cpu = with(0x09, 0);
    cpu.add8(0x08);
    cpu.daa();
    assert_eq!(cpu.regs.a, 0x17);

    let mut cpu = with(0x99, 0);
    cpu.add8(0x01);
    cpu.daa();
    assert_eq!(cpu.regs.a, 0x00);
    assert_eq!(flags(&cpu), "Z--C");

    let mut cpu = with(0x10, 0);
    cpu.sub8(0x01);
    cpu.daa();
    assert_eq!(cpu.regs.a, 0x09);
}

#[test]
fn cpl_scf_and_ccf() {
    let mut cpu = with(0x35, 0);
    cpu.cpl();
    assert_eq!(cpu.regs.a, 0xCA);
    assert_eq!(flags(&cpu), "-NH-");

    let mut cpu = with(0, FLAG_Z | FLAG_N | FLAG_H);
    cpu.scf();
    assert_eq!(flags(&cpu), "Z--C");

    let mut cpu = with(0, FLAG_C);
    cpu.ccf();
    assert_eq!(flags(&cpu), "----");
}

#[test]
fn the_a_rotates_always_clear_z_but_the_cb_ones_do_not() {
    let mut cpu = with(0x00, 0);
    cpu.rlca();
    assert_eq!(cpu.regs.a, 0x00);
    assert_eq!(flags(&cpu), "----", "RLCA clears Z even on a zero result");

    let mut cpu = with(0, 0);
    assert_eq!(cpu.rlc8(0x00), 0x00);
    assert_eq!(flags(&cpu), "Z---", "CB RLC sets Z from the result");
}

#[test]
fn rotates_move_the_right_bit_into_carry() {
    let mut cpu = with(0x85, 0);
    cpu.rlca();
    assert_eq!(cpu.regs.a, 0x0B);
    assert_eq!(flags(&cpu), "---C");

    let mut cpu = with(0x3B, FLAG_C);
    cpu.rra();
    assert_eq!(cpu.regs.a, 0x9D);
    assert_eq!(flags(&cpu), "---C");

    let mut cpu = with(0, FLAG_C);
    assert_eq!(cpu.rl8(0x80), 0x01);
    assert_eq!(flags(&cpu), "---C");
}

#[test]
fn sra_keeps_the_sign_and_srl_does_not() {
    let mut cpu = with(0, 0);
    assert_eq!(cpu.sra8(0x8A), 0xC5);
    assert_eq!(flags(&cpu), "----");

    let mut cpu = with(0, 0);
    assert_eq!(cpu.srl8(0x8A), 0x45);
    assert_eq!(flags(&cpu), "----");

    let mut cpu = with(0, 0);
    assert_eq!(cpu.sla8(0x80), 0x00);
    assert_eq!(flags(&cpu), "Z--C");
}

#[test]
fn swap_and_bit() {
    let mut cpu = with(0, 0);
    assert_eq!(cpu.swap8(0x1F), 0xF1);
    assert_eq!(flags(&cpu), "----");

    let mut cpu = with(0, FLAG_C);
    cpu.bit8(7, 0x80);
    assert_eq!(flags(&cpu), "--HC", "BIT leaves C alone");

    let mut cpu = with(0, 0);
    cpu.bit8(0, 0xFE);
    assert_eq!(flags(&cpu), "Z-H-");
}
