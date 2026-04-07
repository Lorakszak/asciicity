use std::path::PathBuf;

use crate::color::{self, ColorMap};

/// Art data loaded for an entity: animation frames + optional color mapping.
pub struct ArtData {
    pub frames: Vec<String>,
    #[allow(dead_code)]
    pub colors: Option<ColorMap>,
}

impl ArtData {
    /// Get the first frame, or empty string if no frames exist.
    pub fn first_frame(&self) -> &str {
        self.frames.first().map(|s| s.as_str()).unwrap_or("")
    }
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
/// Checks ~/.config/bootiful/scenes/ for user overrides (art, colors, colormap),
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
            load_override_file(scene, name, "colors")
                .and_then(|c| color::parse_palette(&c))
        });

    ArtData { frames, colors }
}

fn load_override_file(scene: &str, name: &str, ext: &str) -> Option<String> {
    let path = config_dir()?
        .join("scenes")
        .join(scene)
        .join(format!("{name}.{ext}"));
    std::fs::read_to_string(path).ok()
}

fn config_dir() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        Some(PathBuf::from(xdg).join("bootiful"))
    } else if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home).join(".config").join("bootiful"))
    } else {
        None
    }
}
