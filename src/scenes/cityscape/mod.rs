mod art;

use rand::rngs::SmallRng;
use rand::Rng;
use ratatui::style::{Color, Style};
use ratatui::Frame;

use crate::behavior::daynight::DayNight;
use crate::behavior::parallax::Parallax;
use crate::behavior::weather::{Weather, WeatherType};
use crate::behavior::wind::Wind;
use crate::entity::Entity;
use crate::layer::Layer;
use crate::scene::Scene;

// color utilities available: crate::color::{lerp_rgb, tint_rgb, fade_rgb}

const FG: usize = 0;
const OVERLAY: usize = 1;

// Parallax
const PARALLAX_RANGE: f64 = 25.0;
const PARALLAX_SPEED: f64 = 1.0;
const FAR_DEPTH: f64 = 0.12;
const MID_DEPTH: f64 = 0.35;
const FAR_EXTRA: u16 = 5;
const MID_EXTRA: u16 = 12;

// Cloud limits
const MAX_CLOUDS: usize = 10;

struct Building {
    x: i32,
    width: i32,
    height: i32,
    has_antenna: bool,
    antenna_height: i32,
    window_seed: u64,
}

struct Star {
    x: u16,
    y: u16,
    char_idx: usize,
    visible: bool,
    timer: f64,
    interval: f64,
}

struct CityscapeArt {
    person_table: crate::art::ArtData,
    clouds: Vec<crate::art::ArtData>,
    planes: Vec<crate::art::ArtData>,
    helis: Vec<crate::art::ArtData>,
    bird: crate::art::ArtData,
    cars: Vec<crate::art::ArtData>,
}

impl CityscapeArt {
    fn load() -> Self {
        Self {
            person_table: crate::art::load(
                "cityscape",
                "person_table",
                art::PERSON_TABLE_DEFAULT,
            ),
            clouds: vec![
                crate::art::load("cityscape", "cloud_small", art::CLOUD_SMALL_DEFAULT),
                crate::art::load("cityscape", "cloud_large", art::CLOUD_LARGE_DEFAULT),
                crate::art::load("cityscape", "cloud_wispy", art::CLOUD_WISPY_DEFAULT),
                crate::art::load("cityscape", "cloud_flat", art::CLOUD_FLAT_DEFAULT),
                crate::art::load("cityscape", "cloud_tiny", art::CLOUD_TINY_DEFAULT),
            ],
            planes: vec![
                crate::art::load("cityscape", "plane", art::PLANE_DEFAULT),
                crate::art::load("cityscape", "plane2", art::PLANE2_DEFAULT),
                crate::art::load("cityscape", "plane3", art::PLANE3_DEFAULT),
            ],
            helis: vec![
                crate::art::load("cityscape", "heli", art::HELI_DEFAULT),
                crate::art::load("cityscape", "heli2", art::HELI2_DEFAULT),
            ],
            bird: crate::art::load("cityscape", "bird", art::BIRD_DEFAULT),
            cars: vec![
                crate::art::load("cityscape", "car_sedan", art::CAR_SEDAN_DEFAULT),
                crate::art::load("cityscape", "car_truck", art::CAR_TRUCK_DEFAULT),
                crate::art::load("cityscape", "car_sports", art::CAR_SPORTS_DEFAULT),
                crate::art::load("cityscape", "car_taxi", art::CAR_TAXI_DEFAULT),
                crate::art::load("cityscape", "car_van", art::CAR_VAN_DEFAULT),
            ],
        }
    }
}

pub struct CityscapeScene {
    width: u16,
    height: u16,
    skyline_y: u16,

    // Layers
    sky_layer: Layer,
    far_layer: Layer,
    mid_layer: Layer,
    rooftop_layer: Layer,
    scratch_layer: Layer,

    // Buildings (for window animation)
    far_buildings: Vec<Building>,
    mid_buildings: Vec<Building>,

    // Stars
    stars: Vec<Star>,

    // Entities
    entities: Vec<Entity>,

    // Behaviors
    wind: Wind,
    daynight: DayNight,
    parallax: Parallax,
    weather: Weather,

