mod art;

use rand::rngs::SmallRng;
use rand::Rng;
use ratatui::style::{Color, Style};
use ratatui::Frame;

use crate::entity::Entity;
use crate::layer::Layer;
use crate::scene::Scene;

const LAYER_COUNT: usize = 4;

/// Background: 0, Midground: 1, Foreground: 2, Overlay: 3
const BG: usize = 0;
const MID: usize = 1;
const FG: usize = 2;
const OVERLAY: usize = 3;

struct Star {
    x: u16,
    y: u16,
    char_idx: usize,
    visible: bool,
    timer: f64,
    interval: f64,
}

pub struct CampfireScene {
    width: u16,
    height: u16,
    layers: Vec<Layer>,
    scratch_layer: Layer,
    stars: Vec<Star>,
    ground_y: u16,
    entities: Vec<Entity>,
    smoke_timer: f64,
    smoke_next_interval: f64,
}

impl CampfireScene {
    fn build_background(&mut self, rng: &mut SmallRng) {
        let bg = &mut self.layers[BG];
        bg.clear();

        // Sky: dark blue background
        let sky_style = Style::default().bg(Color::Rgb(10, 10, 30));
        for y in 0..self.ground_y {
            for x in 0..self.width {
                bg.set(x, y, ' ', sky_style);
            }
        }

        // Stars
        self.stars.clear();
        let star_count = (self.width as usize * self.ground_y as usize) / 60;
        for _ in 0..star_count {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.ground_y.saturating_sub(2));
            let char_idx = rng.random_range(0..art::STAR_CHARS.len());
            self.stars.push(Star {
                x,
                y,
                char_idx,
                visible: true,
                timer: 0.0,
                interval: rng.random_range(2.0..8.0),
            });
        }

        // Ground
        let ground_style = Style::default()
            .fg(Color::Rgb(40, 80, 20))
            .bg(Color::Rgb(20, 40, 10));
        for y in self.ground_y..self.height {
            for x in 0..self.width {
                bg.set(x, y, art::GROUND_CHAR, ground_style);
            }
        }

        // Horizon line
        let horizon_style = Style::default()
            .fg(Color::Rgb(30, 60, 15))
            .bg(Color::Rgb(20, 40, 10));
        for x in 0..self.width {
            bg.set(x, self.ground_y, '_', horizon_style);
        }
    }

    fn build_midground(&mut self, _rng: &mut SmallRng) {
        let mid = &mut self.layers[MID];
        mid.clear();

        let tree_style = Style::default().fg(Color::Rgb(30, 70, 30));

        // Place a few trees
        if self.width > 30 {
            // Left side trees
            mid.draw_ascii(2, self.ground_y as i32 - 6, art::TREE_MEDIUM, tree_style);
            if self.width > 50 {
                mid.draw_ascii(8, self.ground_y as i32 - 4, art::TREE_SMALL, tree_style);
            }

            // Right side trees
            let right_x = self.width as i32 - 10;
            mid.draw_ascii(right_x, self.ground_y as i32 - 6, art::TREE_MEDIUM, tree_style);
            if self.width > 50 {
                mid.draw_ascii(right_x - 7, self.ground_y as i32 - 4, art::TREE_SMALL, tree_style);
            }
        }
    }

    fn setup_entities(&mut self, _rng: &mut SmallRng) {
        self.entities.clear();

        // Knight (static, positioned left of center)
        let center_x = self.width as f64 / 2.0;
        let knight = Entity::new(
            center_x - 6.0,
            self.ground_y as f64 - 5.0,
            vec![art::KNIGHT.to_string(), art::KNIGHT_BREATHE.to_string()],
            3.0, // slow breathing cycle
            Style::default().fg(Color::Rgb(160, 160, 170)),
            FG,
        );
        self.entities.push(knight);

        // Fire (animated, just right of knight)
        let fire = Entity::new(
            center_x + 1.0,
            self.ground_y as f64 - 4.0,
            art::FIRE_FRAMES.iter().map(|s| s.to_string()).collect(),
            0.2,
            Style::default().fg(Color::Rgb(255, 140, 0)),
            FG,
        );
        self.entities.push(fire);
    }

    fn tick_stars(&mut self, dt: f64, rng: &mut SmallRng) {
        for star in &mut self.stars {
            star.timer += dt;
            if star.timer >= star.interval {
                star.timer = 0.0;
                star.visible = !star.visible;
                star.interval = rng.random_range(2.0..8.0);
            }
        }
    }

    fn tick_smoke(&mut self, dt: f64, rng: &mut SmallRng) {
        self.smoke_timer += dt;
        // Spawn smoke particle at pre-determined interval
        if self.smoke_timer > self.smoke_next_interval {
            self.smoke_timer = 0.0;
            self.smoke_next_interval = rng.random_range(0.5..1.5);

            let fire_x = self.width as f64 / 2.0 + 1.5;
            let fire_y = self.ground_y as f64 - 5.0;
            let char_idx = rng.random_range(0..art::SMOKE_CHARS.len());
            let ch = art::SMOKE_CHARS[char_idx];

            let mut smoke = Entity::new(
                fire_x + rng.random_range(-1.0..1.0),
                fire_y,
                vec![String::from(ch)],
                1.0,
                Style::default().fg(Color::Rgb(100, 100, 110)),
                OVERLAY,
            );
            smoke.vy = rng.random_range(-2.0..-0.5);
            smoke.vx = rng.random_range(-0.3..0.3);
            self.entities.push(smoke);
        }

        // Kill smoke that's risen too high
        for entity in &mut self.entities {
            if entity.layer == OVERLAY && entity.y < 1.0 {
                entity.alive = false;
            }
        }
    }

    fn render_stars_to_scratch(&mut self) {
        let star_style = Style::default()
            .fg(Color::Rgb(200, 200, 150))
            .bg(Color::Rgb(10, 10, 30));

        for star in &self.stars {
            if !star.visible {
                continue;
            }
            let ch = art::STAR_CHARS[star.char_idx];
            self.scratch_layer.set(star.x, star.y, ch, star_style);
        }
    }
}

