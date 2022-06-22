use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use serde::Deserialize;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Debug, Deserialize)]
pub struct ActCard {
    title: String,
    clues: u8,
    scale_clues: bool,
    text: String,
}

impl ActCard {
    fn render<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
        text: &str,
        x: u32,
        y: u32,
    ) -> Result<(Texture<'a>, Rect), String> {
        let surface = font
            .render(text)
            .blended_wrapped(Color::RGBA(0, 0, 0, 255), 300)
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let target = rect!(x, y, width, height);

        Ok((texture, target))
    }

    pub fn render_title<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.title, 75, 15)
    }

    pub fn render_clues<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.clues.to_string(), 384, 256)
    }

    pub fn render_text<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.text, 35, 50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACT_YAML: &str = "
title: ActCard
clues: 3
scale_clues: true
text: The game is afoot!
";

    #[test]
    fn act_is_parseable_from_yaml() {
        let act_card: ActCard = serde_yaml::from_str(ACT_YAML).unwrap();

        assert_eq!(act_card.title, "ActCard");
        assert_eq!(act_card.clues, 3);
        assert_eq!(act_card.scale_clues, true);
        assert_eq!(act_card.text, "The game is afoot!");
    }
}
