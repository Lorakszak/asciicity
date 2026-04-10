pub mod cityscape;

/// Scene description for --list output.
pub struct SceneInfo {
    pub name: &'static str,
    pub description: &'static str,
}

pub const SCENES: &[SceneInfo] = &[SceneInfo {
    name: "cityscape",
    description: "Rooftop view of a city skyline at night",
}];
