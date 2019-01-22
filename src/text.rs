//! Text Element

use sdl2::surface::Surface;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use tdmath::Vector2i;

/// Text Element ID
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TextID {
    /// Projection Option Text
    Projection,

    /// Smooth Shading Option Text
    SmoothShading,
}

/// Anchor Position
#[derive(Debug, Copy, Clone)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// Text Element
pub struct Text<'a> {
    surface: Option<Surface<'a>>,
    texture: Option<Texture<'a>>,
    anchor: Anchor,
    offset: Vector2i,
}

impl<'a> Text<'a> {
    /// Returns a new empty `Text`.
    pub fn new() -> Text<'a> {
        Text {
            surface: None,
            texture: None,
            anchor: Anchor::TopLeft,
            offset: Vector2i::new(0, 0),
        }
    }

    /// Sets the text and recreates the surface and texture.
    pub fn set_text(&mut self, font: &Font, texture_creator: &'a TextureCreator<WindowContext>, text: &str, color: Color) {
        let surface = font.render(text).blended(color).unwrap();

        self.texture = Some(texture_creator.create_texture_from_surface(&surface).unwrap());
        self.surface = Some(surface);
    }

    /// Sets the anchor.
    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.anchor = anchor;
    }

    /// Sets the position offset.
    pub fn set_offset(&mut self, offset: Vector2i) {
        self.offset = offset;
    }

    /// Returns the width of the text.
    /// Returns None if the text has not been set.
    pub fn width(&self) -> Option<u32> {
        match self.surface {
            Some(ref surface) => Some(surface.width()),
            None => None
        }
    }

    /// Returns the height of the text.
    /// Returns None if the text has not been set.
    pub fn height(&self) -> Option<u32> {
        match self.surface {
            Some(ref surface) => Some(surface.height()),
            None => None
        }
    }

    /// Returns the anchor.
    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    /// Returns the texture.
    pub fn texture(&self) -> &Option<Texture> {
        &self.texture
    }

    /// Returns the position of the text element after the anchor and offset has been applied.
    pub fn get_position(&self, screen_width: u32, screen_height: u32) -> Vector2i {
        let base_position = match self.anchor {
            Anchor::TopLeft => Vector2i::new(0, 0),
            Anchor::TopCenter => Vector2i::new(screen_width as i32 / 2, 0),
            Anchor::TopRight => Vector2i::new(screen_width as i32, 0),
            Anchor::MiddleLeft => Vector2i::new(0, screen_height as i32 / 2),
            Anchor::Center => Vector2i::new(screen_width as i32 / 2, screen_height as i32 / 2),
            Anchor::MiddleRight => Vector2i::new(screen_width as i32, screen_height as i32 / 2),
            Anchor::BottomLeft => Vector2i::new(0, screen_height as i32),
            Anchor::BottomCenter => Vector2i::new(screen_width as i32 / 2, screen_height as i32),
            Anchor::BottomRight => Vector2i::new(screen_width as i32, screen_height as i32),
        };

        base_position + self.offset
    }
}