    // Spawners
    cloud_timer: f64,
    cloud_next: f64,
    plane_timer: f64,
    plane_next: f64,
    heli_timer: f64,
    heli_next: f64,
    bird_timer: f64,
    bird_next: f64,
    car_timer: f64,
    car_next: f64,
    smoke_timer: f64,
    smoke_next: f64,
    person_x: f64,
    person_y: f64,

    art: CityscapeArt,
}

// Tags to identify entity types
// Entity tags reserved for future use: cloud=1, plane=2, bird=3, smoke=4

impl CityscapeScene {
    fn generate_buildings(
        rng: &mut SmallRng,
        layer_width: i32,
        skyline_y: i32,
        min_frac: f64,
        max_frac: f64,
        min_width: i32,
        max_width: i32,
        gap_min: i32,
        gap_max: i32,
    ) -> Vec<Building> {
        let mut buildings = Vec::new();
        let mut x = 0;
        let sky_h = skyline_y as f64;
        while x < layer_width {
            let w = rng.random_range(min_width..max_width);
            let h = rng.random_range((sky_h * min_frac) as i32..(sky_h * max_frac) as i32);
            let has_antenna = rng.random_range(0..4_u32) == 0;
            buildings.push(Building {
                x,
                width: w,
                height: h.max(3),
                has_antenna,
                antenna_height: if has_antenna {
                    rng.random_range(2..5)
                } else {
                    0
                },
                window_seed: rng.random_range(0..10000_u64),
            });
            x += w + rng.random_range(gap_min..gap_max);
        }
        buildings
    }

    fn draw_buildings_to_layer(
        layer: &mut Layer,
        buildings: &[Building],
        ground_y: i32,
        wall_style: Style,
        roof_style: Style,
        antenna_style: Style,
    ) {
        for b in buildings {
            let top = ground_y - b.height;

            // Roof line
            for x in b.x..(b.x + b.width) {
                if x >= 0 && x < layer.width as i32 && top >= 0 {
                    layer.set(x as u16, top as u16, '_', roof_style);
                }
            }

            // Walls and fill
            for y in (top + 1)..ground_y {
                if y < 0 {
                    continue;
                }
                if b.x >= 0 && b.x < layer.width as i32 {
                    layer.set(b.x as u16, y as u16, '|', wall_style);
                }
                let rx = b.x + b.width - 1;
                if rx >= 0 && rx < layer.width as i32 {
                    layer.set(rx as u16, y as u16, '|', wall_style);
                }
                for x in (b.x + 1)..(b.x + b.width - 1) {
                    if x >= 0 && x < layer.width as i32 {
                        layer.set(x as u16, y as u16, ' ', wall_style);
                    }
                }
            }

            // Subtle ledge on taller buildings (just the edges)
            if b.height > 12 {
                let ledge_y = top + b.height / 3;
                if ledge_y >= 0 && ledge_y < layer.height as i32 {
                    if b.x >= 0 && b.x < layer.width as i32 {
                        layer.set(b.x as u16, ledge_y as u16, '[', roof_style);
                    }
                    let rx = b.x + b.width - 1;
                    if rx >= 0 && rx < layer.width as i32 {
                        layer.set(rx as u16, ledge_y as u16, ']', roof_style);
                    }
                }
            }

            // Antenna
            if b.has_antenna {
                let ax = b.x + b.width / 2;
                if ax >= 0 && ax < layer.width as i32 {
                    for dy in 1..=b.antenna_height {
                        let ay = top - dy;
                        if ay >= 0 {
                            layer.set(ax as u16, ay as u16, '|', antenna_style);
                        }
                    }
                    let tip_y = top - b.antenna_height - 1;
                    if tip_y >= 0 {
                        layer.set(ax as u16, tip_y as u16, '*', antenna_style);
                    }
                }
            }
        }
    }

    fn is_window_lit(seed: u64, wx: i32, wy: i32, time: f64, night_factor: f64) -> bool {
        let hash = seed
            .wrapping_mul(31)
            .wrapping_add(wx as u64 * 17)
            .wrapping_add(wy as u64 * 53);
        let offset = (hash % 1000) as f64 / 1000.0 * 20.0;
        let phase = (time * 0.15 + offset).sin() * 0.5 + 0.5;
        let threshold = night_factor * 0.75;
        phase < threshold
    }

