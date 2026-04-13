# asciicity - Claude Code Instructions

## Project Overview
Animated ASCII cityscape for the terminal, written in Rust. Renders a procedurally animated rooftop view of a city skyline with buildings, traffic, planes, helicopters, birds, clouds, weather, and a day/night cycle. Single-purpose tool.

## Tech Stack
- Rust (edition 2024)
- crossterm - terminal control (raw mode, input, resize)
- ratatui - rendering (buffer diffing, styled cells)
- clap - CLI argument parsing (derive macros)
- rand - random number generation

## Architecture
```
CLI (main.rs) -> SceneConfig -> Engine (engine.rs) -> CityscapeScene (cityscape/)

Art pipeline:
  assets/*.txt           --include_str!-->  cityscape/art.rs (defaults)
  ~/.config/asciicity/   --runtime load-->  art.rs (overrides)
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
  Layer compositing (back-to-front, with parallax offsets)
```

- **Engine** - main loop: poll input, tick scene, render via ratatui, sleep. Takes a `SceneConfig` and passes it to `Scene::setup`. The `Scene` trait is the engine/scene boundary, not a multi-scene hook: there is one scene, `CityscapeScene`.
- **Scene trait** - `setup(width, height, cfg, rng)`, `tick(dt, rng)`, `render(frame)`, `resize()`. `SceneConfig` carries CLI spawn-rate multipliers, weather override, and day/night time settings. The scene stores its config so `resize()` can re-apply it.
- **Art loader** (`art.rs`) - loads art from embedded defaults or user overrides in `~/.config/asciicity/`, returns `ArtData`. `mirror_horizontal()` flips art for direction-aware entities.
- **Layer** - 2D grid of optional styled cells, composited back-to-front. `composite_offset()` iterates the full layer dimensions (not screen-clamped) so wide parallax layers draw their off-screen content when panned into view.
- **Entity** - position/velocity/frames/style + `tag: u32` (type discriminator, e.g. cloud/plane/heli/bird/car), `meta: f64` (per-entity scalar, e.g. cloud brightness bias), and `bob_amp/freq/phase` for sinusoidal vertical motion on top of `vy` drift.
- **Color** (`color.rs`) - `ColorMap` enum (Palette/Grid), color math utilities (`lerp_rgb`, `fade_rgb`), hex/palette/colormap parsers.
- **Behaviors** (`behavior/`) - wind, day/night, parallax, weather systems. The cityscape embeds and ticks them. `Weather` supports `Rain`, `Snow`, `Fog`, and `Thunder` (rain particles + periodic lightning bolts with sky flash).

`CityscapeScene` owns its layers, entities, spawners, behavior system instances, and a cloned `SceneConfig`.

## Building and Running
```bash
cargo build                                            # build
cargo run                                              # run the cityscape
cargo run -- --fps 15                                  # adjust frame rate
cargo run -- --car-rate 3 --weather rain               # busier cars + rain
cargo run -- --weather thunder                         # thunderstorm with lightning
cargo run -- --cloud-direction left                    # clouds drift right-to-left only
cargo run -- --time-speed 2 --start-time 5             # fast day/night starting at sunrise
cargo install --path .                                 # install system-wide
```

Full invocation with every flag explicit at its default:
```bash
cargo run -- --fps 15 --cloud-rate 1.0 --plane-rate 1.0 --heli-rate 1.0 --bird-rate 1.0 --car-rate 1.0 --cloud-direction both --weather-intensity 1.0 --time-speed 0.2 --start-time 20.0
```

Press any key to exit.

## Quality Gates
Run these while iterating and before declaring any change done:
```bash
cargo check                     # fast type-check during iteration
cargo clippy -- -D warnings     # lint, treat warnings as errors
cargo fmt --check               # formatting check (use `cargo fmt` to apply)
cargo test                      # run tests
cargo build --release           # final build sanity-check
```

Pre-commit checklist (do not skip, do not `--no-verify`):
- `cargo fmt` applied
- `cargo clippy -- -D warnings` clean
- `cargo test` passes
- No stray `dbg!`, debug `println!`, or commented-out code
- No new `unwrap()` or `panic!`/`todo!`/`unimplemented!` in merged code paths

When iterating on a compiler/borrow-checker error, paste the full `rustc` output including the `E0xxx` code into the conversation rather than paraphrasing. Do not silence clippy warnings with `#[allow(...)]` unless the suppression is justified in a comment.

## Rust Conventions
- **No `unwrap()` in production paths.** Use `?` to propagate, or `.expect("why this invariant holds")` only when an invariant genuinely cannot fail. Same rule for `panic!`/`todo!`/`unimplemented!`.
- **No `unsafe`** without a `// SAFETY:` comment spelling out the invariants the caller must uphold.
- **Prefer borrowing** (`&T`, `&mut T`) over owning when the function does not need ownership. Call `.clone()` explicitly and only when actually necessary.
- **Prefer iterators and combinators** (`map`/`filter`/`fold`/`enumerate`) over manual index loops.
- **Prefer `if let` / `while let`** for single-pattern matches instead of full `match` blocks.
- **No wildcard imports** (`use foo::*`) except `use super::*;` inside `#[cfg(test)]` modules.
- **Naming:** `snake_case` for fns/vars/modules, `PascalCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts. Rustfmt defaults (100-col) are authoritative, do not hand-reformat against them.
- **Scope fixes narrowly.** When fixing a bug or a clippy warning, do not drive-by refactor unrelated code in the same change.

## Conventions
- The cityscape implementation lives in `src/cityscape/` as `mod.rs` + `art.rs`.
- `CityscapeScene` implements the `Scene` trait (in `src/scene.rs`). The trait is the engine/scene boundary, not a multi-scene hook.
- Entity frames use `Vec<String>`, not `&'static str` (no Box::leak).
- Art lives in `assets/*.txt`, loaded via `include_str!` in `src/cityscape/art.rs`.
- Multi-frame animations use `---` line separator in `.txt` files.
- Optional `.colors` files map characters to hex colors (e.g. `O #FFD700`).
- Optional `.colormap` files provide positional color grids (`@palette` + `@map` sections).
- User overrides go in `~/.config/asciicity/` (same filenames as under `assets/`).
- Scratch layers are pre-allocated and reused via `.clear()`, never allocated per-frame.
- `art::mirror_horizontal()` flips art left/right for entities traveling the opposite direction.
- ASCII art reference sites listed in `docs/ascii-art-resources.md` (gitignored, local only).
- Parallax layers must have enough extra width for at least 100px of max shift on the nearest parallax layer. Far layers scale proportionally by depth ratio. Use large PARALLAX_RANGE (~200) for noticeable drift.
- Building colors must use `lerp_rgb` for smooth day/night transitions, never binary if/else.
- Entity `tag`/`meta` are the generic way to discriminate and parameterize entities. Use them instead of stuffing state into `frame_interval` or cloning sibling Vecs.
- Flying entities (planes, helis, birds) should set `bob_amp/freq/phase` so they don't travel in flat lines.
- Vehicle-like entities share the 9-color palette in `cityscape/mod.rs::VEHICLE_PALETTE` via `pick_vehicle_color`.
- Direction-aware entities (clouds, birds, planes, helis, cars) must spawn both directions when their config allows it: random `going_right`, mirror art via `art::mirror_frames` if the source faces the wrong way, and flip the sign of `vx`.
- Runtime options are read from the `&SceneConfig` passed to `setup()`, cloned into the scene struct, and used via `scene::scale_interval` when computing spawn delays so `--*-rate 0` disables that entity cleanly.
