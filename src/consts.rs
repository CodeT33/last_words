pub(crate) const MAX_WORDS_PER_LEAF: usize = 64;
pub(crate) const CURSOR_SPEED: f32 = 8.0;
pub(crate) const LINE_HEIGHT: i32 = 32;
pub(crate) const COUNTDOWN_IN_SECONDS: u32 = 30;
pub(crate) const TILE_SIZE: f32 = 64.0;
pub(crate) const NUMBER_OF_STACKS: u32 = 4;
pub(crate) const MAX_STACK_HEIGHT: u32 = 8;

pub(crate) const WINDOW_SIZE: (i32, i32) = ((TILE_SIZE * 9.0) as i32, (TILE_SIZE * 13.0) as i32);

pub(crate) const LEAF_WRITING_WIDTH: f32 = TILE_SIZE * 6.0;

pub mod ui_positions {
    use crate::consts::TILE_SIZE;
    use raylib::ffi::Vector2;

    pub(crate) const COUNTDOWN: Vector2 = Vector2 {
        x: TILE_SIZE,
        y: 42.0,
    };
    pub(crate) const TOTAL_POINTS: Vector2 = Vector2 {
        x: TILE_SIZE * 3.5,
        y: 42.0,
    };
    pub(crate) const STATS: Vector2 = Vector2 {
        x: TILE_SIZE * 6.2,
        y: TILE_SIZE * 2.5,
    };
    pub(crate) const LEAF_WRITING: Vector2 = Vector2 {
        x: TILE_SIZE * 1.5,
        y: TILE_SIZE * 4.5,
    };
    pub(crate) const STACK_SIGN_A: Vector2 = Vector2 {
        x: TILE_SIZE * 1.5,
        y: TILE_SIZE * 7.8,
    };
    pub(crate) const STACK_SIGN_B: Vector2 = Vector2 {
        x: TILE_SIZE * 3.0,
        y: TILE_SIZE * 7.6,
    };
    pub(crate) const STACK_SIGN_C: Vector2 = Vector2 {
        x: TILE_SIZE * 5.0,
        y: TILE_SIZE * 8.0,
    };
    pub(crate) const STACK_SIGN_D: Vector2 = Vector2 {
        x: TILE_SIZE * 6.5,
        y: TILE_SIZE * 7.8,
    };
    pub(crate) const STACK_A: Vector2 = Vector2 {
        x: TILE_SIZE * 0.75,
        y: TILE_SIZE * 10.0,
    };
    pub(crate) const STACK_B: Vector2 = Vector2 {
        x: TILE_SIZE * 0.75,
        y: TILE_SIZE * 11.5,
    };
    pub(crate) const STACK_C: Vector2 = Vector2 {
        x: TILE_SIZE * 5.25,
        y: TILE_SIZE * 10.0,
    };
    pub(crate) const STACK_D: Vector2 = Vector2 {
        x: TILE_SIZE * 5.25,
        y: TILE_SIZE * 11.5,
    };
    pub(crate) const TOTAL_POINTS_SIGN: Vector2 = Vector2 {
        x: TILE_SIZE * 2.7,
        y: TILE_SIZE * 2.5,
    };
    pub(crate) const TOTAL_POINTS_CALCULATION: Vector2 = Vector2 {
        x: TILE_SIZE * 1.0,
        y: TILE_SIZE * 10.7,
    };
}
