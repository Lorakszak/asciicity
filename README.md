# Bootiful

Terminal ambiance engine - beautiful ASCII art scenes for your terminal.

Bootiful displays procedurally generated, animated ASCII art scenes directly in your terminal. Think of it as a screensaver for your shell - twinkling stars, flickering campfires, rising smoke, all rendered in characters.

## Features

- Procedurally generated scenes (never the same twice)
- Layered rendering with transparent compositing
- Entity system with animated sprites and spawners
- Lightweight (~15 FPS, minimal CPU usage)
- Press any key to exit

## Installation

Requires [Rust](https://rustup.rs/) toolchain.

```bash
git clone https://github.com/Lorakszak/Bootiful.git
cd Bootiful
cargo build --release
```

The binary will be at `target/release/bootiful`. Copy it somewhere on your `$PATH`:

```bash
cp target/release/bootiful ~/.local/bin/
```

## Usage

```bash
# Run default scene
bootiful

# Run a specific scene
bootiful --scene campfire

# List available scenes
bootiful --list

# Adjust frame rate
bootiful --fps 10
```

Press any key to exit.

### Auto-start with terminal

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
bootiful -s campfire
```

## Available Scenes

| Scene | Description |
|-------|-------------|
| `campfire` | A knight resting by a campfire in the wilderness |

More scenes coming soon.

## Roadmap

### Phase 1: Art loading from external files
Load ASCII art from `.txt` or similar files instead of hardcoding in Rust string literals. Makes it easier to create and iterate on complex scene art.

### Phase 2: Color and shading improvements
Richer color palettes, gradient/shading effects, fire glow on nearby objects, smoke fading as it rises, ambient lighting.

### Phase 3: Rich scenes
Make all four scenes visually detailed and alive:
- **Campfire** - Dense forest, waterfall, fireflies, detailed knight, glowing embers
- **Cityscape** - Skyline with blinking lights, moving traffic, clouds, sun/moon cycle
- **Nature landscape** - Mountains, river, birds, clouds, wind in grass, weather
- **Lofi girl** - Girl at desk with headphones, sleeping cat, steaming coffee, city through window, lamp glow

### Phase 4: Dynamic entity behaviors
Wind affecting trees/smoke, parallax scrolling, day/night cycle, weather systems (rain, snow, fog), more interactive and organic animations.

### Phase 5: User-defined scenes
Config format (TOML/YAML) for users to define their own scenes without writing Rust. Custom art, entities, behaviors, and effects.

### Phase 6: Screensaver mode
Detect terminal idle and auto-show after configurable timeout. Shell integration via zsh/bash hooks.

### Phase 7: Distribution
Publish to crates.io. Pre-built binaries for Linux/macOS. AUR package.

## License

MIT
