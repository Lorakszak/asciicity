use ratatui::style::Color;

use crate::color::lerp_rgb;

/// Day/night cycle that tracks time of day and provides colors.
/// Speed controls how fast time passes (e.g., 0.5 = 1 full cycle per 48 real seconds).
pub struct DayNight {
    /// Current time (0.0 to 24.0 hours)
    time: f64,
    /// How many in-game hours pass per real second
    speed: f64,
}

/// Sky color keyframes: (hour, color)
const SKY_KEYFRAMES: &[(f64, (u8, u8, u8))] = &[
    (0.0, (5, 5, 20)),       // midnight
    (5.0, (10, 10, 30)),     // pre-dawn
    (6.0, (60, 40, 50)),     // dawn
    (7.0, (120, 80, 60)),    // sunrise
    (8.0, (100, 150, 200)),  // morning
    (12.0, (135, 180, 220)), // noon
    (17.0, (100, 150, 200)), // afternoon
    (18.0, (140, 90, 60)),   // sunset
    (19.0, (60, 30, 50)),    // dusk
    (20.0, (15, 10, 35)),    // evening
    (24.0, (5, 5, 20)),      // midnight (wrap)
];

impl DayNight {
    pub fn new(start_time: f64, speed: f64) -> Self {
        Self {
            time: start_time % 24.0,
            speed,
        }
    }

    pub fn tick(&mut self, dt: f64) {
        self.time = (self.time + self.speed * dt) % 24.0;
    }

    /// Current time of day (0.0 to 24.0).
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Sky color for the current time, interpolated between keyframes.
    pub fn sky_color(&self) -> Color {
        for i in 0..SKY_KEYFRAMES.len() - 1 {
            let (t0, (r0, g0, b0)) = SKY_KEYFRAMES[i];
            let (t1, (r1, g1, b1)) = SKY_KEYFRAMES[i + 1];
            if self.time >= t0 && self.time <= t1 {
                let t = (self.time - t0) / (t1 - t0);
                return lerp_rgb(
                    Color::Rgb(r0, g0, b0),
                    Color::Rgb(r1, g1, b1),
                    t,
                );
            }
        }
        let (_, (r, g, b)) = SKY_KEYFRAMES[0];
        Color::Rgb(r, g, b)
    }

    /// Ambient light factor: 0.0 = dark night, 1.0 = bright midday.
    pub fn ambient(&self) -> f64 {
        let hour_rad = (self.time - 6.0) * std::f64::consts::PI / 12.0;
        hour_rad.sin().max(0.0)
    }

    /// Star visibility: 1.0 = fully visible (night), 0.0 = hidden (day).
    /// Fades during dawn/dusk transitions.
    pub fn stars_visibility(&self) -> f64 {
        if self.time < 5.0 || self.time > 21.0 {
            1.0
        } else if self.time < 7.0 {
            // Dawn fade-out
            1.0 - (self.time - 5.0) / 2.0
        } else if self.time > 19.0 {
            // Dusk fade-in
            (self.time - 19.0) / 2.0
        } else {
            0.0
        }
    }
}
