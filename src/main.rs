mod art;
mod behavior;
mod cityscape;
mod color;
mod engine;
mod entity;
mod layer;
mod scene;

use clap::{Parser, ValueEnum};

use crate::scene::{CloudDirection, PanDir, SceneConfig};

#[derive(Parser)]
#[command(
    name = "asciicity",
    version,
    about = "Animated ASCII cityscape for your terminal"
)]
struct Cli {
    /// Target frames per second
    #[arg(long, default_value_t = 15)]
    fps: u32,

    /// Cloud spawn rate multiplier (0 = none, 1 = default, 2 = twice as often)
    #[arg(long, default_value_t = 1.0)]
    cloud_rate: f64,

    /// Plane spawn rate multiplier
    #[arg(long, default_value_t = 1.0)]
    plane_rate: f64,

    /// Helicopter spawn rate multiplier
    #[arg(long, default_value_t = 1.0)]
    heli_rate: f64,

    /// Bird flock spawn rate multiplier
    #[arg(long, default_value_t = 1.0)]
    bird_rate: f64,

    /// Car spawn rate multiplier
    #[arg(long, default_value_t = 1.0)]
    car_rate: f64,

    /// Direction clouds drift across the sky
    #[arg(long, value_enum, default_value_t = CloudDirectionArg::Both)]
    cloud_direction: CloudDirectionArg,

    /// Pan direction for the far (background) skyline layer
    #[arg(long, value_enum, default_value_t = PanDirArg::Auto)]
    far_pan: PanDirArg,

    /// Pan direction for the near (mid) skyline layer
    #[arg(long, value_enum, default_value_t = PanDirArg::Auto)]
    near_pan: PanDirArg,

    /// Weather override
    #[arg(long, value_enum)]
    weather: Option<WeatherArg>,

    /// Weather intensity (0.1..3.0)
    #[arg(long, default_value_t = 1.0)]
    weather_intensity: f64,

    /// Day/night speed in in-game hours per real second (default 0.2 = ~2 minutes per day)
    #[arg(long, default_value_t = 0.2)]
    time_speed: f64,

    /// Starting hour of day (0.0..24.0)
    #[arg(long, default_value_t = 20.0)]
    start_time: f64,

