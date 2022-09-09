extern crate sdl2;
use std::env;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use dcg_render::card::act_card::ActCard;
use dcg_render::card::agenda_card::AgendaCard;
use dcg_render::card::character_card::CharacterCard;
use dcg_render::card::encounter_card::EncounterCard;
use dcg_render::card::enemy_card::EnemyCard;
use dcg_render::card::error::CardError;
use dcg_render::card::location_card::LocationCard;
use dcg_render::card::player_card::PlayerCard;
use dcg_render::card::types::CardType;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;

pub const CARD_HEIGHT: u32 = 420;
pub const CARD_WIDTH: u32 = 300;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Debug)]
pub enum Error {
    CardError(CardError),
    InitError(sdl2::ttf::InitError),
    IoError(std::io::Error),
    ParseError(dcg_render::error::ParseError),
    Sdl2Error(String),
    SerdeYamlError(serde_yaml::Error),
}

impl From<CardError> for Error {
    fn from(err: CardError) -> Self {
        Self::CardError(err)
    }
}

impl From<sdl2::ttf::InitError> for Error {
    fn from(err: sdl2::ttf::InitError) -> Self {
        Self::InitError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<dcg_render::error::ParseError> for Error {
    fn from(err: dcg_render::error::ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Sdl2Error(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerdeYamlError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn get_player_card_render_objects<'a, T>(
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    texture_creator: &'a sdl2::render::TextureCreator<T>,
    path_to_card: &Path,
) -> Result<Vec<(Texture<'a>, Rect)>> {
    // Load card
    let file = File::open(&path_to_card)?;
    let card: PlayerCard = serde_yaml::from_reader(&file)?;

    // Image
    let image_path = Path::new("img/templates/player-card.png");
    let card_image = texture_creator.load_texture(image_path)?;

    // Text
    let font_path = Path::new("ttf/aovel-sans-rounded.ttf");
    let cost_font = ttf_context.load_font(font_path, 36)?;
    let title_font = ttf_context.load_font(font_path, 24)?;
    let text_font = ttf_context.load_font(font_path, 14)?;

    let mut targets = vec![
        (card_image, rect!(0, 0, CARD_WIDTH, CARD_HEIGHT)),
        card.render_cost(&cost_font, texture_creator)?,
        card.render_title(&title_font, texture_creator)?,
    ];

    let mut slot_targets = card.render_slots(texture_creator)?;
    targets.append(&mut slot_targets);
    targets.push(card.render_subtype_icon(texture_creator)?);
    targets.push(card.render_class_icon(texture_creator)?);
    targets.push(card.render_tags(&title_font, texture_creator)?);
    targets.push(card.render_text(&text_font, texture_creator)?);

    Ok(targets)
}

pub fn get_agenda_card_render_objects<'a, T>(
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    texture_creator: &'a sdl2::render::TextureCreator<T>,
    path_to_card: &Path,
) -> Result<Vec<(Texture<'a>, Rect)>> {
    // Load card
    let file = File::open(&path_to_card)?;
    let card: AgendaCard = serde_yaml::from_reader(&file)?;

    // Image
    let image_path = Path::new("img/templates/agenda-card.png");
    let card_image = texture_creator.load_texture(image_path)?;

    // Text
    let font_path = Path::new("ttf/aovel-sans-rounded.ttf");
    let doom_font = ttf_context.load_font(font_path, 36)?;
    let title_font = ttf_context.load_font(font_path, 24)?;
    let text_font = ttf_context.load_font(font_path, 14)?;

    let targets = vec![
        // Note Width / Height switched since this card is landscape
        (card_image, rect!(0, 0, CARD_HEIGHT, CARD_WIDTH)),
        card.render_doom(&doom_font, texture_creator)?,
        card.render_title(&title_font, texture_creator)?,
        card.render_text(&text_font, texture_creator)?,
    ];

    Ok(targets)
}

pub fn get_act_card_render_objects<'a, T>(
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    texture_creator: &'a sdl2::render::TextureCreator<T>,
    path_to_card: &Path,
) -> Result<Vec<(Texture<'a>, Rect)>> {
    // Load card
    let file = File::open(&path_to_card)?;
    let card: ActCard = serde_yaml::from_reader(&file)?;

    // Image
    let image_path = Path::new("img/templates/act-card.png");
    let card_image = texture_creator.load_texture(image_path)?;

    // Text
    let font_path = Path::new("ttf/aovel-sans-rounded.ttf");
    let clues_font = ttf_context.load_font(font_path, 36)?;
    let title_font = ttf_context.load_font(font_path, 24)?;
    let text_font = ttf_context.load_font(font_path, 14)?;

    let targets = vec![
        // Note Width / Height switched since this card is landscape
        (card_image, rect!(0, 0, CARD_HEIGHT, CARD_WIDTH)),
        card.render_clues(&clues_font, texture_creator)?,
        card.render_title(&title_font, texture_creator)?,
        card.render_text(&text_font, texture_creator)?,
    ];

    Ok(targets)
}

pub fn get_character_card_render_objects<'a, T>(
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    texture_creator: &'a sdl2::render::TextureCreator<T>,
    path_to_card: &Path,
) -> Result<Vec<(Texture<'a>, Rect)>> {
    // Load card
    let file = File::open(&path_to_card)?;
    let card: CharacterCard = serde_yaml::from_reader(&file)?;

    // Image
    let image_path = Path::new("img/templates/character-card.png");
    let card_image = texture_creator.load_texture(image_path)?;

    // Text
    let font_path = Path::new("ttf/aovel-sans-rounded.ttf");
    let stats_font = ttf_context.load_font(font_path, 18)?;
    let title_font = ttf_context.load_font(font_path, 24)?;
    let text_font = ttf_context.load_font(font_path, 14)?;

    let targets = vec![
        // Note Width / Height switched since this card is landscape
        (card_image, rect!(0, 0, CARD_HEIGHT, CARD_WIDTH)),
        card.render_name(&title_font, texture_creator)?,
        card.render_combat(&stats_font, texture_creator)?,
        card.render_attunement(&stats_font, texture_creator)?,
        card.render_intellect(&stats_font, texture_creator)?,
        card.render_speed(&stats_font, texture_creator)?,
        card.render_willpower(&stats_font, texture_creator)?,
        card.render_health(&title_font, texture_creator)?,
        card.render_sanity(&title_font, texture_creator)?,
        card.render_text(&text_font, texture_creator)?,
    ];

    Ok(targets)
}

pub fn get_encounter_card_render_objects<'a, T>(
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    texture_creator: &'a sdl2::render::TextureCreator<T>,
    path_to_card: &Path,
) -> Result<Vec<(Texture<'a>, Rect)>> {
    // Load card
    let file = File::open(&path_to_card)?;
    let card: EncounterCard = serde_yaml::from_reader(&file)?;

    // Image
    let image_path = Path::new("img/templates/encounter-card.png");
    let card_image = texture_creator.load_texture(image_path)?;

    // Text
    let font_path = Path::new("ttf/aovel-sans-rounded.ttf");
    let title_font = ttf_context.load_font(font_path, 24)?;
    let text_font = ttf_context.load_font(font_path, 14)?;

    let targets = vec![
        (card_image, rect!(0, 0, CARD_WIDTH, CARD_HEIGHT)),
        card.render_title(&title_font, texture_creator)?,
        card.render_text(&text_font, texture_creator)?,
    ];

    Ok(targets)
}

