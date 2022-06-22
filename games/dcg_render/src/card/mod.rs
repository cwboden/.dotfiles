use crate::card::types::PlayerCardClass;
use crate::error::ParseError;
use components::{CardComponent, RenderError, Renderable};
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use std::str::{FromStr, Lines};
use std::string::ToString;
use std::vec::Vec;
use types::CardType;

pub mod act_card;
pub mod agenda_card;
pub mod character_card;
mod components;
pub mod encounter_card;
pub mod enemy_card;
pub mod error;
pub mod location_card;
pub mod player_card;
pub mod types;

#[allow(dead_code)]
pub struct Card<T: RenderTarget, C> {
    card_type: CardType,
    components: Vec<Box<dyn CardComponent<T, C>>>,
}

#[allow(dead_code)]
impl<T: RenderTarget, C> Card<T, C> {
    pub fn render(
        &self,
        canvas: &mut Canvas<T>,
        texture_creator: &TextureCreator<C>,
    ) -> Result<(), RenderError> {
        for component in &self.components {
            (*component).render(canvas, texture_creator)?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub struct CardFactory<'r, C> {
    templates: HashMap<CardType, Texture<'r>>,
    texture_creator: Rc<TextureCreator<C>>,
}

pub enum CardFactoryError {
    IoError(std::io::Error),
    ParseError(ParseError),
}

impl From<std::io::Error> for CardFactoryError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<ParseError> for CardFactoryError {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl ToString for CardFactoryError {
    fn to_string(&self) -> String {
        String::from("CardFactoryError")
    }
}

#[allow(dead_code)]
impl<'r, C> CardFactory<'r, C> {
    const TEMPLATE_DIR: &'static str = "img/templates/";

    pub fn new(texture_creator: Rc<TextureCreator<C>>) -> Result<Self, CardFactoryError> {
        let templates = HashMap::new();

        for entry in fs::read_dir(Self::TEMPLATE_DIR)? {
            let path = entry?.path();
            assert!(!path.is_dir());
            println!("{}", path.display());
        }

        Ok(CardFactory {
            templates,
            texture_creator,
        })
    }

    pub fn from_file<T: RenderTarget>(&self, path: &Path) -> Result<Card<T, C>, CardFactoryError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut content_iter = contents.lines();
        let card_type = CardType::from_str(content_iter.next().unwrap())?;
        match card_type {
            CardType::Player => self.create_player_card(&mut content_iter),
            CardType::Location => self.create_location_card(&mut content_iter),
            _ => panic!("Not yet implemented!"),
        }
    }

    fn create_player_card<T: RenderTarget>(
        &self,
        content_iter: &mut Lines,
    ) -> Result<Card<T, C>, CardFactoryError> {
        let _class = PlayerCardClass::from_str(content_iter.next().unwrap())
            .unwrap_or(PlayerCardClass::Scavenger);
        let _title = String::from(content_iter.next().unwrap());
        let _cost = content_iter.next().unwrap().parse::<u8>().unwrap_or(0);
        let _image = String::from(content_iter.next().unwrap());
        let _description = String::from(content_iter.next().unwrap());
        assert_eq!(content_iter.next(), None);

        Ok(Card {
            card_type: CardType::Player,
            components: Vec::new(),
        })
    }

    fn create_location_card<T: RenderTarget>(
        &self,
        content_iter: &mut Lines,
    ) -> Result<Card<T, C>, CardFactoryError> {
        let _title = String::from(content_iter.next().unwrap());
        let _shroud = content_iter.next().unwrap().parse::<u8>().unwrap();
        let _num_clues = String::from(content_iter.next().unwrap());
        let _image = String::from(content_iter.next().unwrap());
        let _description = String::from(content_iter.next().unwrap());
        assert_eq!(content_iter.next(), None);

        Ok(Card {
            card_type: CardType::Location,
            components: Vec::new(),
        })
    }
}
