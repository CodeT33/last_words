use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ffi::Vector2;
use raylib::prelude::{Color, Font};
use raylib::{RaylibHandle, RaylibThread};

pub(crate) struct Fonts {
    pub(crate) body: Font,
    pub(crate) title: Font,
    pub(crate) info: Font,
    pub(crate) error: Font,
}

impl Fonts {
    pub(crate) fn load(raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let body = raylib_handle
            .load_font_ex(thread, "res/fonts/pixantiqua.ttf", 32, None)
            .expect("Could not load font");

        let title = raylib_handle
            .load_font_ex(thread, "res/fonts/pixantiqua.ttf", 32, None)
            .expect("Could not load font");

        let info = raylib_handle
            .load_font_ex(thread, "res/fonts/pixantiqua.ttf", 32, None)
            .expect("Could not load font");
        let error = raylib_handle
            .load_font_ex(thread, "res/fonts/pixantiqua.ttf", 32, None)
            .expect("Could not load font");
        Self {
            body,
            title,
            info,
            error,
        }
    }
}

pub(crate) struct TextDrawer {
    fonts: Fonts,
}

impl TextDrawer {
    pub(crate) fn load(raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let fonts = Fonts::load(raylib_handle, thread);
        Self { fonts }
    }

    pub(crate) fn draw_text_body(
        &self,
        drawing: &mut RaylibDrawHandle,
        content: &str,
        Vector2 {
            x: char_x,
            y: char_y,
        }: Vector2,
        color: Color,
    ) {
        drawing.draw_text_ex(
            &self.fonts.body,
            content,
            Vector2::new(char_x, char_y),
            48.0,
            1.0,
            color,
        );
    }

    pub(crate) fn draw_text_title(
        &self,
        drawing: &mut RaylibDrawHandle,
        content: &str,
        Vector2 {
            x: char_x,
            y: char_y,
        }: Vector2,
    ) {
        drawing.draw_text_ex(
            &self.fonts.title,
            content,
            Vector2::new(char_x, char_y),
            48.0,
            1.0,
            Color::BLACK,
        );
    }

    pub(crate) fn draw_text_info(
        &self,
        drawing: &mut RaylibDrawHandle,
        content: &str,
        Vector2 {
            x: char_x,
            y: char_y,
        }: Vector2,
    ) {
        drawing.draw_text_ex(
            &self.fonts.info,
            content,
            Vector2::new(char_x, char_y),
            24.0,
            1.0,
            Color::DARKGRAY,
        );
    }
    
    pub(crate) fn draw_text_error(&self, drawing: &mut RaylibDrawHandle, content: &str, Vector2 { x: char_x, y: char_y }: Vector2) {
        drawing.draw_text_ex(&self.fonts.error, content, Vector2::new(char_x, char_y), 24.0, 1.0, Color::RED)
    }
}