    fn draw_windows(
        layer: &mut Layer,
        buildings: &[Building],
        ground_y: i32,
        time: f64,
        night_factor: f64,
        lit_style: Style,
    ) {
        for b in buildings {
            let top = ground_y - b.height;
            let mut wy = top + 2;
            while wy < ground_y - 1 {
                let mut wx = b.x + 2;
                while wx < b.x + b.width - 2 {
                    if wx >= 0 && wx < layer.width as i32 && wy >= 0 && wy < layer.height as i32 {
                        if Self::is_window_lit(b.window_seed, wx, wy, time, night_factor) {
                            layer.set(wx as u16, wy as u16, '#', lit_style);
                        }
                    }
                    wx += 3;
                }
                wy += 2;
            }
        }
    }

    fn build_sky(&mut self) {
        self.sky_layer.clear();
        let sky_color = self.daynight.sky_color();
        let style = Style::default().bg(sky_color);
        for y in 0..self.height {
            for x in 0..self.width {
                self.sky_layer.set(x, y, ' ', style);
            }
        }
    }

    fn build_far_skyline(&mut self) {
        self.far_layer.clear();
        let ambient = self.daynight.ambient();
        let night = Color::Rgb(15, 18, 30);
        let day = Color::Rgb(60, 65, 80);
        let base = crate::color::lerp_rgb(night, day, ambient);
        let wall_style = Style::default().fg(base).bg(base);
        let roof_night = Color::Rgb(25, 30, 45);
        let roof_day = Color::Rgb(55, 60, 75);
        let roof_style = Style::default().fg(crate::color::lerp_rgb(roof_night, roof_day, ambient));
        let ant_night = Color::Rgb(50, 50, 60);
        let ant_day = Color::Rgb(100, 100, 110);
        let antenna_style = Style::default().fg(crate::color::lerp_rgb(ant_night, ant_day, ambient));

        Self::draw_buildings_to_layer(
            &mut self.far_layer,
            &self.far_buildings,
            self.skyline_y as i32,
            wall_style,
            roof_style,
            antenna_style,
        );

        let night = 1.0 - self.daynight.ambient();
        let lit_style = Style::default()
            .fg(Color::Rgb(220, 200, 140))
            .bg(base);
        Self::draw_windows(
            &mut self.far_layer,
            &self.far_buildings,
            self.skyline_y as i32,
            self.daynight.time(),
            night,
            lit_style,
        );
    }

    fn build_mid_skyline(&mut self) {
        self.mid_layer.clear();
        let ambient = self.daynight.ambient();
        let night = Color::Rgb(10, 12, 22);
        let day = Color::Rgb(45, 50, 65);
        let base = crate::color::lerp_rgb(night, day, ambient);
        let wall_style = Style::default().fg(base).bg(base);
        let roof_night = Color::Rgb(20, 22, 35);
        let roof_day = Color::Rgb(45, 50, 65);
        let roof_style = Style::default().fg(crate::color::lerp_rgb(roof_night, roof_day, ambient));
        let ant_night = Color::Rgb(40, 40, 50);
        let ant_day = Color::Rgb(90, 90, 100);
        let antenna_style = Style::default().fg(crate::color::lerp_rgb(ant_night, ant_day, ambient));

        Self::draw_buildings_to_layer(
            &mut self.mid_layer,
            &self.mid_buildings,
            self.skyline_y as i32,
            wall_style,
            roof_style,
            antenna_style,
        );

        let night = 1.0 - self.daynight.ambient();
        let lit_style = Style::default()
            .fg(Color::Rgb(255, 230, 150))
            .bg(Color::Rgb(40, 35, 20));
        Self::draw_windows(
            &mut self.mid_layer,
            &self.mid_buildings,
            self.skyline_y as i32,
            self.daynight.time(),
            night,
            lit_style,
        );
    }

