use crate::card::components::{RenderError, Surfaced};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use std::rc::Rc;

struct Title<'ttf_module, 'rwops> {
    title: String,
    font: Rc<Font<'ttf_module, 'rwops>>,
}

#[allow(dead_code)]
impl<'ttf_module, 'rwops> Title<'ttf_module, 'rwops> {
    const X_POSITION: u32 = 75;
    const Y_POSITION: u32 = 15;
    const TARGET_WIDTH: u32 = 200;
    const TARGET_HEIGHT: u32 = 40;
    const FONT_COLOR: Color = Color::RGBA(0, 0, 0, 255);
    const MAX_WRAP_WIDTH: u32 = 200;

    pub fn new(title: String, font: Rc<Font<'ttf_module, 'rwops>>) -> Self {
        Self { title, font }
    }
}

#[allow(dead_code)]
impl<'ttf_module, 'rwops> Surfaced for Title<'ttf_module, 'rwops> {
    fn get_surface(&self) -> Result<Surface, RenderError> {
        let font = Rc::clone(&self.font);
        let surface = Rc::try_unwrap(font)?
            .render(&self.title)
            .blended_wrapped(Self::FONT_COLOR, Self::MAX_WRAP_WIDTH)?;

        Ok(surface)
    }

    fn get_target_rect(&self) -> Result<Rect, RenderError> {
        Ok(Rect::new(
            Self::X_POSITION as i32,
            Self::Y_POSITION as i32,
            Self::TARGET_WIDTH,
            Self::TARGET_HEIGHT,
        ))
    }
}
