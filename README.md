# ferroboy
A Game Boy emulator written in Rust

![gif](https://raw.githubusercontent.com/dbrizov/ferroboy/refs/heads/master/docs/ferroboy.gif)

# Build & Run

```
cargo run -p ferroboy_desktop -- <rom.gb>   # boot a cartridge
cargo run -p ferroboy_desktop               # no cartridge, boot ROM only
```

```
cargo test -p ferroboy --release            # run the test ROMs at full speed
cargo fmt && cargo clippy --all-targets
```

# References

## Hardware
- [gbdev.io](https://gbdev.io/)
- [Pan Docs](https://gbdev.io/pandocs/)
- [The Game Boy Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)

## Instruction set
- [gbops](https://izik1.github.io/gbops/)

## Test ROMs
- [Blargg's test ROMs](https://github.com/retrio/gb-test-roms)
- [dmg-acid2](https://github.com/mattcurrie/dmg-acid2)

## Other emulators
- [SameBoy](https://github.com/LIJI32/SameBoy)
- [Gameboy Emulator Development Guide](https://hacktix.github.io/GBEDG/)
