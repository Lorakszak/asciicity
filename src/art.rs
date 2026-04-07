use std::path::PathBuf;

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

/// Load art frames for a scene asset.
/// Checks ~/.config/bootiful/scenes/<scene>/<name>.txt for user override,
/// falls back to the compiled-in default if override is missing or empty.
pub fn load(scene: &str, name: &str, default: &str) -> Vec<String> {
    if let Some(content) = load_override(scene, name) {
        let frames = parse_frames(&content);
        if !frames.is_empty() {
            return frames;
        }
    }
    parse_frames(default)
}

fn load_override(scene: &str, name: &str) -> Option<String> {
    let path = config_dir()?.join("scenes").join(scene).join(format!("{name}.txt"));
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
