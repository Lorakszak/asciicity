use rand::rngs::SmallRng;
use rand::Rng;
use ratatui::style::{Color, Style};

use crate::layer::Layer;

#[derive(Clone, Copy, PartialEq)]
pub enum WeatherType {
    Clear,
    Rain,
    Snow,
    Fog,
}

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    ch: char,
    style: Style,
}

/// Weather particle system. Spawns and manages rain, snow, or fog particles.
pub struct Weather {
    weather_type: WeatherType,
    intensity: f64,
    particles: Vec<Particle>,
    spawn_timer: f64,
    spawn_interval: f64,
}

impl Weather {
    pub fn new(weather_type: WeatherType, intensity: f64) -> Self {
        Self {
            weather_type,
            intensity,
            particles: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: Self::calc_interval(weather_type, intensity),
        }
    }

    pub fn clear() -> Self {
        Self::new(WeatherType::Clear, 0.0)
    }

    pub fn set_type(&mut self, weather_type: WeatherType, intensity: f64) {
        self.weather_type = weather_type;
        self.intensity = intensity;
        self.spawn_interval = Self::calc_interval(weather_type, intensity);
        if weather_type == WeatherType::Clear {
            self.particles.clear();
        }
    }

    fn calc_interval(weather_type: WeatherType, intensity: f64) -> f64 {
        let base = intensity.max(0.1);
        match weather_type {
            WeatherType::Clear => f64::MAX,
            WeatherType::Rain => 0.02 / base,
            WeatherType::Snow => 0.1 / base,
            WeatherType::Fog => 0.3 / base,
        }
    }

    pub fn tick(
        &mut self,
        dt: f64,
        rng: &mut SmallRng,
        width: u16,
        height: u16,
        wind_x: f64,
    ) {
        if self.weather_type == WeatherType::Clear {
            return;
        }

        self.spawn_timer += dt;
        while self.spawn_timer >= self.spawn_interval {
            self.spawn_timer -= self.spawn_interval;
            self.spawn_particle(rng, width, height, wind_x);
        }

        for p in &mut self.particles {
            p.x += (p.vx + wind_x * 0.5) * dt;
            p.y += p.vy * dt;
        }

        let w = width as f64;
        let h = height as f64;
        self.particles
            .retain(|p| p.y < h && p.x >= -1.0 && p.x < w + 1.0);
    }

    fn spawn_particle(
        &mut self,
        rng: &mut SmallRng,
        width: u16,
        height: u16,
        wind_x: f64,
    ) {
        const RAIN_CHARS: &[char] = &['|', '/', '\\', ':'];
        const SNOW_CHARS: &[char] = &['*', '.', '+', 'o'];
        const FOG_CHARS: &[char] = &['~', '-', '.'];

        let (ch, vx, vy, style, x, y) = match self.weather_type {
            WeatherType::Rain => {
                let ch = RAIN_CHARS[rng.random_range(0..RAIN_CHARS.len())];
                (
                    ch,
                    wind_x * 2.0 + rng.random_range(-0.5..0.5),
                    rng.random_range(15.0..25.0),
                    Style::default().fg(Color::Rgb(100, 130, 180)),
                    rng.random_range(0.0..width as f64),
                    -1.0,
                )
            }
            WeatherType::Snow => {
                let ch = SNOW_CHARS[rng.random_range(0..SNOW_CHARS.len())];
                (
                    ch,
                    wind_x + rng.random_range(-1.0..1.0),
                    rng.random_range(1.0..4.0),
                    Style::default().fg(Color::Rgb(200, 210, 220)),
                    rng.random_range(0.0..width as f64),
                    -1.0,
                )
            }
            WeatherType::Fog => {
                let ch = FOG_CHARS[rng.random_range(0..FOG_CHARS.len())];
                (
                    ch,
                    wind_x * 0.5 + rng.random_range(-0.3..0.3),
                    rng.random_range(-0.2..0.2),
                    Style::default().fg(Color::Rgb(120, 120, 130)),
                    rng.random_range(0.0..width as f64),
                    rng.random_range(0.0..height as f64),
                )
            }
            WeatherType::Clear => unreachable!(),
        };

        self.particles.push(Particle {
            x,
            y,
            vx,
            vy,
            ch,
            style,
        });
    }

    /// Render weather particles onto a layer.
    pub fn render(&self, layer: &mut Layer) {
        for p in &self.particles {
            if p.x >= 0.0 && p.y >= 0.0 {
                layer.set(p.x as u16, p.y as u16, p.ch, p.style);
            }
        }
    }

    pub fn weather_type(&self) -> WeatherType {
        self.weather_type
    }

    pub fn intensity(&self) -> f64 {
        self.intensity
    }
}
