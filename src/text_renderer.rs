use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::{Font, Vector2};
use raylib::{RaylibHandle, RaylibThread};

pub(crate) struct FontsHandle {
    pub(crate) body: Font,
    pub(crate) title: Font,
    pub(crate) info: Font,
    pub(crate) error: Font,
}

impl FontsHandle {
    pub(crate) fn load(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            body: handle
                .load_font_ex(thread, "res/fonts/CormorantGaramond-Regular.ttf", 32, None)
                .expect("Could not load body font"),
            title: handle
                .load_font_ex(thread, "res/fonts/CormorantGaramond-SemiBold.ttf", 32, None)
                .expect("Could not load title font"),
            info: handle
                .load_font_ex(
                    thread,
                    "res/fonts/CormorantGaramond-MediumItalic.ttf",
                    32,
                    None,
                )
                .expect("Could not load info font"),
            error: handle
                .load_font_ex(
                    thread,
                    "res/fonts/CormorantGaramond-MediumItalic.ttf",
                    32,
                    None,
                )
                .expect("Could not load error font"),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct StyleSettings<'a> {
    pub(crate) color: Color,
    pub(crate) font: &'a Font,
    pub(crate) font_size: f32,
    pub(crate) spacing: f32,
}

pub(crate) enum TextStyle<'a> {
    Body,
    Title,
    Info,
    Error,
    Custom(StyleSettings<'a>),
}

impl<'a> TextStyle<'a> {
    pub(crate) fn get_settings(&self, fonts: &'a FontsHandle) -> StyleSettings<'a> {
        match self {
            Self::Body => StyleSettings {
                color: Color::WHITE,
                font: &fonts.body,
                font_size: 48.0,
                spacing: 1.0,
            },
            Self::Title => StyleSettings {
                color: Color::WHITE,
                font: &fonts.title,
                font_size: 64.0,
                spacing: 1.0,
            },
            Self::Info => StyleSettings {
                color: Color::LIGHTGRAY,
                font: &fonts.info,
                font_size: 32.0,
                spacing: 1.0,
            },
            Self::Error => StyleSettings {
                color: Color::RED,
                font: &fonts.error,
                font_size: 32.0,
                spacing: 1.0,
            },
            Self::Custom(settings) => *settings,
        }
    }
}

pub(crate) fn draw_styled_text(
    drawing: &mut RaylibDrawHandle,
    fonts: &FontsHandle,
    content: &str,
    position: Vector2,
    style: TextStyle<'_>,
) {
    let settings = style.get_settings(fonts);
    drawing.draw_text_ex(
        settings.font,
        content,
        position,
        settings.font_size,
        settings.spacing,
        settings.color,
    )
}
