use rand::Rng;
use rand::rngs::SmallRng;
use ratatui::style::{Color, Style};

use crate::layer::Layer;

#[derive(Clone, Copy, PartialEq)]
pub enum WeatherType {
    Clear,
    Rain,
    Snow,
    Fog,
    Thunder,
}

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    ch: char,
    style: Style,
}

/// An active lightning strike: zigzag segments + age driving flash intensity.
struct Bolt {
    cells: Vec<(u16, u16, char)>,
    age: f64,
}

const BOLT_LIFETIME: f64 = 0.22;
const BOLT_FLASH_PHASE: f64 = 0.08;

/// Weather particle system. Spawns and manages rain, snow, or fog particles.
pub struct Weather {
    weather_type: WeatherType,
    intensity: f64,
    particles: Vec<Particle>,
    spawn_timer: f64,
    spawn_interval: f64,
    strike_timer: f64,
    strike_next: f64,
    bolt: Option<Bolt>,
    /// Y of the horizon/rooftop line. Lightning bolts terminate here and the
    /// thunder flash fades into it. None falls back to full layer height.
    ground_y: Option<u16>,
}

impl Weather {
    pub fn new(weather_type: WeatherType, intensity: f64) -> Self {
        Self {
            weather_type,
            intensity,
            particles: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: Self::calc_interval(weather_type, intensity),
            strike_timer: 0.0,
            strike_next: 3.0,
            bolt: None,
            ground_y: None,
        }
    }

    /// Tell the weather where the horizon line is, so lightning bolts strike
    /// near the rooftops and the thunder flash fades into the skyline instead
    /// of cutting off in mid-air.
    pub fn set_ground_y(&mut self, y: u16) {
        self.ground_y = Some(y);
    }

    pub fn clear() -> Self {
        Self::new(WeatherType::Clear, 0.0)
    }

    fn calc_interval(weather_type: WeatherType, intensity: f64) -> f64 {
        let base = intensity.max(0.1);
        match weather_type {
            WeatherType::Clear => f64::MAX,
            WeatherType::Rain => 0.02 / base,
            WeatherType::Snow => 0.1 / base,
            WeatherType::Fog => 0.3 / base,
            WeatherType::Thunder => 0.015 / base,
        }
    }

    pub fn tick(&mut self, dt: f64, rng: &mut SmallRng, width: u16, height: u16, wind_x: f64) {
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

        if self.weather_type == WeatherType::Thunder {
            self.tick_thunder(dt, rng, width, height);
        }
    }

    fn tick_thunder(&mut self, dt: f64, rng: &mut SmallRng, width: u16, height: u16) {
        if let Some(bolt) = &mut self.bolt {
            bolt.age += dt;
            if bolt.age >= BOLT_LIFETIME {
                self.bolt = None;
            }
        }

        if self.bolt.is_none() {
            self.strike_timer += dt;
            if self.strike_timer >= self.strike_next {
                self.strike_timer = 0.0;
                let base = self.intensity.max(0.1);
                self.strike_next = rng.random_range(2.5..7.0) / base;
                let ground = self.ground_y.unwrap_or(height).min(height);
                self.bolt = Some(Self::generate_bolt(rng, width, ground));
            }
        }
    }

    fn generate_bolt(rng: &mut SmallRng, width: u16, ground_y: u16) -> Bolt {
        let mut cells = Vec::new();
        let margin = (width / 6).max(1) as i32;
        let mut x: i32 = rng.random_range(margin..(width as i32 - margin).max(margin + 1));
        let max_y = (ground_y as i32).max(4);
        for y in 0..max_y {
            let dx: i32 = rng.random_range(-1..=1);
            let ch = match dx {
                d if d < 0 => '/',
                d if d > 0 => '\\',
                _ => '|',
            };
            if x >= 0 && (x as u16) < width {
                cells.push((x as u16, y as u16, ch));
                if rng.random_range(0.0..1.0) < 0.35 && (x as u16 + 1) < width {
                    cells.push(((x + 1) as u16, y as u16, '\\'));
                }
            }
            x += dx;
            if x < 0 {
                x = 0;
            }
            if x >= width as i32 {
                x = width as i32 - 1;
            }
        }
        Bolt { cells, age: 0.0 }
    }

    fn spawn_particle(&mut self, rng: &mut SmallRng, width: u16, height: u16, wind_x: f64) {
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
            WeatherType::Thunder => {
                let ch = RAIN_CHARS[rng.random_range(0..RAIN_CHARS.len())];
                (
                    ch,
                    wind_x * 2.5 + rng.random_range(-0.7..0.7),
                    rng.random_range(18.0..30.0),
                    Style::default().fg(Color::Rgb(120, 140, 180)),
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
            // `tick` already early-returns on Clear before reaching here,
            // but honour the "no panic" convention and bail safely if that
            // invariant ever breaks.
            WeatherType::Clear => return,
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
        if let Some(bolt) = &self.bolt
            && bolt.age < BOLT_FLASH_PHASE
        {
            let t = 1.0 - (bolt.age / BOLT_FLASH_PHASE);
            let ground = self.ground_y.unwrap_or(layer.height).min(layer.height);
            // Fade brightest at the top of the sky, easing out to zero right
            // at the horizon. Anything below the ground line is left alone.
            for y in 0..ground {
                let depth = y as f64 / ground.max(1) as f64;
                let falloff = (1.0 - depth).powf(1.4);
                let lum_f = 200.0 * t * falloff;
                if lum_f < 6.0 {
                    continue;
                }
                let lum = lum_f as u8;
                let bg = Color::Rgb(lum, lum, lum.saturating_add(20));
                let flash_style = Style::default().bg(bg);
                for x in 0..layer.width {
                    layer.set(x, y, ' ', flash_style);
                }
            }
        }

        for p in &self.particles {
            if p.x >= 0.0 && p.y >= 0.0 {
                layer.set(p.x as u16, p.y as u16, p.ch, p.style);
            }
        }

        if let Some(bolt) = &self.bolt {
            let fade = 1.0 - (bolt.age / BOLT_LIFETIME).clamp(0.0, 1.0);
            let lum = (200.0 + 55.0 * fade) as u8;
            let bolt_style = Style::default().fg(Color::Rgb(lum, lum, 150));
            for (x, y, ch) in &bolt.cells {
                layer.set(*x, *y, *ch, bolt_style);
            }
        }
    }

    pub fn weather_type(&self) -> WeatherType {
        self.weather_type
    }
}
