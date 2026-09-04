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

// The sound ROMs report through cartridge RAM rather than the link port:
// 0x80 while running, then a result code with a DE B0 61 signature behind it.
fn assert_passes_in_ram(name: &str) {
    let rom = std::fs::read(format!("tests/roms/{name}"))
        .unwrap_or_else(|error| panic!("cannot read {name}: {error}"));
    let mut emulator = Emulator::new(&rom);
    let mut running = false;

    for _ in 0..MAX_STEPS {
        emulator.step();

        let signature = [0xA001, 0xA002, 0xA003].map(|address| emulator.read(address));
        if signature != [0xDE, 0xB0, 0x61] {
            continue;
        }

        // The signature lands before the marker does, so a result is only
        // trustworthy once the ROM has been seen holding 0x80.
        let code = emulator.read(0xA000);
        if code == 0x80 {
            running = true;
            continue;
        }
        if !running {
            continue;
        }

        assert_eq!(code, 0, "{name} failed check #{code}");
        return;
    }

    panic!("{name} produced no verdict in {MAX_STEPS} steps");
}

#[test]
fn cpu_instrs_01_special() {
    assert_passes("01_special.gb");
}

#[test]
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

#[test]
fn sound_01_registers() {
    assert_passes_in_ram("sound_01_registers.gb");
}

#[test]
fn sound_02_len_ctr() {
    assert_passes_in_ram("sound_02_len_ctr.gb");
}

#[test]
fn sound_03_trigger() {
    assert_passes_in_ram("sound_03_trigger.gb");
}

#[test]
fn sound_04_sweep() {
    assert_passes_in_ram("sound_04_sweep.gb");
}

#[test]
fn sound_05_sweep_details() {
    assert_passes_in_ram("sound_05_sweep_details.gb");
}

#[test]
fn sound_06_overflow_on_trigger() {
    assert_passes_in_ram("sound_06_overflow_on_trigger.gb");
}

#[test]
fn sound_07_len_sweep_period_sync() {
    assert_passes_in_ram("sound_07_len_sweep_period_sync.gb");
}

#[test]
fn sound_08_len_ctr_during_power() {
    assert_passes_in_ram("sound_08_len_ctr_during_power.gb");
}

#[test]
fn sound_09_wave_read_while_on() {
    assert_passes_in_ram("sound_09_wave_read_while_on.gb");
}

#[test]
fn sound_10_wave_trigger_while_on() {
    assert_passes_in_ram("sound_10_wave_trigger_while_on.gb");
}

#[test]
fn sound_11_regs_after_power() {
    assert_passes_in_ram("sound_11_regs_after_power.gb");
}

#[test]
fn sound_12_wave_write_while_on() {
    assert_passes_in_ram("sound_12_wave_write_while_on.gb");
}
