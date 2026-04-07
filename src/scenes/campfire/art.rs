// Embedded defaults - baked into binary at compile time from assets/ files.
// Users can override these by placing files in ~/.config/bootiful/scenes/campfire/
pub const KNIGHT_DEFAULT: &str = include_str!("../../../assets/campfire/knight.txt");
pub const FIRE_DEFAULT: &str = include_str!("../../../assets/campfire/fire.txt");
pub const TREE_SMALL_DEFAULT: &str = include_str!("../../../assets/campfire/tree_small.txt");
pub const TREE_MEDIUM_DEFAULT: &str = include_str!("../../../assets/campfire/tree_medium.txt");

// Simple character sets - not worth externalizing
pub const STAR_CHARS: &[char] = &['.', '*', '+', '`'];
pub const GROUND_CHAR: char = '.';
pub const SMOKE_CHARS: &[char] = &['~', '.', '\'', ','];
