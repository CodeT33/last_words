pub mod consts;
pub mod desktop;
pub mod game;
pub mod gui;
pub mod leaf_pool;
pub mod leaves;
pub mod player;
pub mod typing;

use crate::desktop::Desktop;
use crate::game::{GameContext, GameState};
use crate::gui::{draw_leaf_typing, draw_stack_selection, draw_stats};
use crate::leaf_pool::LeafPool;
use raylib::prelude::*;

fn main() {
    let leaf_pool = LeafPool::from_path("res/leaves/leaves1.md").unwrap();
    let desktop = Desktop::new(&leaf_pool, 3, 2);
    let mut game = GameContext::new(desktop);

    let (mut raylib_handle, thread) = init().size(1200, 700).title("Last Words").build();

    while !raylib_handle.window_should_close() {
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

        let mut drawing = raylib_handle.begin_drawing(&thread);
        drawing.clear_background(Color::WHITE);
        draw_stats(&game, &mut drawing);

        match &game.game_state {
            GameState::SelectStack { input, error } => {
                draw_stack_selection(&game, &mut drawing, input, error);
            }

            GameState::WriteLeaf { typing, .. } => {
                draw_leaf_typing(&mut drawing, typing);
            }
        }
    }
}
