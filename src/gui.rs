use crate::game::GameContext;
use crate::typing::{CharacterState, TypingSession};
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::prelude::RaylibDrawHandle;

pub(crate) fn draw_stack_selection(
    game: &GameContext,
    drawing: &mut RaylibDrawHandle,
    input: &str,
    error: &Option<String>,
) {
    drawing.draw_text(
        "Choose a stack: type its top word, then press Enter or Space",
        50,
        65,
        22,
        Color::BLACK,
    );
    for (index, stack) in game.desktop.stacks.iter().enumerate() {
        let x = 50 + index as i32 * 360;
        drawing.draw_rectangle_lines(x, 120, 330, 170, Color::DARKGRAY);
        drawing.draw_text(
            &format!("STACK {}", index + 1),
            x + 20,
            140,
            18,
            Color::DARKGRAY,
        );
        match stack.top_word() {
            Some(word) => {
                drawing.draw_text(word, x + 20, 180, 26, Color::BLACK);
                drawing.draw_text(
                    &format!("{} leaves remaining", stack.len()),
                    x + 20,
                    235,
                    18,
                    Color::DARKGRAY,
                );
            }
            None => {
                drawing.draw_text("EMPTY", x + 20, 180, 26, Color::GRAY);
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
    drawing.draw_text(
        "Backspace edits the selection.",
        50,
        650,
        18,
        Color::DARKGRAY,
    );
}

pub(crate) fn draw_leaf_typing(drawing: &mut RaylibDrawHandle, typing: &TypingSession) {
    drawing.draw_text("Transcribe the page exactly:", 50, 65, 22, Color::BLACK);
    draw_typing_session(typing, drawing, 50, 110, 1100);
    drawing.draw_text(
        &format!(
            "{}/{} characters",
            typing.typed_characters(),
            typing.character_count()
        ),
        50,
        620,
        20,
        Color::DARKGRAY,
    );
    if typing.is_complete() && !typing.is_perfect() {
        drawing.draw_text(
            "Fix the red characters with Backspace to save this page.",
            50,
            650,
            18,
            Color::RED,
        );
    } else {
        drawing.draw_text(
            "Backspace removes the last character.",
            50,
            650,
            18,
            Color::DARKGRAY,
        );
    }
}

pub(crate) fn draw_stats(game: &GameContext, drawing: &mut RaylibDrawHandle) {
    drawing.draw_text(
        &format!("Saved pages: {}", game.desktop.finished_leaves.len()),
        50,
        25,
        20,
        Color::DARKGRAY,
    );
}

pub(crate) fn draw_typing_session(
    typing_session: &TypingSession,
    drawing: &mut RaylibDrawHandle,
    x: i32,
    y: i32,
    max_width: i32,
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
        drawing.draw_text(&character.to_string(), char_x, char_y, FONT_SIZE, color);
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