    fn build_road(&mut self) {
        self.rooftop_layer.clear();

        // Road surface
        let road_style = Style::default()
            .fg(Color::Rgb(40, 40, 45))
            .bg(Color::Rgb(30, 30, 35));
        for y in self.skyline_y..self.height {
            for x in 0..self.width {
                self.rooftop_layer.set(x, y, ' ', road_style);
            }
        }

        // Curb / sidewalk edge at top
        let curb_style = Style::default()
            .fg(Color::Rgb(80, 80, 75))
            .bg(Color::Rgb(55, 55, 50));
        for x in 0..self.width {
            self.rooftop_layer.set(x, self.skyline_y, '_', curb_style);
        }

        // Dashed center line
        let lane_y = self.skyline_y + (self.height - self.skyline_y) / 2;
        let dash_style = Style::default()
            .fg(Color::Rgb(180, 170, 50))
            .bg(Color::Rgb(30, 30, 35));
        for x in 0..self.width {
            if (x / 3) % 2 == 0 {
                self.rooftop_layer.set(x, lane_y, '-', dash_style);
            }
        }
    }

    fn setup_stars(&mut self, rng: &mut SmallRng) {
        self.stars.clear();
        let sky_area = self.width as usize * self.skyline_y.saturating_sub(3) as usize;
        let count = sky_area / 35;
        for _ in 0..count {
            self.stars.push(Star {
                x: rng.random_range(0..self.width),
                y: rng.random_range(0..self.skyline_y.saturating_sub(3)),
                char_idx: rng.random_range(0..art::STAR_CHARS.len()),
                visible: true,
                timer: 0.0,
                interval: rng.random_range(2.0..8.0),
            });
        }
    }

    fn setup_entities(&mut self, rng: &mut SmallRng) {
        self.entities.clear();

        // Person at table on right side
        self.person_x = self.width as f64 - 14.0;
        self.person_y = self.skyline_y as f64 - 5.0;
        let person = Entity::new(
            self.person_x,
            self.person_y,
            self.art.person_table.frames.clone(),
            4.0,
            Style::default().fg(Color::Rgb(170, 160, 150)),
            FG,
        );
        self.entities.push(person);

        // Initial clouds (2)
        self.spawn_cloud(rng);
        self.spawn_cloud(rng);
    }

    fn cloud_count(&self) -> usize {
        self.entities
            .iter()
            .filter(|e| e.layer == OVERLAY && e.frame_interval > 0.9 && e.frame_interval < 1.1)
            .count()
    }

    fn spawn_cloud(&mut self, rng: &mut SmallRng) {
        if self.cloud_count() >= MAX_CLOUDS {
            return;
        }
        let idx = rng.random_range(0..self.art.clouds.len());
        let frames = self.art.clouds[idx].frames.clone();
        let y = rng.random_range(1.0..(self.skyline_y as f64 * 0.35));
        let x = -(rng.random_range(5.0..25.0));

        let brightness = rng.random_range(50..100_u8);
        let mut cloud = Entity::new(
            x,
            y,
            frames,
            1.0, // tag: cloud
            Style::default().fg(Color::Rgb(brightness, brightness, brightness + 20)),
            OVERLAY,
        );
        cloud.vx = rng.random_range(3.0..8.0); // much faster
        self.entities.push(cloud);
    }

    fn spawn_plane(&mut self, rng: &mut SmallRng) {
        let idx = rng.random_range(0..self.art.planes.len());
        let art_faces_right = idx == 2; // plane3 faces right, others face left
        let going_right = rng.random_range(0..2_u32) == 0;

        let frames = if going_right == art_faces_right {
            self.art.planes[idx].frames.clone()
        } else {
            crate::art::mirror_frames(&self.art.planes[idx].frames)
        };

        let y = rng.random_range(2.0..(self.skyline_y as f64 * 0.3));
        let x = if going_right {
            -20.0
        } else {
            self.width as f64 + 10.0
        };

        let mut plane = Entity::new(
            x, y, frames, 0.5,
            Style::default().fg(Color::Rgb(180, 180, 190)),
            OVERLAY,
        );
        plane.vx = if going_right {
            rng.random_range(12.0..20.0)
        } else {
            -rng.random_range(12.0..20.0)
        };
        self.entities.push(plane);
    }

