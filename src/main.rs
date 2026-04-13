mod art;
mod behavior;
mod color;
mod engine;
mod entity;
mod layer;
mod scene;
mod scenes;

use clap::{Parser, ValueEnum};

use crate::scene::{CloudDirection, SceneConfig};

#[derive(Parser)]
#[command(
    name = "asciicity",
    version,
    about = "Animated ASCII cityscape for your terminal"
)]
struct Cli {
    /// Scene to display
    #[arg(short, long, value_enum, default_value_t = SceneName::Cityscape)]
    scene: SceneName,

    /// List available scenes
    #[arg(short, long)]
    list: bool,

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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum SceneName {
    Cityscape,
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
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.list {
        println!("Available scenes:");
        for scene in scenes::SCENES {
            println!("  {:<12} - {}", scene.name, scene.description);
        }
        return Ok(());
    }

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
        weather: cli.weather.map(|w| w.as_str().to_string()),
        weather_intensity: cli.weather_intensity,
        time_speed: cli.time_speed,
        start_time: cli.start_time,
    };

    match cli.scene {
        SceneName::Cityscape => engine::run::<scenes::cityscape::CityscapeScene>(cli.fps, cfg)?,
    }

    Ok(())
}
