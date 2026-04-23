use rand::rngs::SmallRng;
use ratatui::Frame;

/// Direction the wind carries drifting sky entities (clouds).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloudDirection {
    Left,
    Right,
    Both,
}

/// Per-layer pan direction requested by the user.
///
/// `Auto` lets the scene pick a default (today: ping-pong with a phase offset
/// per layer so the layers don't move in lockstep). `Left`/`Right` force the
/// layer to scroll continuously in that direction, wrapping at the bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanDir {
    Auto,
    Left,
    Right,
}

/// Runtime configuration passed to every scene from the CLI.
///
/// Rate multipliers scale spawn intervals inversely: 2.0 = twice as frequent,
/// 0.5 = half as frequent, 0.0 = disabled. Scenes are free to ignore fields
/// that don't apply.
#[derive(Clone)]
pub struct SceneConfig {
    pub cloud_rate: f64,
    pub plane_rate: f64,
    pub heli_rate: f64,
    pub bird_rate: f64,
    pub car_rate: f64,
    pub cloud_direction: CloudDirection,
    /// Pan direction for the far (background) skyline layer.
    pub far_pan: PanDir,
    /// Pan direction for the near (mid) skyline layer.
    pub near_pan: PanDir,
    /// Weather override: "clear", "rain", "snow", "fog", "thunder". None leaves the scene default.
    pub weather: Option<String>,
    pub weather_intensity: f64,
    /// In-game hours per real second for the day/night cycle.
    pub time_speed: f64,
    /// Starting hour (0.0..24.0).
    pub start_time: f64,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            cloud_rate: 1.0,
            plane_rate: 1.0,
            heli_rate: 1.0,
            bird_rate: 1.0,
            car_rate: 1.0,
            cloud_direction: CloudDirection::Both,
            far_pan: PanDir::Auto,
            near_pan: PanDir::Auto,
            weather: None,
            weather_intensity: 1.0,
            time_speed: 0.2,
            start_time: 20.0,
        }
    }
}

/// Scale a base interval by a rate multiplier.
///
/// Returns `f64::INFINITY` (meaning "never spawn") if `rate` is zero,
/// negative, or non-finite, and if `base` is not finite. This keeps a bad
/// config from silently disabling a spawner by producing NaN comparisons
/// that are always false.
pub fn scale_interval(base: f64, rate: f64) -> f64 {
    if !base.is_finite() || !rate.is_finite() || rate <= 0.0 {
        return f64::INFINITY;
    }
    base / rate
}

pub trait Scene {
    /// Create the scene for a given terminal size.
    fn setup(width: u16, height: u16, cfg: &SceneConfig, rng: &mut SmallRng) -> Self
    where
        Self: Sized;

    /// Advance the scene by dt seconds.
    fn tick(&mut self, dt: f64, rng: &mut SmallRng);

    /// Draw the scene to the frame.
    fn render(&mut self, frame: &mut Frame);

    /// Handle terminal resize.
    fn resize(&mut self, width: u16, height: u16, rng: &mut SmallRng);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_interval_rejects_non_finite() {
        assert_eq!(scale_interval(1.0, f64::NAN), f64::INFINITY);
        assert_eq!(scale_interval(1.0, f64::INFINITY), f64::INFINITY);
        assert_eq!(scale_interval(f64::NAN, 1.0), f64::INFINITY);
        assert_eq!(scale_interval(1.0, -1.0), f64::INFINITY);
        assert_eq!(scale_interval(1.0, 0.0), f64::INFINITY);
    }

    #[test]
    fn scale_interval_scales_normally() {
        assert!((scale_interval(10.0, 2.0) - 5.0).abs() < 1e-9);
        assert!((scale_interval(10.0, 0.5) - 20.0).abs() < 1e-9);
    }
}