    fn spawn_heli(&mut self, rng: &mut SmallRng) {
        let idx = rng.random_range(0..self.art.helis.len());
        // Both helis face left
        let going_right = rng.random_range(0..2_u32) == 0;

        let frames = if going_right {
            // Art faces left, need mirror for going right
            crate::art::mirror_frames(&self.art.helis[idx].frames)
        } else {
            self.art.helis[idx].frames.clone()
        };

        let y = rng.random_range(1.0..(self.skyline_y as f64 * 0.4));
        let x = if going_right {
            -20.0
        } else {
            self.width as f64 + 10.0
        };

        let mut heli = Entity::new(
            x, y, frames, 0.4,
            Style::default().fg(Color::Rgb(150, 150, 160)),
            OVERLAY,
        );
        heli.vx = if going_right {
            rng.random_range(4.0..8.0)
        } else {
            -rng.random_range(4.0..8.0)
        };
        self.entities.push(heli);
    }

    fn spawn_car(&mut self, rng: &mut SmallRng) {
        let idx = rng.random_range(0..self.art.cars.len());
        let road_h = self.height - self.skyline_y;
        let lane_mid = self.skyline_y + road_h / 2;
        let going_right = rng.random_range(0..2_u32) == 0;

        // All car art faces right. Mirror for left-going lane.
        let frames = if going_right {
            self.art.cars[idx].frames.clone()
        } else {
            crate::art::mirror_frames(&self.art.cars[idx].frames)
        };

        // Top lane goes right, bottom lane goes left. Cars are ~4 rows tall.
        let y = if going_right {
            (self.skyline_y + 2) as f64
        } else {
            (lane_mid + 1) as f64
        };
        let x = if going_right {
            -(rng.random_range(10.0..30.0))
        } else {
            self.width as f64 + rng.random_range(5.0..20.0)
        };

        // Random car colors
        let colors = [
            Color::Rgb(180, 50, 50),   // red
            Color::Rgb(50, 50, 180),   // blue
            Color::Rgb(180, 180, 180), // silver
            Color::Rgb(200, 200, 60),  // yellow (taxi)
            Color::Rgb(60, 60, 60),    // dark
            Color::Rgb(180, 120, 40),  // orange
        ];
        let color = colors[rng.random_range(0..colors.len())];

        let mut car = Entity::new(
            x,
            y,
            frames,
            1.0,
            Style::default().fg(color),
            FG,
        );
        car.vx = if going_right {
            rng.random_range(5.0..12.0)
        } else {
            -rng.random_range(5.0..12.0)
        };
        self.entities.push(car);
    }

