use rand::rngs::SmallRng;
use rand::Rng;

/// Global wind state with smooth gusting.
/// Scenes tick this and query `force_x()` to apply wind to entities.
/// Multiply by each entity's wind sensitivity (smoke: 1.0, tree: 0.2, etc).
pub struct Wind {
    /// Current wind strength (-1.0 leftward to 1.0 rightward)
    strength: f64,
    /// Target strength we're smoothly interpolating toward
    target: f64,
    /// Interpolation speed (per second)
    lerp_speed: f64,
    /// Timer until next gust change
    gust_timer: f64,
    /// Current interval between gust changes
    gust_interval: f64,
}

impl Wind {
    pub fn new(rng: &mut SmallRng) -> Self {
        Self {
            strength: 0.0,
            target: 0.0,
            lerp_speed: 0.5,
            gust_timer: 0.0,
            gust_interval: rng.random_range(2.0..6.0),
        }
    }

    pub fn tick(&mut self, dt: f64, rng: &mut SmallRng) {
        self.gust_timer += dt;
        if self.gust_timer >= self.gust_interval {
            self.gust_timer = 0.0;
            self.target = rng.random_range(-1.0..1.0);
            self.gust_interval = rng.random_range(2.0..6.0);
            self.lerp_speed = rng.random_range(0.3..1.0);
        }
        let diff = self.target - self.strength;
        self.strength += diff * self.lerp_speed * dt;
    }

    /// Current horizontal wind force.
    pub fn force_x(&self) -> f64 {
        self.strength
    }
}
