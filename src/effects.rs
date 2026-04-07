use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

use crate::color::tint_rgb;

/// Apply radial glow: tint cells within radius of (cx, cy) toward a color.
/// Strength falls off linearly from `intensity` at center to 0 at edge.
pub fn apply_glow(
    buf: &mut Buffer,
    area: Rect,
    cx: f64,
    cy: f64,
    color: Color,
    radius: f64,
    intensity: f64,
) {
    let r2 = radius * radius;
    for y in 0..area.height {
        for x in 0..area.width {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            let dist2 = dx * dx + dy * dy;
            if dist2 >= r2 {
                continue;
            }
            let dist = dist2.sqrt();
            let strength = intensity * (1.0 - dist / radius);
            if let Some(cell) = buf.cell_mut((area.x + x, area.y + y)) {
                cell.set_fg(tint_rgb(cell.fg, color, strength));
                cell.set_bg(tint_rgb(cell.bg, color, strength * 0.5));
            }
        }
    }
}

/// Tint a horizontal band of the buffer, fading linearly between two strengths.
/// Useful for smoke fading, atmospheric haze, etc.
pub fn apply_vertical_fade(
    buf: &mut Buffer,
    area: Rect,
    y_start: u16,
    y_end: u16,
    color: Color,
    strength_start: f64,
    strength_end: f64,
) {
    if y_start >= y_end {
        return;
    }
    let span = (y_end - y_start) as f64;
    for y in y_start..y_end.min(area.height) {
        let t = (y - y_start) as f64 / span;
        let strength = strength_start + (strength_end - strength_start) * t;
        for x in 0..area.width {
            if let Some(cell) = buf.cell_mut((area.x + x, area.y + y)) {
                cell.set_fg(tint_rgb(cell.fg, color, strength));
                cell.set_bg(tint_rgb(cell.bg, color, strength * 0.5));
            }
        }
    }
}
