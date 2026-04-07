use std::collections::HashMap;

use ratatui::style::Color;

/// Character-based or positional color mapping for art.
#[derive(Clone)]
pub enum ColorMap {
    /// Same character always gets same color, regardless of position.
    Palette(HashMap<char, Color>),
    /// Per-cell color using a palette of named indices + positional grid.
    Grid {
        palette: HashMap<char, Color>,
        map: Vec<Vec<Option<char>>>,
    },
}

impl ColorMap {
    /// Get the foreground color for a character at a given row/col.
    /// Returns None if no color is specified (caller should use base style).
    pub fn get_color(&self, ch: char, row: usize, col: usize) -> Option<Color> {
        match self {
            ColorMap::Palette(p) => p.get(&ch).copied(),
            ColorMap::Grid { palette, map } => map
                .get(row)
                .and_then(|r| r.get(col))
                .copied()
                .flatten()
                .and_then(|idx| palette.get(&idx).copied()),
        }
    }
}

/// Parse a .colors file (character-based palette).
///
/// Format: one `<char> #RRGGBB` per line. Lines starting with # are comments.
/// ```text
/// # head
/// O #FFD700
/// , #CCCCCC
/// ```
pub fn parse_palette(content: &str) -> Option<ColorMap> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut chars = line.chars();
        let ch = chars.next()?;
        let rest = chars.as_str().trim();
        let color = parse_hex_color(rest)?;
        map.insert(ch, color);
    }
    if map.is_empty() {
        None
    } else {
        Some(ColorMap::Palette(map))
    }
}

/// Parse a .colormap file (positional grid with palette header).
///
/// Format:
/// ```text
/// @palette
/// a #FFD700
/// b #808080
///
/// @map
///   ab
///   bab
/// ```
pub fn parse_colormap(content: &str) -> Option<ColorMap> {
    let mut palette = HashMap::new();
    let mut grid = Vec::new();
    let mut in_palette = false;
    let mut in_map = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "@palette" {
            in_palette = true;
            in_map = false;
            continue;
        }
        if trimmed == "@map" {
            in_palette = false;
            in_map = true;
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if in_palette {
            let mut chars = trimmed.chars();
            let key = chars.next()?;
            let rest = chars.as_str().trim();
            let color = parse_hex_color(rest)?;
            palette.insert(key, color);
        } else if in_map {
            // Use raw line (not trimmed) to preserve leading spaces for alignment
            let row: Vec<Option<char>> = line.chars().map(|c| {
                if c == ' ' { None } else { Some(c) }
            }).collect();
            grid.push(row);
        }
    }

    if palette.is_empty() || grid.is_empty() {
        None
    } else {
        Some(ColorMap::Grid { palette, map: grid })
    }
}

/// Parse a hex color string like "#FF8000" into Color::Rgb.
pub fn parse_hex_color(s: &str) -> Option<Color> {
    let s = s.strip_prefix('#')?;
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

/// Linearly interpolate between two RGB colors.
/// t=0.0 returns a, t=1.0 returns b. Non-RGB colors return a unchanged.
pub fn lerp_rgb(a: Color, b: Color, t: f64) -> Color {
    match (a, b) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let t = t.clamp(0.0, 1.0);
            Color::Rgb(
                (r1 as f64 + (r2 as f64 - r1 as f64) * t) as u8,
                (g1 as f64 + (g2 as f64 - g1 as f64) * t) as u8,
                (b1 as f64 + (b2 as f64 - b1 as f64) * t) as u8,
            )
        }
        _ => a,
    }
}

/// Tint a color toward a target color by a given strength (0.0 to 1.0).
pub fn tint_rgb(base: Color, tint: Color, strength: f64) -> Color {
    lerp_rgb(base, tint, strength)
}

/// Fade (darken) a color by a factor. 0.0 = black, 1.0 = unchanged.
pub fn fade_rgb(color: Color, factor: f64) -> Color {
    match color {
        Color::Rgb(r, g, b) => {
            let f = factor.clamp(0.0, 1.0);
            Color::Rgb(
                (r as f64 * f) as u8,
                (g as f64 * f) as u8,
                (b as f64 * f) as u8,
            )
        }
        _ => color,
    }
}
