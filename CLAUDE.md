# Terminart - Claude Code Instructions

## Project Overview
Terminal ambiance engine in Rust. Displays procedurally generated ASCII art scenes with animated entities. Current scene: cityscape (rooftop city skyline with buildings, traffic, planes, weather).

## Tech Stack
- Rust (edition 2024)
- crossterm - terminal control (raw mode, input, resize)
- ratatui - rendering (buffer diffing, styled cells)
- clap - CLI argument parsing (derive macros)
- rand - random number generation

## Architecture
```
CLI (main.rs) -> SceneConfig -> Engine (engine.rs) -> Scene trait (scene.rs)
                                                          |
                                                    CityscapeScene (scenes/cityscape/)

Art pipeline:
  assets/<scene>/*.txt  --include_str!-->  art.rs (defaults)
  ~/.config/terminart/   --runtime load-->  art.rs (overrides)
       |
       v
  ArtData { frames: Vec<String>, colors: Option<ColorMap> }

Color pipeline:
  .colors file  (char-based palette)  --> ColorMap::Palette
  .colormap file (positional grid)    --> ColorMap::Grid
       |
       v
  draw_ascii_styled() applies per-character fg colors

Behavior systems:
  Wind     - smooth gusting, affects entity drift
  DayNight - sky color keyframes, ambient light, star visibility
  Parallax - camera scroll with per-layer depth offsets
  Weather  - rain/snow/fog particle spawning

Rendering:
  Layer compositing (back-to-front, with parallax offsets) -> Buffer effects (glow, fade)
```

- **Engine** - main loop: poll input, tick scene, render via ratatui, sleep. Takes a `SceneConfig` and passes it to `Scene::setup`.
- **Scene trait** - `setup(width, height, cfg, rng)`, `tick(dt, rng)`, `render(frame)`, `resize()`. `SceneConfig` carries CLI spawn-rate multipliers, weather override, and day/night time settings. Scenes store their config so `resize()` can re-apply it.
- **Art loader** (`art.rs`) - loads art from embedded defaults or user overrides, returns `ArtData`. `mirror_horizontal()` flips art for direction-aware entities.
- **Layer** - 2D grid of optional styled cells, composited back-to-front. `composite_offset()` iterates the full layer dimensions (not screen-clamped) so wide parallax layers draw their off-screen content when panned into view.
- **Entity** - position/velocity/frames/style + `tag: u32` (scene-defined type discriminator), `meta: f64` (per-entity scalar, e.g. cloud brightness bias), and `bob_amp/freq/phase` for sinusoidal vertical motion on top of `vy` drift.
- **Color** (`color.rs`) - `ColorMap` enum (Palette/Grid), color math utilities (lerp, tint, fade), format parsers
- **Effects** (`effects.rs`) - post-compositing buffer modifications (radial glow, vertical fade)
- **Behaviors** (`behavior/`) - wind, day/night, parallax, weather systems. Scenes opt-in by embedding and ticking them. `Weather` supports `Rain`, `Snow`, `Fog`, and `Thunder` (rain particles + periodic lightning bolts with sky flash).

Scenes own their layers, entities, spawners, behavior system instances, and a cloned `SceneConfig`.

## Building and Running
```bash
cargo build                                            # build
cargo run                                              # run default scene (cityscape)
cargo run -- --list                                    # list scenes
cargo run -- -s cityscape --fps 15                     # pick scene + fps
cargo run -- --car-rate 3 --weather rain               # busier cars + rain
cargo run -- --weather thunder                         # thunderstorm with lightning
cargo run -- --cloud-direction left                    # clouds drift right-to-left only
cargo run -- --time-speed 2 --start-time 5             # fast day/night starting at sunrise
cargo install --path .                                 # install system-wide
```

Full invocation with every flag explicit at its default:
```bash
cargo run -- --scene cityscape --fps 15 --cloud-rate 1.0 --plane-rate 1.0 --heli-rate 1.0 --bird-rate 1.0 --car-rate 1.0 --cloud-direction both --weather-intensity 1.0 --time-speed 0.2 --start-time 20.0
```

Press any key to exit.

## Conventions
- Each scene lives in `src/scenes/<name>/` with `mod.rs` + `art.rs`
- New scenes implement the `Scene` trait
- Entity frames use `Vec<String>`, not `&'static str` (no Box::leak)
- Art lives in `assets/<scene>/*.txt`, loaded via `include_str!` in scene `art.rs`
- Multi-frame animations use `---` line separator in `.txt` files
- Optional `.colors` files map characters to hex colors (e.g. `O #FFD700`)
- Optional `.colormap` files provide positional color grids (`@palette` + `@map` sections)
- User overrides go in `~/.config/terminart/scenes/<scene>/` (same filenames)
- Scratch layers are pre-allocated and reused via `.clear()`, never allocated per-frame
- `art::mirror_horizontal()` flips art left/right for entities traveling the opposite direction
- ASCII art reference sites listed in `docs/ascii-art-resources.md` (gitignored, local only)
- Parallax layers must have enough extra width for at least 100px of max shift on the nearest parallax layer. Far layers scale proportionally by depth ratio. Use large PARALLAX_RANGE (~200) for noticeable drift.
- Building colors must use `lerp_rgb` for smooth day/night transitions, never binary if/else
- Entity `tag`/`meta` are the generic way to discriminate and parameterize entities. Use them instead of stuffing state into `frame_interval` or cloning sibling Vecs.
- Flying entities (planes, helis, birds) should set `bob_amp/freq/phase` so they don't travel in flat lines.
- Vehicle-like entities share the 9-color palette in `cityscape/mod.rs::VEHICLE_PALETTE` via `pick_vehicle_color`.
- Direction-aware entities (clouds, birds, planes, helis, cars) must spawn both directions when their config allows it: random `going_right`, mirror art via `art::mirror_frames` if the source faces the wrong way, and flip the sign of `vx`.
- New scenes that take runtime options should read them from the `&SceneConfig` passed to `setup()`, clone it into the scene struct, and use `scene::scale_interval` when computing spawn delays so `--*-rate 0` disables that entity cleanly.

## Adding a New Scene
1. Create art files in `assets/<name>/` (`.txt`, optional `.colors`/`.colormap`)
2. Create `src/scenes/<name>/art.rs` with `include_str!` defaults
3. Create `src/scenes/<name>/mod.rs` implementing `Scene` trait
4. Add `pub mod <name>;` to `src/scenes/mod.rs`
5. Add `SceneInfo` entry to `SCENES` array in `src/scenes/mod.rs`
6. Add variant to `SceneName` enum in `main.rs`
7. Add match arm in `main.rs`
