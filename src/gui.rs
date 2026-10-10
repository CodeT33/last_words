use crate::consts::{LINE_HEIGHT, ui_positions};
use crate::desktop::Desktop;
use crate::game::{GameContext, GameState};
use crate::rendering::{AssetServer, TextureName, draw_sprite};
use crate::text_renderer::{FontsHandle, StyleSettings, TextStyle, draw_styled_text};
use crate::typing::{CharacterState, TypingSession};
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibFont, Vector2};
use raylib::{RaylibHandle, RaylibThread};

pub(crate) unsafe fn draw_ui(
    raylib_handle: &mut RaylibHandle,
    thread: &RaylibThread,
    game: &mut GameContext,
    fonts: &FontsHandle,
    asset_server: &AssetServer,
) {
    let frame_time = raylib_handle.get_frame_time();
    let mut drawing = raylib_handle.begin_drawing(thread);
    drawing.clear_background(Color::WHITE);
    draw_sprite(
        &mut drawing,
        asset_server,
        TextureName::StackSelectionScreen,
        Vector2::new(0.0, 0.0),
        0.0,
        4.0,
    );
    draw_sprite(
        &mut drawing,
        asset_server,
        TextureName::JohnConcentrated,
        Vector2::new(16.0 * 4.0 * 1.0, 16.0 * 4.0 * 2.5),
        0.0,
        4.0,
    );

    draw_timer(game, &mut drawing, fonts, ui_positions::COUNTDOWN);
    draw_total_points(&mut drawing, fonts, ui_positions::TOTAL_POINTS);

    let desktop = &game.desktop;

    match &mut game.game_state {
        GameState::SelectStack { input } => {
            draw_stats(&desktop, &mut drawing, fonts, ui_positions::STATS);
            draw_stack_selection(&game.desktop, &mut drawing, fonts, input);
        }

        GameState::WriteLeaf { typing, .. } => {
            draw_leaf_typing(&mut drawing, fonts, typing, frame_time);
        }
    }
}

pub(crate) fn draw_stats(
    desktop: &Desktop,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    position: Vector2,
) {
    draw_styled_text(
        drawing,
        fonts,
        &format!(
            "{} Done\n{} Perfect\n{} Chars\n{} Mistakes\n{}% Accuracy\n{} WPM",
            desktop.get_transcribed_leaves(),
            desktop.get_perfect_leaves(),
            desktop.get_transcribed_chars(),
            desktop.get_typing_errors(),
            desktop.get_correct_transcribing_in_percent(),
            67,
        ),
        position,
        TextStyle::Info,
    );
}

fn draw_total_points(drawing: &mut RaylibDrawHandle, fonts: &FontsHandle, position: Vector2) {
    draw_styled_text(drawing, fonts, "000067", position, TextStyle::Title);
}

pub(crate) fn draw_stack_selection(
    desktop: &Desktop,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    input: &str,
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
                    Vector2::new((x + 20) as f32, 180.0),
                );
                draw_styled_text(
                    drawing,
                    fonts,
                    &format!("{} leaves remaining", stack.len()),
                    Vector2::new((x + 20) as f32, 235.0),
                    TextStyle::Info,
                );
            }
            None => {
                draw_styled_text(
                    drawing,
                    fonts,
                    "EMPTY",
                    Vector2::new((x + 20) as f32, 180.0),
                    TextStyle::Info,
                );
                draw_styled_text(
                    drawing,
                    fonts,
                    "No more leaves remaining",
                    Vector2::new((x + 20) as f32, 235.0),
                    TextStyle::Info,
                );
            }
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
    positon: Vector2,
) {
    let body = TextStyle::Body.get_settings(fonts);
    let highlighted_characters = if is_matching {
        input.chars().count()
    } else {
        0
    };
    let mut char_x = positon.x;
    let fading_word = format!("{}...", &word);

    for (index, character) in fading_word.chars().enumerate() {
        let text = character.to_string();
        draw_styled_text(
            drawing,
            fonts,
            &text,
            Vector2::new(char_x, positon.y),
            TextStyle::Custom(StyleSettings {
                color: if index < highlighted_characters {
                    Color::BLACK
                } else {
                    Color::GRAY
                },
                ..body
            }),
        );
        let char_width = body
            .font
            .measure_text(&text, body.font_size, body.spacing)
            .x;
        char_x += char_width;
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
    draw_typing_session(typing, drawing, fonts, 50, 130, 1100, frame_time);
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
    let mut cursor_y = y as f32;
    let mut cursor_position = (x as f32, y as f32);

    for (index, character) in typing_session.target.iter().enumerate() {
        if *character == '\n' {
            if index == typing_session.cursor {
                cursor_position = (cursor_x, cursor_y);
            }

            cursor_x = x as f32;
            cursor_y += LINE_HEIGHT as f32;
            continue;
        }

        let text = character.to_string();

        let char_width = settings
            .font
            .measure_text(&text, settings.font_size, settings.spacing)
            .x;

        if cursor_x + char_width > (x + max_width) as f32 && cursor_x > x as f32 {
            cursor_x = x as f32;
            cursor_y += LINE_HEIGHT as f32;
        }

        if index == typing_session.cursor {
            cursor_position = (cursor_x, cursor_y);
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
            Vector2::new(cursor_x, cursor_y),
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
        cursor_position = (cursor_x, cursor_y);
    }

    let cursor_position = typing_session.animate_cursor(
        Vector2::new(cursor_position.0, cursor_position.1),
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

fn draw_timer(
    game: &GameContext,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    position: Vector2,
) {
    draw_styled_text(
        drawing,
        fonts,
        &game.timer.display(),
        position,
        TextStyle::Title,
    );
}
