# asciicity

Animated ASCII cityscape for your terminal. A rooftop view of a city skyline with blinking windows, drifting clouds, traffic, planes, helicopters, birds, weather, and a day/night cycle.

<!-- TODO: drop the real GIFs into media/ and these placeholders will render. -->
![asciicity default run](media/demo-default.gif)

### Weather variants

| Rain | Snow |
|---|---|
| ![rain](media/demo-rain.gif) | ![snow](media/demo-snow.gif) |

| Fog | Thunder |
|---|---|
| ![fog](media/demo-fog.gif) | ![thunder](media/demo-thunder.gif) |

## Features

- Procedurally animated (never the same twice)
- Layered rendering with transparent compositing and wide-world parallax scrolling
- Day/night cycle with smooth sky colour transitions and stars that fade in at dusk
- Weather: clear, rain, snow, fog, and full thunderstorms with lightning bolts and sky flashes
- Bidirectional sky traffic (clouds, birds, planes, helicopters); drift direction configurable
- Multi-frame car animations, shared vehicle palette, cloud re-tinting to track the sky
- Configurable spawn rates, weather, and day/night speed via CLI
- External art files with user override support (`~/.config/asciicity/`)
- Per-character colouring via `.colors` palette or `.colormap` positional grid
- Lightweight: ~15 FPS default, minimal CPU usage
- Press any key to exit

## Installation

Requires the [Rust toolchain](https://rustup.rs/).

```bash
git clone https://github.com/Lorakszak/asciicity.git
cd asciicity
cargo install --path .
```

Or build manually:

```bash
cargo build --release
cp target/release/asciicity ~/.local/bin/
```

## Usage

```bash
# Run with defaults
asciicity

# Adjust frame rate
asciicity --fps 10

# Busier city: more cars, less frequent planes, rainy weather
asciicity --car-rate 3 --plane-rate 0.3 --weather rain

# Thunderstorm with clouds drifting from right to left
asciicity --weather thunder --cloud-direction left

# Fast-forward the day/night cycle and start at sunrise
asciicity --time-speed 2 --start-time 5
```

### All flags

| Flag | Default | Description |
|---|---|---|
| `--fps <N>` | `15` | Target frames per second |
| `--cloud-rate <N>` | `1.0` | Cloud spawn multiplier (0 = off) |
| `--plane-rate <N>` | `1.0` | Plane spawn multiplier |
| `--heli-rate <N>` | `1.0` | Helicopter spawn multiplier |
| `--bird-rate <N>` | `1.0` | Bird flock spawn multiplier |
| `--car-rate <N>` | `1.0` | Car spawn multiplier |
| `--cloud-direction <DIR>` | `both` | Cloud drift direction: `left`, `right`, `both` |
| `--weather <TYPE>` | | `clear`, `rain`, `snow`, `fog`, `thunder` |
| `--weather-intensity <N>` | `1.0` | Weather intensity (0.1..3.0) |
| `--time-speed <N>` | `0.2` | In-game hours per real second |
| `--start-time <N>` | `20.0` | Starting hour of day (0..24) |
| `-h, --help` | | Print help |
| `-V, --version` | | Print version |

Rate multipliers scale spawn intervals inversely: `2.0` is twice as often, `0.5` is half as often, `0.0` disables that entity entirely.

### Auto-start with your terminal

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
asciicity
```

## Customising the art

Drop override files into `~/.config/asciicity/` using the same filenames as under `assets/` in this repo. For example, `~/.config/asciicity/plane.txt` replaces the default plane art. Optional `.colors` (character palette) and `.colormap` (positional grid) files tune per-character colour. If an override file is missing or unreadable, the compiled-in default is used.

## Building from source

```bash
cargo build --release
./target/release/asciicity
```

Run the quality gate during development:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## License

[MIT](LICENSE)

## Built with

[ratatui](https://ratatui.rs/) and [crossterm](https://github.com/crossterm-rs/crossterm).
