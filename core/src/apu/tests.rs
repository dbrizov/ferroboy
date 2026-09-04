use super::*;

const NR11: u16 = 0xFF11;
const NR12: u16 = 0xFF12;
const NR30: u16 = 0xFF1A;
const NR34: u16 = 0xFF1E;

const CHANNEL_1: u8 = 1 << 0;
const CHANNEL_3: u8 = 1 << 2;

fn powered_on() -> Apu {
    let mut apu = Apu::new(false);
    apu.write(addr::NR52, NR52_ENABLED);
    apu
}

fn advance(apu: &mut Apu, t_cycles: u32) {
    for _ in 0..t_cycles / 4 {
        apu.tick(4);
    }
}

fn active(apu: &Apu, channel: u8) -> bool {
    apu.read(addr::NR52) & channel != 0
}

fn start_square1(apu: &mut Apu, length: u8, control: u8) {
    apu.write(NR12, 0xF0);
    apu.write(NR11, length);
    apu.write(addr::NR14, control);
}

#[test]
fn a_powered_off_apu_ignores_writes() {
    let mut apu = Apu::new(false);
    apu.write(addr::NR50, 0x77);
    apu.write(addr::NR51, 0xF3);

    assert_eq!(apu.read(addr::NR50), 0);
    assert_eq!(apu.read(addr::NR51), 0);
}

#[test]
fn powering_off_clears_the_registers() {
    let mut apu = powered_on();
    apu.write(addr::NR50, 0x77);
    apu.write(addr::NR52, 0);

    assert_eq!(apu.read(addr::NR50), 0);
}

#[test]
fn wave_ram_stays_writable_with_the_apu_off() {
    let mut apu = Apu::new(false);
    apu.write(addr::WAVE_START, 0xAB);

    assert_eq!(apu.read(addr::WAVE_START), 0xAB);
}

#[test]
fn powering_off_leaves_wave_ram_alone() {
    let mut apu = powered_on();
    apu.write(addr::WAVE_END, 0xCD);
    apu.write(addr::NR52, 0);

    assert_eq!(apu.read(addr::WAVE_END), 0xCD);
}

#[test]
fn nr52_reads_with_the_unused_bits_set() {
    assert_eq!(Apu::new(false).read(addr::NR52), 0x70);
}

#[test]
fn triggering_a_channel_with_a_live_dac_starts_it() {
    let mut apu = powered_on();
    start_square1(&mut apu, 0, 0x80);

    assert!(active(&apu, CHANNEL_1));
}

#[test]
fn triggering_a_channel_with_a_dead_dac_leaves_it_silent() {
    let mut apu = powered_on();
    apu.write(NR12, 0x00);
    apu.write(addr::NR14, 0x80);

    assert!(!active(&apu, CHANNEL_1));
}

#[test]
fn an_expiring_length_counter_stops_the_channel() {
    let mut apu = powered_on();
    start_square1(&mut apu, 0x3F, 0xC0);
    assert!(active(&apu, CHANNEL_1));

    advance(&mut apu, CYCLES_PER_FRAME_STEP);

    assert!(!active(&apu, CHANNEL_1));
}

// The two halves of the quirk: enabling the counter on a step that will not
// clock it steals the clock the channel would otherwise have kept.
#[test]
fn enabling_length_between_length_steps_steals_a_clock() {
    let mut apu = powered_on();
    advance(&mut apu, CYCLES_PER_FRAME_STEP);
    start_square1(&mut apu, 0x3F, 0x80);
    assert!(active(&apu, CHANNEL_1));

    apu.write(addr::NR14, 0x40);

    assert!(!active(&apu, CHANNEL_1));
}

#[test]
fn enabling_length_on_a_length_step_does_not() {
    let mut apu = powered_on();
    start_square1(&mut apu, 0x3F, 0x80);

    apu.write(addr::NR14, 0x40);

    assert!(active(&apu, CHANNEL_1));
}

// Loading a counter with the APU off is allowed, and powering off keeps the
// counter that is already there - two separate halves of the same DMG quirk.
#[test]
fn a_length_counter_can_be_loaded_while_the_apu_is_off() {
    let mut apu = Apu::new(false);
    apu.write(NR11, 0x3F);
    apu.write(addr::NR52, NR52_ENABLED);
    start_square1(&mut apu, 0x3F, 0xC0);

    advance(&mut apu, CYCLES_PER_FRAME_STEP);

    assert!(!active(&apu, CHANNEL_1));
}

#[test]
fn powering_off_keeps_the_length_counters() {
    let mut apu = powered_on();
    apu.write(NR11, 0x3F);
    apu.write(addr::NR52, 0);
    apu.write(addr::NR52, NR52_ENABLED);

    apu.write(NR12, 0xF0);
    apu.write(addr::NR14, 0xC0);
    advance(&mut apu, CYCLES_PER_FRAME_STEP);

    assert!(!active(&apu, CHANNEL_1));
}

#[test]
fn a_running_wave_channel_hides_its_ram() {
    let mut apu = powered_on();
    apu.write(addr::WAVE_START, 0xAB);
    apu.write(NR30, 0x80);
    apu.write(NR34, 0x80);
    assert!(active(&apu, CHANNEL_3));

    assert_eq!(apu.read(addr::WAVE_START), 0xFF);
    apu.write(addr::WAVE_START, 0x00);
    apu.write(NR30, 0x00);

    assert_eq!(apu.read(addr::WAVE_START), 0xAB);
}

#[test]
fn leaving_sweep_negate_after_a_calculation_stops_the_channel() {
    let mut apu = powered_on();
    apu.write(addr::NR10, 0x09);
    start_square1(&mut apu, 0, 0x80);
    assert!(active(&apu, CHANNEL_1));

    apu.write(addr::NR10, 0x01);

    assert!(!active(&apu, CHANNEL_1));
}

#[test]
fn the_frame_sequencer_restarts_when_the_apu_powers_on() {
    let mut apu = powered_on();
    advance(&mut apu, CYCLES_PER_FRAME_STEP);
    apu.write(addr::NR52, 0);
    apu.write(addr::NR52, NR52_ENABLED);

    start_square1(&mut apu, 0x3F, 0x80);
    apu.write(addr::NR14, 0x40);

    assert!(active(&apu, CHANNEL_1));
}

#[test]
fn cgb_exposes_wave_ram_while_the_channel_runs() {
    let mut apu = Apu::new(true);
    apu.write(addr::NR52, NR52_ENABLED);
    apu.write(addr::WAVE_START, 0xAB);
    apu.write(NR30, 0x80);
    apu.write(NR34, 0x80);
    assert!(active(&apu, CHANNEL_3));

    assert_eq!(apu.read(addr::WAVE_START), 0xAB);
    apu.write(addr::WAVE_START, 0xCD);
    apu.write(NR30, 0x00);
    assert_eq!(apu.read(addr::WAVE_START), 0xCD);
}
