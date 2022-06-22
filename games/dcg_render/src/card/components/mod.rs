use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, TextureCreator};
use sdl2::surface::Surface;

pub mod cost;
pub mod description;
pub mod title;

pub enum RenderError {
    Font(sdl2::ttf::FontError),
    RcTryUnwrap(()),
    Sdl2(String),
    TextureValue(sdl2::render::TextureValueError),
}

impl From<sdl2::ttf::FontError> for RenderError {
    fn from(err: sdl2::ttf::FontError) -> Self {
        Self::Font(err)
    }
}

impl<T> From<Rc<T>> for RenderError {
    fn from(_err: Rc<T>) -> Self {
        Self::RcTryUnwrap(())
    }
}

impl From<String> for RenderError {
    fn from(err: String) -> Self {
        Self::Sdl2(err)
    }
}

impl From<sdl2::render::TextureValueError> for RenderError {
    fn from(err: sdl2::render::TextureValueError) -> Self {
        Self::TextureValue(err)
    }
}

pub trait CardComponent<T: RenderTarget, C>: Surfaced {}

#[allow(dead_code)]
impl<T: RenderTarget, C> Renderable<T, C> for dyn CardComponent<T, C> {
    fn render(
        &self,
        canvas: &mut Canvas<T>,
        texture_creator: &TextureCreator<C>,
    ) -> Result<(), RenderError> {
        let surface = self.get_surface()?;
        let target = self.get_target_rect()?;

        let texture = texture_creator.create_texture_from_surface(&surface)?;
        canvas.copy(&texture, None /* src_rect */, target)?;

        Ok(())
    }
}

pub trait Renderable<T: RenderTarget, C> {
    fn render(
        &self,
        canvas: &mut Canvas<T>,
        texture_creator: &TextureCreator<C>,
    ) -> Result<(), RenderError>;
}

pub trait Surfaced {
    fn get_surface(&self) -> Result<Surface, RenderError>;

    fn get_target_rect(&self) -> Result<Rect, RenderError>;
}
