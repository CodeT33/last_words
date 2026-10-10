pub mod consts;
pub mod desktop;
pub mod game;
pub mod gui;
pub mod leaf_pool;
pub mod leaves;
pub mod player;
pub mod text_renderer;
pub mod timer;
pub mod typing;

use crate::desktop::Desktop;
use crate::game::GameContext;
use crate::gui::draw_ui;
use crate::leaf_pool::LeafPool;
use crate::text_renderer::FontsHandle;
use raylib::prelude::*;

fn main() {
    let leaf_pool = LeafPool::from_path("res/leaves/rational_gaze.md").unwrap();
    let desktop = Desktop::new(&leaf_pool, 3, 8);
    let mut game = GameContext::new(desktop);

    let (mut raylib_handle, thread) = init().size(1200, 700).title("Last Words").build();
    let fonts = FontsHandle::load(&mut raylib_handle, &thread);

    while !raylib_handle.window_should_close() {
        game.update(raylib_handle.get_frame_time());

        let mut typed_characters = Vec::new();
        while let Some(character) = raylib_handle.get_char_pressed() {
            typed_characters.push(character);
        }
        let mut state_changed = false;
        for character in typed_characters {
            if game.type_character(character) {
                state_changed = true;
                break;
            }
        }
        if !state_changed {
            if raylib_handle.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                game.backspace();
            }
            if raylib_handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
                game.press_enter();
            }
        }

        unsafe {
            draw_ui(&mut raylib_handle, &thread, &mut game, &fonts);
        }
    }
}
