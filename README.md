# Bootiful

Terminal ambiance engine - beautiful ASCII art scenes for your terminal.

Bootiful displays procedurally generated, animated ASCII art scenes directly in your terminal. Think of it as a screensaver for your shell - city skylines with blinking windows, drifting clouds, planes, and cars.

## Features

- Procedurally generated scenes (never the same twice)
- Layered rendering with transparent compositing and parallax scrolling
- Entity system with animated sprites, spawners, and auto-mirroring
- Behavior systems: wind, day/night cycle, weather, parallax
- External art files with user override support (`~/.config/bootiful/`)
- Per-character coloring via `.colors` palette or `.colormap` positional grid
- Post-compositing effects (glow, fade)
- Smooth day/night color transitions
- Lightweight (~15 FPS, minimal CPU usage)
- Press any key to exit

## Installation

Requires [Rust](https://rustup.rs/) toolchain.

```bash
git clone https://github.com/Lorakszak/Bootiful.git
cd Bootiful
cargo install --path .
```

Or build manually:

```bash
cargo build --release
cp target/release/bootiful ~/.local/bin/
```

## Usage

```bash
# Run default scene (cityscape)
bootiful

# Run a specific scene
bootiful --scene cityscape

# List available scenes
bootiful --list

# Adjust frame rate
bootiful --fps 10
```

Press any key to exit.

### Auto-start with terminal

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
bootiful
```

## Available Scenes

| Scene | Description |
|-------|-------------|
| `cityscape` | Rooftop view of a city skyline at night |

## Roadmap

### ~~Phase 1: Art loading from external files~~ (done)
Art loaded from `.txt` files in `assets/`, embedded via `include_str!` at compile time. User overrides supported from `~/.config/bootiful/scenes/`.

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
