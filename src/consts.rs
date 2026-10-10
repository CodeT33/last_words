pub(crate) const MAX_WORDS_PER_LEAF: usize = 64;
pub(crate) const CURSOR_SPEED: f32 = 8.0;
pub(crate) const LINE_HEIGHT: i32 = 32;
pub(crate) const COUNTDOWN_IN_SECONDS: u32 = 60;
pub(crate) const TILE_SIZE: u32 = 64;

pub mod ui_positions {
    use crate::consts::{LINE_HEIGHT, TILE_SIZE};
    use raylib::ffi::Vector2;

    pub(crate) const COUNTDOWN: Vector2 = Vector2 {
        x: TILE_SIZE as f32,
        y: (TILE_SIZE as i32 - LINE_HEIGHT) as f32,
    };
    pub(crate) const TOTAL_POINTS: Vector2 = Vector2 {
        x: TILE_SIZE as f32 * 3.5,
        y: (TILE_SIZE as i32 - LINE_HEIGHT) as f32,
    };
    pub(crate) const STATS: Vector2 = Vector2 {
        x: TILE_SIZE as f32 * 6.2,
        y: TILE_SIZE as f32 * 2.5,
    };
}
