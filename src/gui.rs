use crate::game::{GameContext, GameState};
use crate::text_renderer::{FontsHandle, StyleSettings, TextStyle, draw_styled_text};
use crate::typing::{CharacterState, TypingSession};
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, Vector2};
use raylib::{RaylibHandle, RaylibThread};

pub(crate) unsafe fn draw_ui(
    raylib_handle: &mut RaylibHandle,
    thread: &RaylibThread,
    game: &mut GameContext,
    fonts: &FontsHandle,
) {
    let mut drawing = raylib_handle.begin_drawing(&thread);
    drawing.clear_background(Color::WHITE);
    draw_stats(&game, &mut drawing, &fonts);

    match &game.game_state {
        GameState::SelectStack { input, error } => {
            draw_stack_selection(&game, &mut drawing, &fonts, input, error);
        }

        GameState::WriteLeaf { typing, .. } => {
            draw_leaf_typing(&mut drawing, &fonts, typing);
        }
    }
}

pub(crate) fn draw_stack_selection(
    game: &GameContext,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    input: &str,
    error: &Option<String>,
) {
    draw_styled_text(
        drawing,
        fonts,
        "Choose a stack: type its top word, then press Enter or Space",
        Vector2::new(50.0, 65.0),
        TextStyle::Body,
    );

    for (index, stack) in game.desktop.stacks.iter().enumerate() {
        let x = 50 + index as i32 * 360;
        drawing.draw_rectangle_lines(x, 120, 330, 170, Color::DARKGRAY);
        draw_styled_text(
            drawing,
            fonts,
            &format!("Stack {}", index + 1),
            Vector2::new((x + 20) as f32, 140.0),
            TextStyle::Title,
        );
        match stack.top_word() {
            Some(word) => {
                draw_styled_text(
                    drawing,
                    fonts,
                    word,
                    Vector2::new((x + 20) as f32, 180.0),
                    TextStyle::Body,
                );
                draw_styled_text(
                    drawing,
                    fonts,
                    &format!("{} leaves remaining", stack.len()),
                    Vector2::new((x + 20) as f32, 235.0),
                    TextStyle::Info,
                );
            }
            None => draw_styled_text(
                drawing,
                fonts,
                "EMPTY",
                Vector2::new((x + 20) as f32, 180.0),
                TextStyle::Info,
            ),
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
    let body_settings = TextStyle::Body.get_settings(fonts);
    draw_styled_text(
        drawing,
        fonts,
        &format!("Selection: {input}"),
        Vector2::new(50.0, 340.0),
        TextStyle::Custom(StyleSettings {
            color: selection_color,
            font: body_settings.font,
            font_size: body_settings.font_size,
            spacing: body_settings.spacing,
        }),
    );
    if let Some(error) = error {
        draw_styled_text(
            drawing,
            fonts,
            error,
            Vector2::new(50.0, 390.0),
            TextStyle::Error,
        );
    }
}

pub(crate) fn draw_leaf_typing(
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    typing: &TypingSession,
) {
    draw_styled_text(
        drawing,
        fonts,
        "Transcribe the page:",
        Vector2::new(50.0, 65.0),
        TextStyle::Title,
    );
    draw_typing_session(typing, drawing, fonts, 50, 110, 1100);
    draw_styled_text(
        drawing,
        fonts,
        &format!(
            "{}/{} characters",
            typing.typed_characters(),
            typing.character_count()
        ),
        Vector2::new(50.0, 620.0),
        TextStyle::Info,
    );
}

pub(crate) fn draw_stats(game: &GameContext, drawing: &mut RaylibDrawHandle, fonts: &FontsHandle) {
    draw_styled_text(
        drawing,
        fonts,
        &format!("Saved leaves: {}", game.desktop.finished_leaves.len()),
        Vector2::new(50.0, 25.0),
        TextStyle::Info,
    );
}

pub(crate) fn draw_typing_session(
    typing_session: &TypingSession,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
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
        let body_settings = TextStyle::Body.get_settings(fonts);
        draw_styled_text(
            drawing,
            &fonts,
            &character.to_string(),
            Vector2::new(char_x as f32, char_y as f32),
            TextStyle::Custom(StyleSettings {
                color,
                font: body_settings.font,
                font_size: body_settings.font_size,
                spacing: body_settings.spacing,
            }),
        );
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
        cursor_position.1 + FONT_SIZE + 8,
        cursor_position.0 + CELL_WIDTH - 2,
        cursor_position.1 + FONT_SIZE + 8,
        Color::DARKGRAY,
    );
}
