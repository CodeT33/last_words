use rand::RngExt;
use crate::consts::{LEAF_WRITING_WIDTH, LINE_HEIGHT, TILE_SIZE, ui_positions};
use crate::desktop::Desktop;
use crate::game::{GameContext, GameState};
use crate::leaves::StackOfLeaves;
use crate::rendering::{AssetServer, TextureName, draw_sprite};
use crate::text_renderer::{FontsHandle, StyleSettings, TextStyle, draw_styled_text};
use crate::timer::Timer;
use crate::typing::{CharacterState, TypingSession};
use raylib::ffi::CSSPalette;
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

    let desktop = &game.desktop;
    let game_state = &mut game.game_state;
    let game_timer = &game.timer;

    match game_state {
        GameState::SelectStack { input } => {
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
                TextureName::JohnSurprised,
                Vector2::new(TILE_SIZE, TILE_SIZE * 2.5),
                0.0,
                4.0,
            );
            draw_timer(game_timer, &mut drawing, fonts, ui_positions::COUNTDOWN);
            draw_total_points(
                &game.desktop,
                &mut drawing,
                fonts,
                ui_positions::TOTAL_POINTS,
            );
            draw_stats(&desktop, &mut drawing, fonts, ui_positions::STATS);
            draw_stack_selection(&game.desktop, &mut drawing, fonts, input);
            draw_stack_signs(&game.desktop, &mut drawing, &asset_server, fonts);
        }
        GameState::WriteLeaf { typing, .. } => {
            draw_sprite(
                &mut drawing,
                asset_server,
                TextureName::LeafWritingScreen,
                Vector2::new(0.0, 0.0),
                0.0,
                4.0,
            );
            draw_timer(game_timer, &mut drawing, fonts, ui_positions::COUNTDOWN);
            draw_total_points(
                &game.desktop,
                &mut drawing,
                fonts,
                ui_positions::TOTAL_POINTS,
            );
            draw_leaf_writing(
                &mut drawing,
                fonts,
                typing,
                frame_time,
                ui_positions::LEAF_WRITING,
            );
        }
    }
}

pub(crate) fn draw_stats(
    desktop: &Desktop,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    position: Vector2,
) {
    let errors = desktop.get_typing_errors();
    draw_styled_text(
        drawing,
        fonts,
        &format!(
            "{} done\n{} perfect\n\n{} {}\n{}% acc.\n{} wpm",
            desktop.get_transcribed_leaves(),
            desktop.get_perfect_leaves(),
            errors,
            if errors == 1 { "error" } else { "errors" },
            desktop.get_correct_transcribing_in_percent(),
            desktop.get_average_wpm(),
        ),
        position,
        TextStyle::Info,
    );
}

fn draw_total_points(
    desktop: &Desktop,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    position: Vector2,
) {
    draw_styled_text(
        drawing,
        fonts,
        &format!("{:0>6} p", desktop.get_total_points()),
        position,
        TextStyle::Title,
    );
}

fn draw_stack_info(
    stack: &StackOfLeaves,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    input: &str,
    stack_name: &str,
    position: Vector2,
) {
    draw_styled_text(
        drawing,
        fonts,
        stack_name,
        Vector2::new(position.x, position.y + 8.0),
        TextStyle::Title,
    );
    match stack.top_word() {
        Some(word) => {
            draw_stack_word(
                drawing,
                fonts,
                &word,
                input,
                input_matches_word(&word, &input),
                Vector2::new(position.x + 0.7 * TILE_SIZE, position.y + 16.0),
            );
        }
        None => {
            draw_styled_text(
                drawing,
                fonts,
                "EMPTY",
                Vector2::new(position.x + 0.7 * TILE_SIZE, position.y + 16.0),
                TextStyle::Info,
            );
        }
    }
}

pub(crate) fn draw_stack_signs(
    desktop: &Desktop,
    drawing: &mut RaylibDrawHandle,
    asset_server: &AssetServer,
    fonts: &FontsHandle,
) {
    draw_stack_of_leaves(
        drawing,
        fonts,
        "A",
        ui_positions::STACK_SIGN_A,
        &desktop,
        asset_server,
    );
    draw_stack_of_leaves(
        drawing,
        fonts,
        "B",
        ui_positions::STACK_SIGN_B,
        &desktop,
        asset_server,
    );
    draw_stack_of_leaves(
        drawing,
        fonts,
        "C",
        ui_positions::STACK_SIGN_C,
        &desktop,
        asset_server,
    );
    draw_stack_of_leaves(
        drawing,
        fonts,
        "D",
        ui_positions::STACK_SIGN_D,
        &desktop,
        asset_server,
    );
}

