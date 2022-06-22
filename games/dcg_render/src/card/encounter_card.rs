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
pub struct EncounterCard {
    title: String,
    text: String,
}

impl EncounterCard {
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
            .blended_wrapped(Color::RGBA(0, 0, 0, 255), 200)
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

    pub fn render_text<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.text, 35, 250)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encounter_card_from_yaml() {
        const ENCOUNTER_CARD_YAML: &str = "
title: EncounterCard
text: Take 2 damage
";

        let encounter_card: EncounterCard = serde_yaml::from_str(ENCOUNTER_CARD_YAML).unwrap();
        assert_eq!(encounter_card.title, "EncounterCard");
    }
}
