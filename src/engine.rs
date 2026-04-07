use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    execute,
    cursor,
    event::{self, Event, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::scene::Scene;

pub fn run<S: Scene>(fps: u32) -> Result<(), Box<dyn std::error::Error>> {
    // Install panic hook to restore terminal on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen, cursor::Show);
        original_hook(panic_info);
    }));

    // Terminal setup
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut rng = SmallRng::from_os_rng();
    let size = terminal.size()?;
    let mut scene = S::setup(size.width, size.height, &mut rng);

    let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);
    let mut last_tick = Instant::now();

    let result = main_loop(&mut terminal, &mut scene, &mut rng, frame_duration, &mut last_tick);

    // Terminal teardown (always runs)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;

    result
}

fn main_loop<S: Scene>(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    scene: &mut S,
    rng: &mut SmallRng,
    frame_duration: Duration,
    last_tick: &mut Instant,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Calculate delta time
        let now = Instant::now();
        let dt = now.duration_since(*last_tick).as_secs_f64();
        *last_tick = now;

        // Tick the scene
        scene.tick(dt, rng);

        // Render
        terminal.draw(|frame| {
            scene.render(frame);
        })?;

        // Input polling: any key = quit
        let sleep_time = frame_duration.saturating_sub(Instant::now().duration_since(now));
        if event::poll(sleep_time)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    return Ok(());
                }
                Event::Resize(w, h) => {
                    scene.resize(w, h, rng);
                }
                _ => {}
            }
        }
    }
}
