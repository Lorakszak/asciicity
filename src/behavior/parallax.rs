/// How a `Parallax` camera advances horizontally.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanMode {
    /// Ping-pong between `min_x` and `max_x`, flipping direction at each bound.
    PingPong,
    /// Pan continuously to the left, wrapping from `min_x` back to `max_x`.
    Left,
    /// Pan continuously to the right, wrapping from `max_x` back to `min_x`.
    Right,
}

/// Parallax camera that auto-scrolls and provides depth-based offsets.
/// Layers query `offset_x(depth)` where depth 0.0 = far static, 1.0 = full camera speed.
pub struct Parallax {
    /// Camera position in world coordinates
    camera_x: f64,
    camera_y: f64,
    /// Auto-scroll speed (cells per second, always non-negative)
    scroll_speed_x: f64,
    scroll_speed_y: f64,
    /// Scroll bounds (ping-pong between min and max; also used for wrap in one-way modes)
    min_x: f64,
    max_x: f64,
    /// Current scroll direction (1.0 or -1.0) for ping-pong mode
    direction_x: f64,
    /// How the camera advances on x
    mode: PanMode,
}

impl Parallax {
    /// Create a camera with a specific pan mode and starting position.
    /// `start_x` is clamped into `[min_x, max_x]`.
    pub fn with_mode(
        scroll_speed_x: f64,
        min_x: f64,
        max_x: f64,
        mode: PanMode,
        start_x: f64,
    ) -> Self {
        let camera_x = start_x.clamp(min_x, max_x);
        let direction_x = match mode {
            PanMode::PingPong | PanMode::Right => 1.0,
            PanMode::Left => -1.0,
        };
        Self {
            camera_x,
            camera_y: 0.0,
            scroll_speed_x,
            scroll_speed_y: 0.0,
            min_x,
            max_x,
            direction_x,
            mode,
        }
    }

    pub fn tick(&mut self, dt: f64) {
        if self.scroll_speed_x == 0.0 && self.scroll_speed_y == 0.0 {
            return;
        }

        let range = self.max_x - self.min_x;
        match self.mode {
            PanMode::PingPong => {
                self.camera_x += self.scroll_speed_x * self.direction_x * dt;
                if self.camera_x >= self.max_x {
                    self.camera_x = self.max_x;
                    self.direction_x = -1.0;
                } else if self.camera_x <= self.min_x {
                    self.camera_x = self.min_x;
                    self.direction_x = 1.0;
                }
            }
            PanMode::Right => {
                self.camera_x += self.scroll_speed_x * dt;
                if range > 0.0 {
                    while self.camera_x > self.max_x {
                        self.camera_x -= range;
                    }
                }
            }
            PanMode::Left => {
                self.camera_x -= self.scroll_speed_x * dt;
                if range > 0.0 {
                    while self.camera_x < self.min_x {
                        self.camera_x += range;
                    }
                }
            }
        }

        self.camera_y += self.scroll_speed_y * dt;
    }

    /// X offset for a layer at the given depth.
    /// depth 0.0 = static background, 1.0 = moves with full camera speed.
    pub fn offset_x(&self, depth: f64) -> i32 {
        -(self.camera_x * depth) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong_reverses_at_bounds() {
        let mut p = Parallax::with_mode(10.0, 0.0, 5.0, PanMode::PingPong, 0.0);
        // Going right: one tick of 1.0s at speed 10 should clamp to max_x.
        p.tick(1.0);
        assert_eq!(p.offset_x(1.0), -5);
        // Next tick must now be moving left.
        p.tick(1.0);
        assert_eq!(p.offset_x(1.0), 0);
    }

    #[test]
    fn right_mode_wraps_from_max_to_min() {
        let mut p = Parallax::with_mode(10.0, 0.0, 5.0, PanMode::Right, 4.0);
        // 2s at speed 10 = +20, wraps through range=5 back down to 4 + 20 - 5*k
        // 24 -> 19 -> 14 -> 9 -> 4. Expect camera_x == 4.0.
        p.tick(2.0);
        assert_eq!(p.offset_x(1.0), -4);
    }

    #[test]
    fn left_mode_wraps_from_min_to_max() {
        let mut p = Parallax::with_mode(10.0, 0.0, 5.0, PanMode::Left, 1.0);
        // 2s at speed 10 = -20, wraps to 1 - 20 + 5*k = 0 when k = 4 (1 + 0). Expect 0.0.
        // (1 - 20) + 4*5 = 1, so actually 1.0. Let me verify: -19 + 20 = 1. Yes camera_x == 1.0.
        p.tick(2.0);
        assert_eq!(p.offset_x(1.0), -1);
    }

    #[test]
    fn with_mode_respects_start_x() {
        let p = Parallax::with_mode(1.0, 0.0, 10.0, PanMode::PingPong, 7.0);
        assert_eq!(p.offset_x(1.0), -7);
    }

    #[test]
    fn with_mode_clamps_start_x() {
        let p = Parallax::with_mode(1.0, 0.0, 10.0, PanMode::PingPong, 99.0);
        assert_eq!(p.offset_x(1.0), -10);
    }
}
