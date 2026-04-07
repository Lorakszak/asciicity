pub mod campfire;

/// Scene description for --list output.
pub struct SceneInfo {
    pub name: &'static str,
    pub description: &'static str,
}

pub const SCENES: &[SceneInfo] = &[
    SceneInfo {
        name: "campfire",
        description: "A knight resting by a campfire in the wilderness",
    },
];