    fn spawn_birds(&mut self, rng: &mut SmallRng) {
        let flock_size = rng.random_range(2..5_u32);
        let base_y = rng.random_range(3.0..(self.skyline_y as f64 * 0.5));
        let base_x: f64 = -5.0;
        let base_vx = rng.random_range(4.0..7.0);

        for i in 0..flock_size {
            let frames = self.art.bird.frames.clone();
            let mut bird = Entity::new(
                base_x - (i as f64 * 2.0),
                base_y + rng.random_range(-1.0..1.0),
                frames,
                0.3,
                Style::default().fg(Color::Rgb(40, 40, 50)),
                OVERLAY,
            );
            bird.vx = base_vx + rng.random_range(-0.5..0.5);
            bird.vy = rng.random_range(-0.3..0.3);
            self.entities.push(bird);
        }
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

    fn tick_spawners(&mut self, dt: f64, rng: &mut SmallRng) {
        let wind = self.wind.force_x();

        // Clouds - spawn 1-5 at once, sometimes clumped
        self.cloud_timer += dt;
        if self.cloud_timer >= self.cloud_next {
            self.cloud_timer = 0.0;
            self.cloud_next = rng.random_range(6.0..15.0);
            let batch = rng.random_range(1..6_u32);
            for _ in 0..batch {
                self.spawn_cloud(rng);
            }
        }

        // Planes
        self.plane_timer += dt;
        if self.plane_timer >= self.plane_next {
            self.plane_timer = 0.0;
            self.plane_next = rng.random_range(30.0..90.0);
            self.spawn_plane(rng);
        }

        // Helicopters
        self.heli_timer += dt;
        if self.heli_timer >= self.heli_next {
            self.heli_timer = 0.0;
            self.heli_next = rng.random_range(40.0..100.0);
            self.spawn_heli(rng);
        }

        // Birds
        self.bird_timer += dt;
        if self.bird_timer >= self.bird_next {
            self.bird_timer = 0.0;
            self.bird_next = rng.random_range(15.0..40.0);
            self.spawn_birds(rng);
        }

        // Cars
        self.car_timer += dt;
        if self.car_timer >= self.car_next {
            self.car_timer = 0.0;
            self.car_next = rng.random_range(2.0..6.0);
            self.spawn_car(rng);
        }

        // Cigarette smoke from person
        self.smoke_timer += dt;
        if self.smoke_timer >= self.smoke_next {
            self.smoke_timer = 0.0;
            self.smoke_next = rng.random_range(0.4..1.2);

            let smoke_chars = ['~', '.', '\'', ','];
            let ch = smoke_chars[rng.random_range(0..smoke_chars.len())];
            let mut smoke = Entity::new(
                self.person_x + 7.5,
                self.person_y + 1.0,
                vec![String::from(ch)],
                0.3, // tag distinguisher
                Style::default().fg(Color::Rgb(90, 90, 100)),
                OVERLAY,
            );
            smoke.vy = rng.random_range(-1.5..-0.5);
            smoke.vx = wind * 0.8 + rng.random_range(-0.2..0.5);
            self.entities.push(smoke);
        }

        // Kill off-screen entities
        let w = self.width as f64;
        for entity in &mut self.entities {
            if entity.layer == OVERLAY {
                if entity.x > w + 30.0 || entity.x < -30.0 || entity.y < -5.0 {
                    entity.alive = false;
                }
            }
        }
    }

    fn render_stars(&mut self) {
        let visibility = self.daynight.stars_visibility();
        if visibility < 0.05 {
            return;
        }
        let sky_color = self.daynight.sky_color();
        for star in &self.stars {
            if !star.visible {
                continue;
            }
            let ch = art::STAR_CHARS[star.char_idx];
            let b = (180.0 * visibility) as u8;
            let style = Style::default()
                .fg(Color::Rgb(b, b, (b as f64 * 0.8) as u8))
                .bg(sky_color);
            self.scratch_layer.set(star.x, star.y, ch, style);
        }
    }
}

impl Scene for CityscapeScene {
    fn setup(width: u16, height: u16, rng: &mut SmallRng) -> Self {
        // Road area: enough for two lanes of cars (~10 rows)
        let road_rows = (height / 4).max(8).min(12);
        let skyline_y = height - road_rows;
        let art = CityscapeArt::load();

        let far_w = width + FAR_EXTRA;
        let mid_w = width + MID_EXTRA;

        // Singapore-style: tall skyscrapers, 40-80% of sky height
        let far_buildings = Self::generate_buildings(
            rng,
            far_w as i32,
            skyline_y as i32,
            0.4,
            0.85, // tall skyscrapers in distance
            5,
            12,
            0,
            2,
        );
        let mid_buildings = Self::generate_buildings(
            rng,
            mid_w as i32,
            skyline_y as i32,
            0.15,
            0.4, // shorter buildings up front
            7,
            16,
            1,
            3,
        );

        let mut scene = Self {
            width,
            height,
            skyline_y,
            sky_layer: Layer::new(width, height),
            far_layer: Layer::new(far_w, height),
            mid_layer: Layer::new(mid_w, height),
            rooftop_layer: Layer::new(width, height),
            scratch_layer: Layer::new(width, height),
            far_buildings,
            mid_buildings,
            stars: Vec::new(),
            entities: Vec::new(),
            wind: Wind::new(rng),
            daynight: DayNight::new(20.0, 0.2),
            parallax: Parallax::new(PARALLAX_SPEED, 0.0, PARALLAX_RANGE),
            weather: Weather::clear(),
            cloud_timer: 0.0,
            cloud_next: rng.random_range(5.0..12.0),
            plane_timer: 0.0,
            plane_next: rng.random_range(20.0..60.0),
            heli_timer: 0.0,
            heli_next: rng.random_range(40.0..100.0),
            bird_timer: 0.0,
            bird_next: rng.random_range(10.0..25.0),
            car_timer: 0.0,
            car_next: rng.random_range(1.0..3.0),
            smoke_timer: 0.0,
            smoke_next: rng.random_range(0.4..1.2),
            person_x: 0.0,
            person_y: 0.0,
            art,
        };

        scene.build_sky();
        scene.build_far_skyline();
        scene.build_mid_skyline();
        scene.build_road();
        scene.setup_stars(rng);
        scene.setup_entities(rng);
        scene
    }