pub fn get_enemy_card_render_objects<'a, T>(
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    texture_creator: &'a sdl2::render::TextureCreator<T>,
    path_to_card: &Path,
) -> Result<Vec<(Texture<'a>, Rect)>> {
    // Load card
    let file = File::open(&path_to_card)?;
    let card: EnemyCard = serde_yaml::from_reader(&file)?;

    // Image
    let image_path = Path::new("img/templates/enemy-card.png");
    let card_image = texture_creator.load_texture(image_path)?;

    // Text
    let font_path = Path::new("ttf/aovel-sans-rounded.ttf");
    let number_font = ttf_context.load_font(font_path, 36)?;
    let title_font = ttf_context.load_font(font_path, 24)?;
    let text_font = ttf_context.load_font(font_path, 14)?;

    let targets = vec![
        (card_image, rect!(0, 0, CARD_WIDTH, CARD_HEIGHT)),
        card.render_health(&number_font, texture_creator)?,
        card.render_strength(&number_font, texture_creator)?,
        card.render_speed(&number_font, texture_creator)?,
        card.render_title(&title_font, texture_creator)?,
        card.render_text(&text_font, texture_creator)?,
    ];

    Ok(targets)
}

pub fn get_location_card_render_objects<'a, T>(
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    texture_creator: &'a sdl2::render::TextureCreator<T>,
    path_to_card: &Path,
) -> Result<Vec<(Texture<'a>, Rect)>> {
    // Load card
    let file = File::open(&path_to_card)?;
    let card: LocationCard = serde_yaml::from_reader(&file)?;

    // Image
    let image_path = Path::new("img/templates/location-card.png");
    let card_image = texture_creator.load_texture(image_path)?;

    // Text
    let font_path = Path::new("ttf/aovel-sans-rounded.ttf");
    let number_font = ttf_context.load_font(font_path, 36)?;
    let title_font = ttf_context.load_font(font_path, 24)?;
    let text_font = ttf_context.load_font(font_path, 14)?;

    let targets = vec![
        (card_image, rect!(0, 0, CARD_WIDTH, CARD_HEIGHT)),
        card.render_shroud(&number_font, texture_creator)?,
        card.render_num_clues(&number_font, texture_creator)?,
        card.render_title(&title_font, texture_creator)?,
        card.render_tags(&title_font, texture_creator)?,
        card.render_text(&text_font, texture_creator)?,
    ];

    Ok(targets)
}

