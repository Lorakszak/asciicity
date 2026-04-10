use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rand::SeedableRng;
use rand::rngs::SmallRng;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::scene::{Scene, SceneConfig};

pub fn run<S: Scene>(fps: u32, cfg: SceneConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Install panic hook so a panic anywhere inside the render loop still
    // restores the terminal. Normal Err returns are handled by the teardown
    // block below.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen, cursor::Show);
        original_hook(panic_info);
    }));

    enable_raw_mode()?;

    // Once raw mode is on we MUST disable it again, even if any setup step
    // below returns Err. Collect the run result and tear down unconditionally.
    let result = run_inner::<S>(fps, cfg);

    let _ = disable_raw_mode();
    let _ = execute!(io::stderr(), LeaveAlternateScreen, cursor::Show);
    result
}

fn run_inner<S: Scene>(fps: u32, cfg: SceneConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut rng = SmallRng::from_os_rng();
    let size = terminal.size()?;
    let mut scene = S::setup(size.width, size.height, &cfg, &mut rng);

    // Defensive floor: fps = 0 would make from_secs_f64 panic on infinity.
    // CLI validation already rejects this, but belt-and-suspenders.
    let fps = fps.max(1);
    let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);
    let mut last_tick = Instant::now();

    main_loop(
        &mut terminal,
        &mut scene,
        &mut rng,
        frame_duration,
        &mut last_tick,
    )
}

fn main_loop<S: Scene>(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    scene: &mut S,
    rng: &mut SmallRng,
    frame_duration: Duration,
    last_tick: &mut Instant,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let now = Instant::now();
        let dt = now.duration_since(*last_tick).as_secs_f64();
        *last_tick = now;

        scene.tick(dt, rng);

        terminal.draw(|frame| {
            scene.render(frame);
        })?;

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
