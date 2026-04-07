use rand::rngs::SmallRng;
use ratatui::Frame;

pub trait Scene {
    /// Create the scene for a given terminal size.
    fn setup(width: u16, height: u16, rng: &mut SmallRng) -> Self
    where
        Self: Sized;

    /// Advance the scene by dt seconds.
    fn tick(&mut self, dt: f64, rng: &mut SmallRng);

    /// Draw the scene to the frame.
    fn render(&mut self, frame: &mut Frame);

    /// Handle terminal resize.
    fn resize(&mut self, width: u16, height: u16, rng: &mut SmallRng);
}
