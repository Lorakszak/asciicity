# Terminart

Terminal ambiance engine - beautiful ASCII art scenes for your terminal.

Terminart displays procedurally generated, animated ASCII art scenes directly in your terminal. Think of it as a screensaver for your shell - city skylines with blinking windows, drifting clouds, planes, and cars.

## Features

- Procedurally generated scenes (never the same twice)
- Layered rendering with transparent compositing and wide-world parallax scrolling
- Entity system with multi-frame sprites, auto-mirroring, and sinusoidal bobbing for flying things
- Behavior systems: wind, day/night cycle, weather (rain/snow/fog/thunder), parallax
- Thunderstorms with lightning bolts and sky flashes
- Bidirectional sky traffic (clouds, birds, planes, helicopters); cloud drift controllable via `--cloud-direction`
- Shared 9-color vehicle palette (planes, helicopters, cars, bird flocks)
- Clouds re-tinted each frame to track the day/night cycle
- Multi-frame car animations (rolling wheels)
- Configurable spawn rates, weather, and day/night speed via CLI
- External art files with user override support (`~/.config/terminart/`)
- Per-character coloring via `.colors` palette or `.colormap` positional grid
- Post-compositing effects (glow, fade)
- Lightweight (~15 FPS, minimal CPU usage)
- Press any key to exit

## Installation

Requires [Rust](https://rustup.rs/) toolchain.

```bash
git clone https://github.com/Lorakszak/Terminart.git
cd Terminart
cargo install --path .
```

Or build manually:

```bash
cargo build --release
cp target/release/terminart ~/.local/bin/
```

## Usage

```bash
# Run default scene (cityscape)
terminart

# Run a specific scene
terminart --scene cityscape

# List available scenes
terminart --list

# Adjust frame rate
terminart --fps 10

# Busier city: more cars, less frequent planes, rainy weather
terminart --car-rate 3 --plane-rate 0.3 --weather rain

# Thunderstorm with clouds drifting from right to left
terminart --weather thunder --cloud-direction left

# Fast-forward the day/night cycle and start at sunrise
terminart --time-speed 2 --start-time 5
```

### All flags

| Flag | Default | Description |
|---|---|---|
| `-s, --scene <SCENE>` | `cityscape` | Scene to display |
| `-l, --list` | | List available scenes |
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

Full invocation with every flag explicit at its default:

```bash
cargo run -- --scene cityscape --fps 15 --cloud-rate 1.0 --plane-rate 1.0 --heli-rate 1.0 --bird-rate 1.0 --car-rate 1.0 --cloud-direction both --weather-intensity 1.0 --time-speed 0.2 --start-time 20.0
```

Press any key to exit.

### Auto-start with terminal

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
terminart
```

## Available Scenes

| Scene | Description |
|-------|-------------|
| `cityscape` | Rooftop view of a city skyline at night |

## Roadmap

### ~~Phase 1: Art loading from external files~~ (done)
Art loaded from `.txt` files in `assets/`, embedded via `include_str!` at compile time. User overrides supported from `~/.config/terminart/scenes/`.

### ~~Phase 2: Color and shading improvements~~ (done)
Color infrastructure: per-character coloring via `.colors` (char-based palette) and `.colormap` (positional grid) files. Color utilities (lerp, tint, fade). Post-compositing buffer effects (radial glow, vertical fade).

### ~~Phase 3: Rich scenes~~ (in progress)
Cityscape scene complete with procedural buildings, parallax skyline, window blinking, clouds, planes, helicopters, birds, cars, and person at table. More scenes planned.

### ~~Phase 4: Dynamic entity behaviors~~ (done)
Wind, parallax scrolling, day/night cycle, weather systems (rain, snow, fog). Art mirroring for direction-aware entities.

### Phase 5: User-defined scenes
Config format (TOML/YAML) for users to define their own scenes without writing Rust. Custom art, entities, behaviors, and effects.

### Phase 6: Screensaver mode
Detect terminal idle and auto-show after configurable timeout. Shell integration via zsh/bash hooks.

### Phase 7: Distribution
Publish to crates.io. Pre-built binaries for Linux/macOS. AUR package.

## License

MIT
