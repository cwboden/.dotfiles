use crate::card::types::{PlayerCardClass, PlayerCardSlotType, PlayerCardType};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use serde::Deserialize;
use std::path::Path;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Deserialize)]
pub struct PlayerCard {
    title: String,
    subtype: PlayerCardType,
    class: PlayerCardClass,
    cost: String,
    slots: Vec<PlayerCardSlotType>,
    tags: String,
    text: String,
}

impl PlayerCard {
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

    pub fn render_class_icon<'a, T>(
        &self,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        let icon_path = match self.class {
            PlayerCardClass::Character => Path::new("img/icons/card-class/character.png"),
            PlayerCardClass::Researcher => Path::new("img/icons/card-class/researcher.png"),
            PlayerCardClass::Runner => Path::new("img/icons/card-class/runner.png"),
            PlayerCardClass::Scavenger => Path::new("img/icons/card-class/scavenger.png"),
            PlayerCardClass::Seeker => Path::new("img/icons/card-class/seeker.png"),
            PlayerCardClass::Soldier => Path::new("img/icons/card-class/soldier.png"),
        };

        let icon_image = texture_creator.load_texture(icon_path)?;
        let target = rect!(3, 90, 32, 32);

        Ok((icon_image, target))
    }

    pub fn render_subtype_icon<'a, T>(
        &self,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        let icon_path = match self.subtype {
            PlayerCardType::Asset => Path::new("img/icons/card-subtype/asset.png"),
            PlayerCardType::Event => Path::new("img/icons/card-subtype/event.png"),
            PlayerCardType::Skill => Path::new("img/icons/card-subtype/skill.png"),
        };

        let icon_image = texture_creator.load_texture(icon_path)?;
        let target = rect!(3, 53, 32, 32);

        Ok((icon_image, target))
    }

    pub fn render_cost<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.cost.to_string(), 15, 7)
    }

    pub fn render_slots<'a, T>(
        &self,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<Vec<(Texture<'a>, Rect)>, String> {
        let mut textures_and_targets = Vec::new();
        let mut x_pos = 265;
        for slot in &self.slots {
            let icon_path = match slot {
                PlayerCardSlotType::Accessory => Path::new("img/icons/card-slot/accessory.png"),
                PlayerCardSlotType::Body => Path::new("img/icons/card-slot/body.png"),
                PlayerCardSlotType::Head => Path::new("img/icons/card-slot/head.png"),
                PlayerCardSlotType::Legs => Path::new("img/icons/card-slot/feet.png"),
                PlayerCardSlotType::OneHand => Path::new("img/icons/card-slot/one-hand.png"),
                PlayerCardSlotType::Relic => Path::new("img/icons/card-slot/relic.png"),
                PlayerCardSlotType::Spell => Path::new("img/icons/card-slot/spell.png"),
                PlayerCardSlotType::TwoHands => Path::new("img/icons/card-slot/two-hands.png"),
            };

            let icon_image = texture_creator.load_texture(icon_path)?;
            let target = rect!(x_pos, 385, 32, 32);
            x_pos -= 32;

            textures_and_targets.push((icon_image, target));
        }

        Ok(textures_and_targets)
    }

    pub fn render_tags<'a, T>(
        &self,
        font: &Font,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<(Texture<'a>, Rect), String> {
        self.render(font, texture_creator, &self.tags, 35, 140)
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
    fn player_card_from_yaml() {
        const PLAYER_CARD_YAML: &str = "
title: PlayerCard
subtype: Asset
class: Scavenger
cost: 3
slots:
    - OneHand
    - Relic
tags: Old World. Weapon.
text: \\{fight\\} For this attack, you have +1 \\{strength\\}.
";

        let player_card: PlayerCard = serde_yaml::from_str(PLAYER_CARD_YAML).unwrap();

        assert_eq!(player_card.title, "PlayerCard");
    }
}