    fn tick(&mut self, dt: f64, rng: &mut SmallRng) {
        self.wind.tick(dt, rng);
        self.daynight.tick(dt);
        self.parallax.tick(dt);
        self.weather
            .tick(dt, rng, self.width, self.height, self.wind.force_x());

        self.build_sky();
        self.build_far_skyline();
        self.build_mid_skyline();

        self.tick_stars(dt, rng);
        self.tick_spawners(dt, rng);

        for entity in &mut self.entities {
            entity.tick_animation(dt);
            entity.tick_movement(dt);
        }
        self.entities.retain(|e| e.alive);
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();

        // 1. Sky
        self.sky_layer.composite(buf, area);

        // 2. Stars
        self.scratch_layer.clear();
        self.render_stars();
        self.scratch_layer.composite(buf, area);

        // 3. Far skyline with parallax
        let far_ox = self.parallax.offset_x(FAR_DEPTH);
        self.far_layer.composite_offset(buf, area, far_ox, 0);

        // 4. Mid skyline with parallax
        let mid_ox = self.parallax.offset_x(MID_DEPTH);
        self.mid_layer.composite_offset(buf, area, mid_ox, 0);

        // 5. Rooftop
        self.rooftop_layer.composite(buf, area);

        // 6. Foreground entities (person)
        self.scratch_layer.clear();
        for entity in &self.entities {
            if entity.layer == FG {
                self.scratch_layer.draw_ascii(
                    entity.x as i32,
                    entity.y as i32,
                    entity.current_frame(),
                    entity.style,
                );
            }
        }
        self.scratch_layer.composite(buf, area);

        // 7. Overlay entities (clouds, planes, birds, smoke)
        self.scratch_layer.clear();
        for entity in &self.entities {
            if entity.layer == OVERLAY {
                self.scratch_layer.draw_ascii(
                    entity.x as i32,
                    entity.y as i32,
                    entity.current_frame(),
                    entity.style,
                );
            }
        }
        self.scratch_layer.composite(buf, area);

        // 8. Weather
        if self.weather.weather_type() != WeatherType::Clear {
            self.scratch_layer.clear();
            self.weather.render(&mut self.scratch_layer);
            self.scratch_layer.composite(buf, area);
        }
    }

    fn resize(&mut self, width: u16, height: u16, rng: &mut SmallRng) {
        self.width = width;
        self.height = height;
        let road_rows = (height / 4).max(8).min(12);
        self.skyline_y = height - road_rows;

        let far_w = width + FAR_EXTRA;
        let mid_w = width + MID_EXTRA;

        self.sky_layer = Layer::new(width, height);
        self.far_layer = Layer::new(far_w, height);
        self.mid_layer = Layer::new(mid_w, height);
        self.rooftop_layer = Layer::new(width, height);
        self.scratch_layer = Layer::new(width, height);

        self.far_buildings = Self::generate_buildings(
            rng,
            far_w as i32,
            self.skyline_y as i32,
            0.4,
            0.85,
            5,
            12,
            0,
            2,
        );
        self.mid_buildings = Self::generate_buildings(
            rng,
            mid_w as i32,
            self.skyline_y as i32,
            0.15,
            0.4,
            7,
            16,
            1,
            3,
        );

        self.build_sky();
        self.build_far_skyline();
        self.build_mid_skyline();
        self.build_road();
        self.setup_stars(rng);
        self.setup_entities(rng);
    }
}
