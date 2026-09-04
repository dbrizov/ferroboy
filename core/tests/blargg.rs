use ferroboy::Emulator;

const MAX_STEPS: u32 = 60_000_000;

fn assert_passes(name: &str) {
    let rom = std::fs::read(format!("tests/roms/{name}"))
        .unwrap_or_else(|error| panic!("cannot read {name}: {error}"));
    let mut emulator = Emulator::new(&rom);
    let mut output = String::new();

    for _ in 0..MAX_STEPS {
        emulator.step();

        let Some(byte) = emulator.take_serial_byte() else {
            continue;
        };
        output.push(byte as char);

        // The verdict arrives before the number that explains it, so keep
        // draining until the ROM stops talking.
        if output.contains("Passed") || output.contains("Failed") {
            for _ in 0..200_000 {
                emulator.step();
                if let Some(byte) = emulator.take_serial_byte() {
                    output.push(byte as char);
                }
            }

            println!("{}", output.trim());
            assert!(output.contains("Passed"), "{}", output.trim());
            return;
        }
    }

    panic!("{name} produced no verdict in {MAX_STEPS} steps:\n{output}");
}

#[test]
fn cpu_instrs_01_special() {
    assert_passes("01_special.gb");
}

#[test]
#[ignore = "needs service_interrupt and a working timer"]
fn cpu_instrs_02_interrupts() {
    assert_passes("02_interrupts.gb");
}

#[test]
fn cpu_instrs_03_op_sp_hl() {
    assert_passes("03_op_sp_hl.gb");
}

#[test]
fn cpu_instrs_04_op_r_imm() {
    assert_passes("04_op_r_imm.gb");
}

#[test]
fn cpu_instrs_05_op_rp() {
    assert_passes("05_op_rp.gb");
}

#[test]
fn cpu_instrs_06_ld_r_r() {
    assert_passes("06_ld_r_r.gb");
}

#[test]
fn cpu_instrs_07_jr_jp_call_ret_rst() {
    assert_passes("07_jr_jp_call_ret_rst.gb");
}

#[test]
fn cpu_instrs_08_misc_instrs() {
    assert_passes("08_misc_instrs.gb");
}

#[test]
fn cpu_instrs_09_op_r_r() {
    assert_passes("09_op_r_r.gb");
}

#[test]
fn cpu_instrs_10_bit_ops() {
    assert_passes("10_bit_ops.gb");
}

#[test]
fn cpu_instrs_11_op_a_hl() {
    assert_passes("11_op_a_hl.gb");
}