fn draw_stack_of_leaves(
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    content: &str,
    position: Vector2,
    desktop: &Desktop,
    asset_server: &AssetServer,
) {
    let stack = match content {
        "A" => desktop.stacks.get(0),
        "B" => desktop.stacks.get(1),
        "C" => desktop.stacks.get(2),
        "D" => desktop.stacks.get(3),
        &_ => None,
    };
    if stack.is_some() {
        let mut stack_height = stack.unwrap().len();
        if stack_height > 0 {
            draw_styled_text(
                drawing,
                fonts,
                &format!("{} [{}]", content, stack_height),
                Vector2::new(position.x, position.y + TILE_SIZE),
                TextStyle::BoldBody,
            );
            draw_sprite(
                drawing,
                asset_server,
                TextureName::LeafBottom,
                position,
                0.0,
                4.0,
            );
            stack_height -= 1;
        }
        for i in 0..stack_height {
            draw_sprite(
                drawing,
                asset_server,
                TextureName::LeafTop,
                Vector2::new(position.x - 4.7 + (i as f32 *2.3)%4.7, position.y - (4 + i * 8) as f32),
                i as f32 * 8.2 % 12.0,
               4.0,
            );
        }
    }
}

pub(crate) fn draw_stack_selection(
    desktop: &Desktop,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    input: &str,
) {
    draw_stack_info(
        desktop.stacks.get(0).unwrap(),
        drawing,
        fonts,
        input,
        "A",
        ui_positions::STACK_A,
    );
    draw_stack_info(
        desktop.stacks.get(1).unwrap(),
        drawing,
        fonts,
        input,
        "B",
        ui_positions::STACK_B,
    );
    draw_stack_info(
        desktop.stacks.get(2).unwrap(),
        drawing,
        fonts,
        input,
        "C",
        ui_positions::STACK_C,
    );
    draw_stack_info(
        desktop.stacks.get(3).unwrap(),
        drawing,
        fonts,
        input,
        "D",
        ui_positions::STACK_D,
    );
}

fn draw_stack_sign(
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    content: &str,
    position: Vector2,
    desktop: &Desktop,
) {
    todo!()
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
                    Color::WHITE
                } else {
                    Color::LIGHTGRAY
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

pub(crate) fn draw_leaf_writing(
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    typing: &mut TypingSession,
    frame_time: f32,
    position: Vector2,
) {
    draw_typing_session(
        typing,
        drawing,
        fonts,
        Vector2::new(position.x, position.y),
        LEAF_WRITING_WIDTH,
        frame_time,
    );
    // draw_styled_text(
    //     drawing,
    //     fonts,
    //     &format!(
    //         "{}/{} characters",
    //         typing.typed_characters(),
    //         typing.character_count()
    //     ),
    //     Vector2::new(50.0, 620.0),
    //     TextStyle::Info,
    // );
}

pub(crate) fn draw_typing_session(
    typing_session: &mut TypingSession,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    position: Vector2,
    max_width: f32,
    frame_time: f32,
) {
    let settings = TextStyle::Body.get_settings(fonts);

    let mut cursor_x = position.x;
    let mut cursor_y = position.y;
    let mut cursor_position = (position.x, position.y);
    let content_right = position.x + max_width;

    for (index, character) in typing_session.target.iter().enumerate() {
        if *character == '\n' {
            if index == typing_session.cursor {
                cursor_position = (cursor_x, cursor_y);
            }

            cursor_x = position.x;
            cursor_y += LINE_HEIGHT as f32;
            continue;
        }

        if !character.is_whitespace()
            && (index == 0 || typing_session.target[index - 1].is_whitespace())
        {
            let word_end = typing_session.target[index..]
                .iter()
                .position(|next| next.is_whitespace())
                .map(|offset| index + offset)
                .unwrap_or(typing_session.target.len());
            let word: String = typing_session.target[index..word_end].iter().collect();
            let word_width = settings
                .font
                .measure_text(&word, settings.font_size, settings.spacing)
                .x;

            if cursor_x > position.x && cursor_x + word_width > content_right {
                cursor_x = position.x;
                cursor_y += LINE_HEIGHT as f32;
            }
        }

        let text = character.to_string();

        let char_width = settings
            .font
            .measure_text(&text, settings.font_size, settings.spacing)
            .x;

        if index == typing_session.cursor {
            cursor_position = (cursor_x, cursor_y);
        }

        let color = match typing_session.states[index] {
            CharacterState::Untyped => Color::BLACK,
            CharacterState::Correct => Color::WHITE,
            CharacterState::Incorrect => Color::TRANSPARENT,
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
    draw_cursor(drawing, &cursor_position, &settings, 3.0);
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
    game_timer: &Timer,
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    position: Vector2,
) {
    draw_styled_text(
        drawing,
        fonts,
        &game_timer.display(),
        position,
        TextStyle::Title,
    );
}
