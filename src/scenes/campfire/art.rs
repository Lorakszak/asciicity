/// Characters used for twinkling stars
pub const STAR_CHARS: &[char] = &['.', '*', '+', '`'];

/// Ground texture character
pub const GROUND_CHAR: char = '.';

/// Simple tree silhouettes (small, medium)
pub const TREE_SMALL: &str = r#"
  ^
 /|\
 /|\
  |
"#;

pub const TREE_MEDIUM: &str = r#"
   ^
  /|\
 /|+|\
 /|+|\
  |||
   |
"#;

/// Knight sitting, facing right toward fire
pub const KNIGHT: &str = r#"
  ,O
  |]_
  /|
 / |
"#;

/// Knight breathing frame (shoulders slightly raised)
pub const KNIGHT_BREATHE: &str = r#"
  ,O
  |]'
  /|
 / |
"#;

/// Fire frames (3 frames for animation)
pub const FIRE_FRAMES: &[&str] = &[
    r#"
 (
(*)
_|_
"#,
    r#"

(^)
_|_
"#,
    r#"
 )
(*')
_|_
"#,
];

/// Smoke particle characters
pub const SMOKE_CHARS: &[char] = &['~', '.', '\'', ','];
