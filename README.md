# ferroboy
A Game Boy emulator written in Rust

# Build & Run
A Rust toolchain with edition 2024 support. No other dependencies — `ferroboy` itself has
none, and `ferroboy_desktop` pulls `winit` and `pixels` through cargo.

```
cargo run -p ferroboy_desktop -- <rom.gb>   # boot a cartridge
cargo run -p ferroboy_desktop               # no cartridge, boot ROM only
```

```
cargo test -p ferroboy --release            # run the test ROMs at full speed
cargo fmt && cargo clippy --all-targets
```

The boot ROM runs on every launch. It is ferroboy's own — see `NOTICES.md`.

# References

## Hardware
- [Pan Docs](https://gbdev.io/pandocs/) — the authority on DMG behaviour, and what this
  emulator is written against
- [gbdev.io](https://gbdev.io/) — the wider community's documentation index
- [The Game Boy Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf) —
  Gekkio, for the parts Pan Docs leaves to measurement

## Instruction set
- [gbops](https://izik1.github.io/gbops/) — the machine-readable opcode table
  `core/src/cpu/opcodes.rs` is generated from
- [pastraiser opcode table](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) —
  mnemonics and lengths. Its `BIT n,(HL)` timings are four cycles too high, so gbops wins
  on anything numeric

## Test ROMs
- [Blargg's test ROMs](https://github.com/retrio/gb-test-roms) — `cpu_instrs` is the CPU
  conformance gate; `instr_timing` and `mem_timing` come later
- [dmg-acid2](https://github.com/mattcurrie/dmg-acid2) — one image drawn out of every PPU
  feature, with a documented failure mode per feature
- [Mooneye Test Suite](https://github.com/Gekkio/mooneye-test-suite) — Gekkio, for accuracy
  work past the point Blargg stops caring

## Other emulators
- [SameBoy](https://github.com/LIJI32/SameBoy) — writes its own boot ROMs in assembly rather
  than shipping Nintendo's, which is the approach `core/src/boot_rom.rs` follows
- [Gameboy Emulator Development Guide](https://hacktix.github.io/GBEDG/) — a readable
  walkthrough of the PPU and timer
