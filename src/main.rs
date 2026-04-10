mod art;
#[allow(dead_code)]
mod behavior;
#[allow(dead_code)]
mod color;
#[allow(dead_code)]
mod effects;
mod engine;
mod entity;
mod layer;
mod scene;
mod scenes;

use clap::{Parser, ValueEnum};

use crate::scene::SceneConfig;

#[derive(Parser)]
#[command(
    name = "terminart",
    version,
    about = "Terminal ambiance engine - beautiful ASCII art scenes"
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.list {
        println!("Available scenes:");
        for scene in scenes::SCENES {
            println!("  {:<12} - {}", scene.name, scene.description);
        }
        return Ok(());
    }

    let cfg = SceneConfig {
        cloud_rate: cli.cloud_rate,
        plane_rate: cli.plane_rate,
        heli_rate: cli.heli_rate,
        bird_rate: cli.bird_rate,
        car_rate: cli.car_rate,
        weather: cli.weather.map(|w| w.as_str().to_string()),
        weather_intensity: cli.weather_intensity,
        time_speed: cli.time_speed,
        start_time: cli.start_time,
    };

    match cli.scene {
        SceneName::Cityscape => {
            engine::run::<scenes::cityscape::CityscapeScene>(cli.fps, cfg)?
        }
    }

    Ok(())
}
