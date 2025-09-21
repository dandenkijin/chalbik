# Chalbik - Klingon Rain TUI

Chalbik is a terminal-based rain animation using tui-rain crate with pIqaD (Klingon script) characters, inspired by matrix-style rain but in Klingon.

## Requirements

- Rust and Cargo (install via [rustup](https://rustup.rs/))
- **Font**: Code2001 font (for pIqaD glyphs). Download from [here](https://www.fontspace.com/code2001-font-f1004) and install it. Set your terminal font to Code2001.
- **Terminal**: Tested only on Kitty. May work on other terminals supporting Unicode, but glyph rendering might vary.

## Building

```bash
git clone https://github.com/dandenkijin/chalbik
cd chalbik
cargo build --release
```

## Running

```bash
./target/release/chalbik
```

### Options

- `-t, --tail-color <COLOR>`: Rain trail color (default: red)
- `-d, --head-color <COLOR>`: Leading drop color (default: yellow)
- `-s, --speed <SPEED>`: Speed: slow|fast (default: fast)
- `-l, --tail-length <LENGTH>`: Trail lifespan in seconds (default: 10)
- `-h, --help`: Print help

Example: `./target/release/chalbik -t green -d light_green -s slow -l 5`

### Available Colors

black, red, green, yellow, blue, magenta, cyan, white, light_green, etc.

## Status

Early alpha – it works, but expect bugs. Report issues on GitHub.

Quit with q or Esc.