mod engine;
mod entity;
mod layer;
mod scene;
mod scenes;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    name = "bootiful",
    version,
    about = "Terminal ambiance engine - beautiful ASCII art scenes"
)]
struct Cli {
    /// Scene to display
    #[arg(short, long, value_enum, default_value_t = SceneName::Campfire)]
    scene: SceneName,

    /// List available scenes
    #[arg(short, long)]
    list: bool,

    /// Target frames per second
    #[arg(long, default_value_t = 15)]
    fps: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum SceneName {
    Campfire,
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

    match cli.scene {
        SceneName::Campfire => engine::run::<scenes::campfire::CampfireScene>(cli.fps)?,
    }

    Ok(())
}
