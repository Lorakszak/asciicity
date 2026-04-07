# Bootiful - Claude Code Instructions

## Project Overview
Terminal ambiance engine in Rust. Displays procedurally generated ASCII art scenes with animated entities. First scene: campfire (knight, fire, smoke, twinkling stars, trees).

## Tech Stack
- Rust (edition 2024)
- crossterm - terminal control (raw mode, input, resize)
- ratatui - rendering (buffer diffing, styled cells)
- clap - CLI argument parsing (derive macros)
- rand - random number generation

## Architecture
```
CLI (main.rs) -> Engine (engine.rs) -> Scene trait (scene.rs)
                                          |
                                    CampfireScene (scenes/campfire/)

Art pipeline:
  assets/<scene>/*.txt  --include_str!-->  art.rs (defaults)
  ~/.config/bootiful/   --runtime load-->  art.rs (overrides)
       |
       v
  ArtData { frames: Vec<String>, colors: Option<ColorMap> }

Color pipeline:
  .colors file  (char-based palette)  --> ColorMap::Palette
  .colormap file (positional grid)    --> ColorMap::Grid
       |
       v
  draw_ascii_styled() applies per-character fg colors

Rendering:
  Layer compositing (back-to-front) -> Buffer effects (glow, fade)
```

- **Engine** - main loop: poll input, tick scene, render via ratatui, sleep
- **Scene trait** - `setup()`, `tick(dt, rng)`, `render(frame)`, `resize()`
- **Art loader** (`art.rs`) - loads art from embedded defaults or user overrides, returns `ArtData`
- **Layer** - 2D grid of optional styled cells, composited back-to-front; `draw_ascii_styled()` for per-character coloring
- **Entity** - animated object with position, velocity, art frames, optional `ColorMap`, layer assignment
- **Color** (`color.rs`) - `ColorMap` enum (Palette/Grid), color math utilities (lerp, tint, fade), format parsers
- **Effects** (`effects.rs`) - post-compositing buffer modifications (radial glow, vertical fade)

Scenes own their layers (background, midground, foreground, overlay), entities, and spawners.

## Building and Running
```bash
cargo build           # build
cargo run             # run default scene
cargo run -- --list   # list scenes
cargo run -- -s campfire --fps 15
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
- User overrides go in `~/.config/bootiful/scenes/<scene>/` (same filenames)
- Scratch layers are pre-allocated and reused via `.clear()`, never allocated per-frame

## Adding a New Scene
1. Create art files in `assets/<name>/` (`.txt`, optional `.colors`/`.colormap`)
2. Create `src/scenes/<name>/art.rs` with `include_str!` defaults
3. Create `src/scenes/<name>/mod.rs` implementing `Scene` trait
4. Add `pub mod <name>;` to `src/scenes/mod.rs`
5. Add `SceneInfo` entry to `SCENES` array in `src/scenes/mod.rs`
6. Add variant to `SceneName` enum in `main.rs`
7. Add match arm in `main.rs`
