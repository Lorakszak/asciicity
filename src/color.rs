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
        let Some(ch) = chars.next() else { continue };
        let rest = chars.as_str().trim();
        if let Some(color) = parse_hex_color(rest) {
            map.insert(ch, color);
        }
        // Silently skip malformed lines: one bad line should not discard
        // the entire palette. User-authored files should be forgiving.
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
            let Some(key) = chars.next() else { continue };
            let rest = chars.as_str().trim();
            if let Some(color) = parse_hex_color(rest) {
                palette.insert(key, color);
            }
        } else if in_map {
            // Use raw line (not trimmed) to preserve leading spaces for alignment
            let row: Vec<Option<char>> = line
                .chars()
                .map(|c| if c == ' ' { None } else { Some(c) })
                .collect();
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
    // `len()` counts bytes; require pure ASCII so direct byte slicing at
    // 0..2 / 2..4 / 4..6 lands on char boundaries. Without the ASCII check,
    // a 6-byte string like "aéaa" would panic inside the slice expressions.
    if s.len() != 6 || !s.is_ascii() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_color_valid() {
        assert_eq!(parse_hex_color("#FF8000"), Some(Color::Rgb(255, 128, 0)));
        assert_eq!(parse_hex_color("#000000"), Some(Color::Rgb(0, 0, 0)));
    }

    #[test]
    fn parse_hex_color_rejects_bad_input() {
        assert_eq!(parse_hex_color("FF8000"), None); // missing #
        assert_eq!(parse_hex_color("#FF80"), None); // too short
        assert_eq!(parse_hex_color("#GG8000"), None); // non-hex
        assert_eq!(parse_hex_color("#FF800000"), None); // too long
    }

    #[test]
    fn parse_hex_color_rejects_multibyte_chars() {
        // 6 bytes but contains a 2-byte char; naive byte slicing would panic.
        assert_eq!(parse_hex_color("#aéaa"), None);
    }

    #[test]
    fn parse_palette_skips_bad_lines() {
        let content = "O #FFD700\nX nothex\n# a comment\n, #CCCCCC\n";
        let map = parse_palette(content).expect("valid lines should still parse");
        match map {
            ColorMap::Palette(p) => {
                assert_eq!(p.get(&'O'), Some(&Color::Rgb(0xFF, 0xD7, 0x00)));
                assert_eq!(p.get(&','), Some(&Color::Rgb(0xCC, 0xCC, 0xCC)));
                assert!(!p.contains_key(&'X'));
            }
            _ => panic!("expected palette variant"),
        }
    }

    #[test]
    fn parse_palette_all_bad_returns_none() {
        assert!(parse_palette("nothing valid\n# just comments\n").is_none());
    }

    #[test]
    fn lerp_rgb_endpoints_and_midpoint() {
        let a = Color::Rgb(0, 0, 0);
        let b = Color::Rgb(100, 200, 50);
        assert_eq!(lerp_rgb(a, b, 0.0), a);
        assert_eq!(lerp_rgb(a, b, 1.0), b);
        assert_eq!(lerp_rgb(a, b, 0.5), Color::Rgb(50, 100, 25));
    }

    #[test]
    fn lerp_rgb_clamps_out_of_range_t() {
        let a = Color::Rgb(0, 0, 0);
        let b = Color::Rgb(100, 100, 100);
        assert_eq!(lerp_rgb(a, b, -10.0), a);
        assert_eq!(lerp_rgb(a, b, 10.0), b);
    }

    #[test]
    fn fade_rgb_clamps_factor() {
        let c = Color::Rgb(200, 100, 50);
        assert_eq!(fade_rgb(c, 0.0), Color::Rgb(0, 0, 0));
        assert_eq!(fade_rgb(c, 1.0), c);
        assert_eq!(fade_rgb(c, -5.0), Color::Rgb(0, 0, 0));
        assert_eq!(fade_rgb(c, 5.0), c);
    }
}
