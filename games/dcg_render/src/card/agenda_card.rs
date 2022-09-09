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

#[derive(Deserialize)]
pub struct AgendaCard {
    title: String,
    doom: u8,
    text: String,
}

impl AgendaCard {
    fn render<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
        text: &str,
        x: u32,
        y: u32,
        color: Color,
    ) -> Result<(Texture<'a>, Rect), String> {
        let surface = font
            .render(text)
            .blended_wrapped(color, 300)
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
        self.render(
            font,
            texture_creator,
            &self.title,
            75,
            15,
            Color::RGBA(0, 0, 0, 255),
        )
    }

    pub fn render_doom<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(
            font,
            texture_creator,
            &self.doom.to_string(),
            16,
            256,
            Color::RGBA(255, 255, 255, 255),
        )
    }

    pub fn render_text<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(
            font,
            texture_creator,
            &self.text,
            35,
            50,
            Color::RGBA(0, 0, 0, 255),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agenda_card_from_yaml() {
        const AGENDA_CARD_YAML: &str = "
title: AgendaCard
doom: 8
text: Get doomed
";

        let agenda_card: AgendaCard = serde_yaml::from_str(AGENDA_CARD_YAML).unwrap();
        assert_eq!(agenda_card.title, "AgendaCard");
    }
}
