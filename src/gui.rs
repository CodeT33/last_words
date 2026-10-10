use crate::fonts::TextDrawer;
use crate::game::{GameContext, GameState};
use crate::typing::{CharacterState, TypingSession};
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, Vector2};
use raylib::{RaylibHandle, RaylibThread};

pub(crate) unsafe fn draw_ui(
    raylib_handle: &mut RaylibHandle,
    thread: &RaylibThread,
    game: &mut GameContext,
    text_drawer: &TextDrawer,
) {
    let mut drawing = raylib_handle.begin_drawing(&thread);
    drawing.clear_background(Color::WHITE);
    draw_stats(&game, &mut drawing, &text_drawer);

    match &game.game_state {
        GameState::SelectStack { input, error } => {
            draw_stack_selection(&game, &mut drawing, input, error, &text_drawer);
        }

        GameState::WriteLeaf { typing, .. } => {
            draw_leaf_typing(&mut drawing, typing, &text_drawer);
        }
    }
}

pub(crate) fn draw_stack_selection(
    game: &GameContext,
    drawing: &mut RaylibDrawHandle,
    input: &str,
    error: &Option<String>,
    text_drawer: &TextDrawer,
) {
    text_drawer.draw_text_body(
        drawing,
        "Choose a stack: type its top word, then press Enter or Space",
        Vector2::new(50.0, 65.0),
        Color::BLACK,
    );

    for (index, stack) in game.desktop.stacks.iter().enumerate() {
        let x = 50 + index as i32 * 360;
        drawing.draw_rectangle_lines(x, 120, 330, 170, Color::DARKGRAY);
        text_drawer.draw_text_title(
            drawing,
            &format!("STACK {}", index + 1),
            Vector2::new((x + 20) as f32, 140.0),
        );
        match stack.top_word() {
            Some(word) => {
                text_drawer.draw_text_body(
                    drawing,
                    word,
                    Vector2::new((x + 20) as f32, 180.0),
                    Color::BLACK,
                );
                text_drawer.draw_text_info(
                    drawing,
                    &format!("{} leaves remaining", stack.len()),
                    Vector2::new((x + 20) as f32, 235.0),
                );
            }
            None => {
                text_drawer.draw_text_info(drawing, "EMPTY", Vector2::new((x + 20) as f32, 180.0));
            }
        }
    }

    let selection_color = if (0..game.desktop.stacks.len())
        .any(|index| !input.is_empty() && game.selection_matches_stack(index))
    {
        Color::BLACK
    } else if input.is_empty() {
        Color::BLACK
    } else {
        Color::RED
    };
    drawing.draw_text(&format!("Selection: {input}"), 50, 340, 24, selection_color);
    if let Some(error) = error {
        drawing.draw_text(error, 50, 390, 20, Color::RED);
    }
}

pub(crate) fn draw_leaf_typing(
    drawing: &mut RaylibDrawHandle,
    typing: &TypingSession,
    text_drawer: &TextDrawer,
) {
    text_drawer.draw_text_title(
        drawing,
        "Transcribe the page exactly:",
        Vector2::new(50.0, 65.0),
    );
    draw_typing_session(typing, drawing, 50, 110, 1100, text_drawer);
    text_drawer.draw_text_info(drawing, &format!("{}/{} characters", typing.typed_characters(), typing.character_count()), Vector2::new(50.0, 620.0));
}

pub(crate) fn draw_stats(
    game: &GameContext,
    drawing: &mut RaylibDrawHandle,
    text_drawer: &TextDrawer,
) {
    text_drawer.draw_text_info(drawing, &format!("Saved leaves: {}", game.desktop.finished_leaves.len()), Vector2::new(50.0, 25.0));
}

pub(crate) fn draw_typing_session(
    typing_session: &TypingSession,
    drawing: &mut RaylibDrawHandle,
    x: i32,
    y: i32,
    max_width: i32,
    text_drawer: &TextDrawer,
) {
    const FONT_SIZE: i32 = 24;
    const CELL_WIDTH: i32 = 18;
    const LINE_HEIGHT: i32 = 36;

    let characters_per_line = (max_width / CELL_WIDTH).max(1) as usize;
    let mut cursor_position = (x, y);
    let mut line = 0;
    let mut column = 0;

    for (index, character) in typing_session.target.iter().enumerate() {
        if index == typing_session.cursor {
            cursor_position = (x + column as i32 * CELL_WIDTH, y + line * LINE_HEIGHT);
        }
        if *character == '\n' {
            line += 1;
            column = 0;
            continue;
        }

        let char_x = x + column as i32 * CELL_WIDTH;
        let char_y = y + line * LINE_HEIGHT;
        let color = match typing_session.states[index] {
            CharacterState::Untyped => Color::BLACK,
            CharacterState::Correct => Color::GRAY,
            CharacterState::Incorrect => Color::RED,
        };
        text_drawer.draw_text_body(drawing, &character.to_string(), Vector2::new(char_x as f32, char_y as f32), color);
        column += 1;
        if column >= characters_per_line {
            line += 1;
            column = 0;
        }
    }

    if typing_session.cursor == typing_session.target.len() {
        cursor_position = (x + column as i32 * CELL_WIDTH, y + line * LINE_HEIGHT);
    }
    drawing.draw_line(
        cursor_position.0,
        cursor_position.1 + FONT_SIZE + 2,
        cursor_position.0 + CELL_WIDTH - 2,
        cursor_position.1 + FONT_SIZE + 2,
        Color::DARKGRAY,
    );
}
