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
pub struct CharacterCard {
    name: String,
    combat: u8,
    attunement: u8,
    intellect: u8,
    speed: u8,
    willpower: u8,
    health: u8,
    sanity: u8,
    text: String,
}

impl CharacterCard {
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

    pub fn render_name<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.name, 35, 25)
    }

    pub fn render_willpower<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.willpower.to_string(), 200, 7)
    }

    pub fn render_intellect<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.intellect.to_string(), 245, 7)
    }

    pub fn render_combat<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.combat.to_string(), 290, 7)
    }

    pub fn render_speed<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.speed.to_string(), 335, 7)
    }

    pub fn render_attunement<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.attunement.to_string(), 370, 7)
    }

    pub fn render_health<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.health.to_string(), 230, 40)
    }

    pub fn render_sanity<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.sanity.to_string(), 320, 40)
    }

    pub fn render_text<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.text, 190, 150)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_card_from_yaml() {
        const CHARACTER_CARD_YAML: &str = "
name: CharacterCard
combat: 5
attunement: 5
intellect: 5
speed: 5
willpower: 5
health: 10
sanity: 10
text: This is a character
";

        let character_card: CharacterCard = serde_yaml::from_str(CHARACTER_CARD_YAML).unwrap();
        assert_eq!(character_card.name, "CharacterCard");
    }
}