fn render_and_save(
    textures_and_targets: Vec<(Texture, Rect)>,
    mut canvas: Canvas<Surface>,
    path_to_output: &Path,
) -> Result<()> {
    canvas.clear();
    for (texture, target) in textures_and_targets {
        canvas.copy(&texture, None, target)?;
    }
    canvas.present();

    canvas.surface().save_bmp(path_to_output)?;

    Ok(())
}

pub fn run(path_to_card: &Path, card_type: CardType, path_to_output: &Path) -> Result<()> {
    let ttf_context = sdl2::ttf::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let surface = match card_type {
        CardType::Act | CardType::Agenda | CardType::Character => {
            Surface::new(CARD_HEIGHT, CARD_WIDTH, PixelFormatEnum::RGB24)?
        }
        _ => Surface::new(CARD_WIDTH, CARD_HEIGHT, PixelFormatEnum::RGB24)?,
    };
    let canvas = Canvas::from_surface(surface)?;
    let texture_creator = canvas.texture_creator();

    let textures_and_targets = match card_type {
        CardType::Act => get_act_card_render_objects(&ttf_context, &texture_creator, path_to_card)?,
        CardType::Agenda => {
            get_agenda_card_render_objects(&ttf_context, &texture_creator, path_to_card)?
        }
        CardType::Character => {
            get_character_card_render_objects(&ttf_context, &texture_creator, path_to_card)?
        }
        CardType::Player => {
            get_player_card_render_objects(&ttf_context, &texture_creator, path_to_card)?
        }
        CardType::Location => {
            get_location_card_render_objects(&ttf_context, &texture_creator, path_to_card)?
        }
        CardType::Encounter => {
            get_encounter_card_render_objects(&ttf_context, &texture_creator, path_to_card)?
        }
        CardType::Enemy => {
            get_enemy_card_render_objects(&ttf_context, &texture_creator, path_to_card)?
        }
    };

    render_and_save(textures_and_targets, canvas, path_to_output)?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 4 {
        return Err(Error::Sdl2Error(String::from(
            "Usage: cargo run -- [path-to-card] [card-type] [path-to-output]",
        )));
    }

    let card_type = CardType::from_str(&args[2])?;

    run(Path::new(&args[1]), card_type, Path::new(&args[3]))?;

    Ok(())
}
