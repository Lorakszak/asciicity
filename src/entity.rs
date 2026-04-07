use ratatui::style::Style;

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
    /// Whether this entity should be removed
    pub alive: bool,
    /// Which layer this entity renders to (0 = background, 3 = overlay)
    pub layer: usize,
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
            alive: true,
            layer,
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

    /// Move by velocity * dt.
    pub fn tick_movement(&mut self, dt: f64) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

    /// Get current ASCII art frame.
    pub fn current_frame(&self) -> &str {
        &self.frames[self.frame_idx]
    }
}
