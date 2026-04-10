use rand::rngs::SmallRng;
use ratatui::Frame;

/// Direction the wind carries drifting sky entities (clouds).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloudDirection {
    Left,
    Right,
    Both,
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
            weather: None,
            weather_intensity: 1.0,
            time_speed: 0.2,
            start_time: 20.0,
        }
    }
}

/// Scale a base interval by a rate multiplier.
/// rate 0.0 returns `f64::INFINITY` so the caller treats it as "never".
pub fn scale_interval(base: f64, rate: f64) -> f64 {
    if rate <= 0.0 {
        f64::INFINITY
    } else {
        base / rate
    }
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
