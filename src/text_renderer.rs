use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::{Font, Vector2};
use raylib::{RaylibHandle, RaylibThread};

pub(crate) struct FontsHandle {
    pub(crate) regular: Font,
    pub(crate) semi_bold: Font,
    pub(crate) bold: Font,
    pub(crate) pixantiqua: Font,
}

impl FontsHandle {
    pub(crate) fn load(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            regular: handle
                .load_font_ex(
                    thread,
                    "res/fonts/pixelify_sans/PixelifySans-Regular.ttf",
                    32,
                    None,
                )
                .expect("Could not load regular font"),
            semi_bold: handle
                .load_font_ex(
                    thread,
                    "res/fonts/pixelify_sans/PixelifySans-SemiBold.ttf",
                    32,
                    None,
                )
                .expect("Could not load semi_bold font"),
            bold: handle
                .load_font_ex(
                    thread,
                    "res/fonts/pixelify_sans/PixelifySans-Bold.ttf",
                    32,
                    None,
                )
                .expect("Could not load bold font"),
            pixantiqua: handle
                .load_font_ex(thread, "res/fonts/pixantiqua.ttf", 32, None)
                .expect(""),
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
    BoldBody,
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
                font: &fonts.regular,
                font_size: 32.0,
                spacing: 1.0,
            },
            Self::BoldBody => StyleSettings {
                color: Color::WHITE,
                font: &fonts.semi_bold,
                font_size: 32.0,
                spacing: 1.0,
            },
            Self::Title => StyleSettings {
                color: Color::WHITE,
                font: &fonts.bold,
                font_size: 48.0,
                spacing: 1.0,
            },
            Self::Info => StyleSettings {
                color: Color::WHITE,
                font: &fonts.regular,
                font_size: 32.0,
                spacing: 1.0,
            },
            Self::Error => StyleSettings {
                color: Color::RED,
                font: &fonts.regular,
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