    /// Pre-simulate the scene for N seconds so it starts already populated
    /// (0 = disabled, default; ~5 fills cars and clouds quickly, larger
    /// values fill rarer entities like planes and birds)
    #[arg(long, default_value_t = 0.0)]
    warmup: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum WeatherArg {
    Clear,
    Rain,
    Snow,
    Fog,
    Thunder,
}

impl WeatherArg {
    fn as_str(self) -> &'static str {
        match self {
            WeatherArg::Clear => "clear",
            WeatherArg::Rain => "rain",
            WeatherArg::Snow => "snow",
            WeatherArg::Fog => "fog",
            WeatherArg::Thunder => "thunder",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum CloudDirectionArg {
    Left,
    Right,
    Both,
}

impl CloudDirectionArg {
    fn to_cfg(self) -> CloudDirection {
        match self {
            CloudDirectionArg::Left => CloudDirection::Left,
            CloudDirectionArg::Right => CloudDirection::Right,
            CloudDirectionArg::Both => CloudDirection::Both,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum PanDirArg {
    Auto,
    Left,
    Right,
}

impl PanDirArg {
    fn to_cfg(self) -> PanDir {
        match self {
            PanDirArg::Auto => PanDir::Auto,
            PanDirArg::Left => PanDir::Left,
            PanDirArg::Right => PanDir::Right,
        }
    }
}

/// Reject non-finite values and values outside `[min, max]`. Gives the user a
/// clear error instead of silent NaN propagation or a `Duration` panic.
fn check_range(name: &str, value: f64, min: f64, max: f64) -> Result<f64, String> {
    if !value.is_finite() {
        return Err(format!("--{name} must be a finite number, got {value}"));
    }
    if value < min || value > max {
        return Err(format!("--{name} must be in [{min}, {max}], got {value}"));
    }
    Ok(value)
}

fn validate(cli: &Cli) -> Result<(), String> {
    if cli.fps == 0 {
        return Err("--fps must be at least 1".into());
    }
    if cli.fps > 240 {
        return Err(format!("--fps must be <= 240, got {}", cli.fps));
    }
    check_range("cloud-rate", cli.cloud_rate, 0.0, 100.0)?;
    check_range("plane-rate", cli.plane_rate, 0.0, 100.0)?;
    check_range("heli-rate", cli.heli_rate, 0.0, 100.0)?;
    check_range("bird-rate", cli.bird_rate, 0.0, 100.0)?;
    check_range("car-rate", cli.car_rate, 0.0, 100.0)?;
    check_range("weather-intensity", cli.weather_intensity, 0.0, 10.0)?;
    check_range("time-speed", cli.time_speed, 0.0, 1000.0)?;
    // start-time is modulo 24 at use, but reject obviously wrong values upfront.
    check_range("start-time", cli.start_time, 0.0, 24.0)?;
    check_range("warmup", cli.warmup, 0.0, 600.0)?;
    Ok(())
}

/// Shift the user's `start_time` backward by however many in-game hours the
/// warmup will tick through, so the day/night clock lands exactly on the
/// requested hour at the first rendered frame. Wraps modulo 24.
fn compensated_start(start_time: f64, warmup: f64, time_speed: f64) -> f64 {
    let shifted = start_time - warmup * time_speed;
    shifted.rem_euclid(24.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Err(msg) = validate(&cli) {
        eprintln!("error: {msg}");
        std::process::exit(2);
    }

    let cfg = SceneConfig {
        cloud_rate: cli.cloud_rate,
        plane_rate: cli.plane_rate,
        heli_rate: cli.heli_rate,
        bird_rate: cli.bird_rate,
        car_rate: cli.car_rate,
        cloud_direction: cli.cloud_direction.to_cfg(),
        far_pan: cli.far_pan.to_cfg(),
        near_pan: cli.near_pan.to_cfg(),
        weather: cli.weather.map(|w| w.as_str().to_string()),
        weather_intensity: cli.weather_intensity,
        time_speed: cli.time_speed,
        start_time: compensated_start(cli.start_time, cli.warmup, cli.time_speed),
        warmup_seconds: cli.warmup,
    };

    engine::run::<cityscape::CityscapeScene>(cli.fps, cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compensated_start_wraps_negative_into_day() {
        // 0:00 minus 6 in-game hours wraps to 18:00.
        assert!((compensated_start(0.0, 30.0, 0.2) - 18.0).abs() < 1e-9);
    }

    #[test]
    fn compensated_start_lands_at_user_intent() {
        // 30s warmup at 0.2 hr/s shifts 6h back from 20:00 -> 14:00. After
        // the warmup ticks 6h forward the day/night clock is at 20:00.
        assert!((compensated_start(20.0, 30.0, 0.2) - 14.0).abs() < 1e-9);
    }

    #[test]
    fn compensated_start_no_warmup_is_identity() {
        assert!((compensated_start(7.5, 0.0, 0.2) - 7.5).abs() < 1e-9);
    }

    #[test]
    fn compensated_start_zero_speed_is_identity() {
        // Frozen day/night: warmup advances no in-game time, no shift.
        assert!((compensated_start(7.5, 60.0, 0.0) - 7.5).abs() < 1e-9);
    }

    #[test]
    fn compensated_start_handles_multiple_day_wrap() {
        // 600s * 1.0 hr/s = 600 in-game hours = 25 days. From 12:00 should
        // wrap back to 12:00 since 600 % 24 = 0.
        assert!((compensated_start(12.0, 600.0, 1.0) - 12.0).abs() < 1e-9);
    }
}
