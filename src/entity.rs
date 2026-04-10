use ratatui::style::Style;

use crate::color::ColorMap;

pub struct Entity {
    /// Position with sub-cell precision
    pub x: f64,
    pub y: f64,
    /// Velocity in cells per second
    pub vx: f64,
    pub vy: f64,
    /// ASCII art frames (each frame is a multi-line string)
    pub frames: Vec<String>,
    /// Current frame index
    pub frame_idx: usize,
    /// Time between frame advances (seconds)
    pub frame_interval: f64,
    /// Accumulated time since last frame advance
    frame_timer: f64,
    /// Style for rendering
    pub style: Style,
    /// Per-character color overrides (from .colors or .colormap files)
    #[allow(dead_code)]
    pub colors: Option<ColorMap>,
    /// Whether this entity should be removed
    pub alive: bool,
    /// Which layer this entity renders to (0 = background, 3 = overlay)
    pub layer: usize,
    /// Scene-defined type discriminator (0 = untagged).
    pub tag: u32,
    /// Scene-defined per-entity scalar (e.g. cloud brightness variation).
    pub meta: f64,
    /// Sinusoidal vertical bob amplitude in cells (0 = no bobbing).
    pub bob_amp: f64,
    /// Bob frequency in radians/sec.
    pub bob_freq: f64,
    /// Phase offset so a flock doesn't bob in lockstep.
    pub bob_phase: f64,
    /// Accumulated time used as the bob input.
    bob_t: f64,
}

impl Entity {
    pub fn new(
        x: f64,
        y: f64,
        frames: Vec<String>,
        frame_interval: f64,
        style: Style,
        layer: usize,
    ) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            frames,
            frame_idx: 0,
            frame_interval,
            frame_timer: 0.0,
            style,
            colors: None,
            alive: true,
            layer,
            tag: 0,
            meta: 0.0,
            bob_amp: 0.0,
            bob_freq: 0.0,
            bob_phase: 0.0,
            bob_t: 0.0,
        }
    }

    /// Advance animation frame based on elapsed time.
    pub fn tick_animation(&mut self, dt: f64) {
        if self.frames.len() <= 1 {
            return;
        }
        self.frame_timer += dt;
        while self.frame_timer >= self.frame_interval {
            self.frame_timer -= self.frame_interval;
            self.frame_idx = (self.frame_idx + 1) % self.frames.len();
        }
    }

    /// Move by velocity * dt, applying sinusoidal vertical bob if configured.
    /// vy drives base drift; bob adds an oscillation on top without disturbing
    /// the base position, so the bob doesn't accumulate.
    pub fn tick_movement(&mut self, dt: f64) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        if self.bob_amp != 0.0 {
            let prev_bob = self.bob_amp * (self.bob_freq * self.bob_t + self.bob_phase).sin();
            self.bob_t += dt;
            let next_bob = self.bob_amp * (self.bob_freq * self.bob_t + self.bob_phase).sin();
            self.y += next_bob - prev_bob;
        }
    }

    /// Get current ASCII art frame.
    pub fn current_frame(&self) -> &str {
        &self.frames[self.frame_idx]
    }
}
