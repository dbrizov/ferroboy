# ferroboy
A Game Boy emulator written in Rust.<br>
Supports DMG and CGB ROMs.

![gif](https://raw.githubusercontent.com/dbrizov/ferroboy/refs/heads/master/docs/ferroboy.gif)

# Build & Run
You can play it in a web browser [here](https://denisrizov.com/games/ferroboy/index.html).<br>
The web demo boots `Tobu Tobu Girl` - an open source game by Tangram Games. Check `NOTICES.md`.

## Desktop
```
cargo run -p ferroboy_desktop -- <rom.gb>   # boot a cartridge
cargo run -p ferroboy_desktop               # no cartridge, boot ROM only
```

## Web
```
cargo install trunk                         # one-time setup for the web build
trunk serve --release                       # then open http://localhost:8080

```
 ## Tests
```
cargo test -p ferroboy --release            # run the test ROMs at full speed
```

# Controls
| Game Boy | Keyboard | XBOX Controller |
| --- | --- | --- |
| D-pad | Arrow keys | D-pad |
| A | X | B |
| B | Z | A |
| Select | Backspace | Back |
| Start | Enter | Start |

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
- [cgb-acid2](https://github.com/mattcurrie/cgb-acid2)

## Other emulators
- [SameBoy](https://github.com/LIJI32/SameBoy)
- [binjgb](https://github.com/binji/binjgb)
