use std::path::PathBuf;

use crate::color::{self, ColorMap};

/// Maximum size of a single art/colors override file on disk. ASCII art is
/// tiny; anything bigger is almost certainly a mistake or a hostile file and
/// we refuse to slurp it into memory.
const MAX_OVERRIDE_BYTES: u64 = 1024 * 1024;

/// Art data loaded for an entity: animation frames + optional color mapping.
pub struct ArtData {
    pub frames: Vec<String>,
    pub colors: Option<ColorMap>,
}

/// Parse a multi-frame art string into individual frames.
/// Frames are separated by a line containing only "---".
pub fn parse_frames(content: &str) -> Vec<String> {
    let content = content.strip_suffix('\n').unwrap_or(content);
    content
        .split("\n---\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Load art data for a scene asset.
/// Checks ~/.config/terminart/scenes/ for user overrides (art, colors, colormap),
/// falls back to compiled-in defaults.
pub fn load(scene: &str, name: &str, default_art: &str) -> ArtData {
    // Load art frames
    let frames = if let Some(content) = load_override_file(scene, name, "txt") {
        let f = parse_frames(&content);
        if f.is_empty() {
            parse_frames(default_art)
        } else {
            f
        }
    } else {
        parse_frames(default_art)
    };

    // Load colors: .colormap (positional) takes priority over .colors (char-based)
    let colors = load_override_file(scene, name, "colormap")
        .and_then(|c| color::parse_colormap(&c))
        .or_else(|| {
            load_override_file(scene, name, "colors").and_then(|c| color::parse_palette(&c))
        });

    ArtData { frames, colors }
}

/// A safe path component is non-empty, contains no path separators, no NUL,
/// is not a parent/current-directory alias, and is plain ASCII-ish. This is
/// the last line of defence against a malicious scene/name looking up files
/// outside `~/.config/terminart/scenes/`.
fn is_safe_component(s: &str) -> bool {
    if s.is_empty() || s.len() > 64 {
        return false;
    }
    if s == "." || s == ".." {
        return false;
    }
    !s.chars()
        .any(|c| c == '/' || c == '\\' || c == '\0' || c == ':' || c.is_control())
}

fn load_override_file(scene: &str, name: &str, ext: &str) -> Option<String> {
    if !is_safe_component(scene) || !is_safe_component(name) || !is_safe_component(ext) {
        return None;
    }
    let path = config_dir()?
        .join("scenes")
        .join(scene)
        .join(format!("{name}.{ext}"));

    // Cap the read at MAX_OVERRIDE_BYTES so a hostile or accidental huge file
    // cannot exhaust memory.
    let meta = std::fs::metadata(&path).ok()?;
    if !meta.is_file() || meta.len() > MAX_OVERRIDE_BYTES {
        return None;
    }
    std::fs::read_to_string(&path).ok()
}

/// Mirror ASCII art horizontally (flip left/right).
/// Pads lines to equal width, reverses characters, and swaps mirror pairs.
pub fn mirror_horizontal(art: &str) -> String {
    let lines: Vec<&str> = art.lines().collect();
    let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    lines
        .iter()
        .map(|line| {
            let line_width = line.chars().count();
            let pad = max_width.saturating_sub(line_width);
            let padded: String = line.chars().chain(std::iter::repeat_n(' ', pad)).collect();
            padded
                .chars()
                .rev()
                .map(|c| match c {
                    '/' => '\\',
                    '\\' => '/',
                    '(' => ')',
                    ')' => '(',
                    '<' => '>',
                    '>' => '<',
                    '[' => ']',
                    ']' => '[',
                    '{' => '}',
                    '}' => '{',
                    _ => c,
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Mirror all frames in an ArtData (for entities going the opposite direction).
pub fn mirror_frames(frames: &[String]) -> Vec<String> {
    frames.iter().map(|f| mirror_horizontal(f)).collect()
}

fn config_dir() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        Some(PathBuf::from(xdg).join("asciicity"))
    } else if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home).join(".config").join("asciicity"))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_component_rejects_traversal() {
        assert!(!is_safe_component(""));
        assert!(!is_safe_component("."));
        assert!(!is_safe_component(".."));
        assert!(!is_safe_component("../etc"));
        assert!(!is_safe_component("a/b"));
        assert!(!is_safe_component("a\\b"));
        assert!(!is_safe_component("a\0b"));
        assert!(!is_safe_component("a:b"));
        assert!(!is_safe_component(&"x".repeat(65)));
    }

    #[test]
    fn safe_component_accepts_normal_names() {
        assert!(is_safe_component("cityscape"));
        assert!(is_safe_component("cloud_small"));
        assert!(is_safe_component("plane3"));
        assert!(is_safe_component("txt"));
        assert!(is_safe_component("colormap"));
    }

    #[test]
    fn mirror_horizontal_swaps_brackets() {
        assert_eq!(mirror_horizontal("</>"), "<\\>");
        assert_eq!(mirror_horizontal("(x)"), "(x)");
        assert_eq!(mirror_horizontal("[ab]"), "[ba]");
    }

    #[test]
    fn parse_frames_handles_multiframe() {
        let s = "frame1\n---\nframe2\n---\nframe3\n";
        assert_eq!(parse_frames(s), vec!["frame1", "frame2", "frame3"]);
    }

    #[test]
    fn parse_frames_handles_single_frame() {
        assert_eq!(parse_frames("hello"), vec!["hello"]);
    }
}
