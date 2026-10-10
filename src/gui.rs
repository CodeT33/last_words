use crate::consts::LINE_HEIGHT;
use crate::desktop::Desktop;
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
    let frame_time = raylib_handle.get_frame_time();
    let mut drawing = raylib_handle.begin_drawing(thread);
    drawing.clear_background(Color::WHITE);
    draw_stats(game, &mut drawing, fonts);

    let selection_matches_stack =
        (0..game.desktop.stacks.len()).any(|index| game.selection_matches_stack(index));
    let desktop = &game.desktop;
    match &mut game.game_state {
        GameState::SelectStack { input } => {
            draw_stack_selection(desktop, &mut drawing, fonts, input, selection_matches_stack);
        }

        GameState::WriteLeaf { typing, .. } => {
            draw_leaf_typing(&mut drawing, fonts, typing, frame_time);
        }
    }
}

pub(crate) fn draw_stack_selection(
    desktop: &Desktop,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    input: &str,
    selection_matches_stack: bool,
) {
    draw_styled_text(
        drawing,
        fonts,
        "Type a stack's word to select it.",
        Vector2::new(50.0, 65.0),
        TextStyle::Body,
    );

    for (index, stack) in desktop.stacks.iter().enumerate() {
        let x = 50 + index as i32 * 360;
        drawing.draw_rectangle_lines(x, 120, 330, 170, Color::DARKGRAY);
        match stack.top_word() {
            Some(word) => {
                draw_stack_word(
                    drawing,
                    fonts,
                    word,
                    input,
                    input_matches_word(word, input),
                    x + 20,
                    180,
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
}

fn input_matches_word(word: &str, input: &str) -> bool {
    let mut word_characters = word.chars();
    input.chars().all(|input_character| {
        word_characters
            .next()
            .map(|word_character| word_character.eq_ignore_ascii_case(&input_character))
            .unwrap_or(false)
    })
}

fn draw_stack_word(
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    word: &str,
    input: &str,
    is_matching: bool,
    x: i32,
    y: i32,
) {
    let body = TextStyle::Body.get_settings(fonts);
    let highlighted_characters = if is_matching {
        input.chars().count()
    } else {
        0
    };
    let mut char_x = x as f32;
    let fading_word = format!("{}...", &word);

    for (index, character) in fading_word.chars().enumerate() {
        let text = character.to_string();
        draw_styled_text(
            drawing,
            fonts,
            &text,
            Vector2::new(char_x, y as f32),
            TextStyle::Custom(StyleSettings {
                color: if index < highlighted_characters {
                    Color::BLACK
                } else {
                    Color::GRAY
                },
                ..body
            }),
        );
        char_x += drawing.measure_text(&text, body.font_size as i32) as f32;
    }
}

pub(crate) fn draw_leaf_typing(
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    typing: &mut TypingSession,
    frame_time: f32,
) {
    draw_styled_text(
        drawing,
        fonts,
        "Transcribe the page:",
        Vector2::new(50.0, 65.0),
        TextStyle::Title,
    );
    draw_typing_session(typing, drawing, fonts, 50, 110, 1100, frame_time);
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
        &format!(
            "You transcribed {} {} while containing {}% of the original text. You transcribed {} leaves perfectly.",
            game.desktop.get_transcribed_leaves(),
            if game.desktop.get_transcribed_leaves() == 1 { "leaf"} else {"leaves"},
            game.desktop.get_correct_transcribing_in_percent(),
            game.desktop.get_perfect_leaves()
        ),
        Vector2::new(50.0, 25.0),
        TextStyle::Info,
    );
}

pub(crate) fn draw_typing_session(
    typing_session: &mut TypingSession,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    x: i32,
    y: i32,
    max_width: i32,
    frame_time: f32,
) {
    let settings = TextStyle::Body.get_settings(fonts);

    let mut cursor_x = x as f32;
    let mut cursor_y = y;
    let mut cursor_position = (x, y);

    for (index, character) in typing_session.target.iter().enumerate() {
        if *character == '\n' {
            if index == typing_session.cursor {
                cursor_position = (cursor_x as i32, cursor_y);
            }

            cursor_x = x as f32;
            cursor_y += LINE_HEIGHT;
            continue;
        }

        let text = character.to_string();

        let char_width = drawing.measure_text(&text, settings.font_size as i32) as f32;

        if cursor_x + char_width > (x + max_width) as f32 && cursor_x > x as f32 {
            cursor_x = x as f32;
            cursor_y += LINE_HEIGHT;
        }

        if index == typing_session.cursor {
            cursor_position = (cursor_x as i32, cursor_y);
        }

        let color = match typing_session.states[index] {
            CharacterState::Untyped => Color::DARKGRAY,
            CharacterState::Correct => Color::BLACK,
            CharacterState::Incorrect => Color::WHITE,
        };

        draw_styled_text(
            drawing,
            fonts,
            &text,
            Vector2::new(cursor_x, cursor_y as f32),
            TextStyle::Custom(StyleSettings {
                color,
                font: settings.font,
                font_size: settings.font_size,
                spacing: settings.spacing,
            }),
        );
        cursor_x += char_width;
    }

    if typing_session.cursor == typing_session.target.len() {
        cursor_position = (cursor_x as i32, cursor_y);
    }

    let cursor_position = typing_session.animate_cursor(
        Vector2::new(cursor_position.0 as f32, cursor_position.1 as f32),
        frame_time,
    );
    draw_cursor(drawing, &cursor_position, &settings, 2.0);
}

fn draw_cursor(
    drawing: &mut RaylibDrawHandle,
    cursor_position: &Vector2,
    settings: &StyleSettings,
    thickness: f32,
) {
    let height_modifier = 4.0;

    drawing.draw_line_ex(
        Vector2::new(cursor_position.x, cursor_position.y + height_modifier),
        Vector2::new(
            cursor_position.x,
            cursor_position.y + settings.font_size - height_modifier,
        ),
        thickness,
        settings.color,
    );

    drawing.draw_line_ex(
        Vector2::new(
            cursor_position.x + 4.0,
            cursor_position.y + settings.font_size - height_modifier,
        ),
        Vector2::new(
            cursor_position.x - 4.0,
            cursor_position.y + settings.font_size - height_modifier,
        ),
        thickness,
        settings.color,
    );

    drawing.draw_line_ex(
        Vector2::new(cursor_position.x + 4.0, cursor_position.y + height_modifier),
        Vector2::new(cursor_position.x - 4.0, cursor_position.y + height_modifier),
        thickness,
        settings.color,
    );
}
