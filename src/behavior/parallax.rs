/// Parallax camera that auto-scrolls and provides depth-based offsets.
/// Layers query `offset_x(depth)` where depth 0.0 = far static, 1.0 = full camera speed.
pub struct Parallax {
    /// Camera position in world coordinates
    camera_x: f64,
    camera_y: f64,
    /// Auto-scroll speed (cells per second)
    scroll_speed_x: f64,
    scroll_speed_y: f64,
    /// Scroll bounds (ping-pong between min and max)
    min_x: f64,
    max_x: f64,
    /// Current scroll direction (1.0 or -1.0)
    direction_x: f64,
}

impl Parallax {
    /// Create a horizontally scrolling camera.
    /// Ping-pongs between min_x and max_x at the given speed.
    pub fn new(scroll_speed_x: f64, min_x: f64, max_x: f64) -> Self {
        Self {
            camera_x: min_x,
            camera_y: 0.0,
            scroll_speed_x,
            scroll_speed_y: 0.0,
            min_x,
            max_x,
            direction_x: 1.0,
        }
    }

    /// No scrolling (static camera).
    pub fn stationary() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            scroll_speed_x: 0.0,
            scroll_speed_y: 0.0,
            min_x: 0.0,
            max_x: 0.0,
            direction_x: 1.0,
        }
    }

    pub fn tick(&mut self, dt: f64) {
        if self.scroll_speed_x == 0.0 && self.scroll_speed_y == 0.0 {
            return;
        }

        self.camera_x += self.scroll_speed_x * self.direction_x * dt;

        if self.camera_x >= self.max_x {
            self.camera_x = self.max_x;
            self.direction_x = -1.0;
        } else if self.camera_x <= self.min_x {
            self.camera_x = self.min_x;
            self.direction_x = 1.0;
        }

        self.camera_y += self.scroll_speed_y * dt;
    }

    /// X offset for a layer at the given depth.
    /// depth 0.0 = static background, 1.0 = moves with full camera speed.
    pub fn offset_x(&self, depth: f64) -> i32 {
        -(self.camera_x * depth) as i32
    }

    /// Y offset for a layer at the given depth.
    pub fn offset_y(&self, depth: f64) -> i32 {
        -(self.camera_y * depth) as i32
    }

    pub fn camera_x(&self) -> f64 {
        self.camera_x
    }

    pub fn camera_y(&self) -> f64 {
        self.camera_y
    }
}