impl Scene for CampfireScene {
    fn setup(width: u16, height: u16, rng: &mut SmallRng) -> Self {
        let ground_y = height * 2 / 3;
        let mut scene = Self {
            width,
            height,
            layers: (0..LAYER_COUNT).map(|_| Layer::new(width, height)).collect(),
            scratch_layer: Layer::new(width, height),
            stars: Vec::new(),
            ground_y,
            entities: Vec::new(),
            smoke_timer: 0.0,
            smoke_next_interval: rng.random_range(0.5..1.5),
        };
        scene.build_background(rng);
        scene.build_midground(rng);
        scene.setup_entities(rng);
        scene
    }

    fn tick(&mut self, dt: f64, rng: &mut SmallRng) {
        self.tick_stars(dt, rng);
        self.tick_smoke(dt, rng);

        for entity in &mut self.entities {
            entity.tick_animation(dt);
            entity.tick_movement(dt);
        }

        self.entities.retain(|e| e.alive);
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();

        // Composite background layer (sky, ground, horizon)
        self.layers[BG].composite(buf, area);

        // Render stars using pre-allocated scratch layer
        self.scratch_layer.clear();
        self.render_stars_to_scratch();
        self.scratch_layer.composite(buf, area);

        // Composite midground (trees)
        self.layers[MID].composite(buf, area);

        // Render entities by layer order using scratch layer
        for layer_idx in 0..LAYER_COUNT {
            let has_entities = self.entities.iter().any(|e| e.layer == layer_idx);
            if !has_entities {
                continue;
            }
            self.scratch_layer.clear();
            for entity in &self.entities {
                if entity.layer == layer_idx {
                    self.scratch_layer.draw_ascii(
                        entity.x as i32,
                        entity.y as i32,
                        entity.current_frame(),
                        entity.style,
                    );
                }
            }
            self.scratch_layer.composite(buf, area);
        }
    }

    fn resize(&mut self, width: u16, height: u16, rng: &mut SmallRng) {
        self.width = width;
        self.height = height;
        self.ground_y = height * 2 / 3;
        self.layers = (0..LAYER_COUNT).map(|_| Layer::new(width, height)).collect();
        self.scratch_layer = Layer::new(width, height);
        self.build_background(rng);
        self.build_midground(rng);
        self.setup_entities(rng);
    }
}
