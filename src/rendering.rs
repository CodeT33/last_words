use std::collections::HashMap;

use raylib::core::{RaylibHandle, RaylibThread};
use raylib::prelude::{Color, RaylibDraw, Texture2D, Vector2};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum TextureName {
    StackSelectionScreen,
    JohnConcentrated,
}

pub(crate) struct AssetServer {
    map: HashMap<TextureName, Texture2D>,
}

impl AssetServer {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub(crate) fn load_sprite(
        &mut self,
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
        name: TextureName,
    ) -> Result<(), String> {
        if self.map.contains_key(&name) {
            return Ok(());
        }

        let texture = raylib.load_texture(thread, name.path()).map_err(|err| {
            format!("failed to load texture '{}': {:?}", name.path(), err)
        })?;

        self.map.insert(name, texture);
        Ok(())
    }

    pub(crate) fn get_texture(&self, texture: TextureName) -> Option<&Texture2D> {
        self.map.get(&texture)
    }
}

pub(crate) fn draw_sprite(
    draw: &mut impl RaylibDraw,
    asset_server: &AssetServer,
    texture_name: TextureName,
    position: impl Into<Vector2>,
    rotation: f32,
    scale: f32,
) {
    let Some(texture) = asset_server.get_texture(texture_name) else {
        return;
    };

    draw.draw_texture_ex(texture, position, rotation, scale, Color::WHITE);
}

impl TextureName {
    pub const fn path(self) -> &'static str {
        match self {
            Self::StackSelectionScreen => "res/textures/screens/i_am_typewriter.stack_selection.png",
            Self::JohnConcentrated => "res/textures/character/john.concentrated.png",
        }
    }
}
