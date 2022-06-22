use crate::engine::types::{Alignment, Banner, Color};
use crate::engine::warbands::Warband;

pub struct Board {
    favor: u8,
    secrets: u8,
    warbands: Bank,
    advisers: Vec<Card>,
    relics: Vec<Relic>,
    banners: Vec<Banner>,
    title: Option<Title>,
    vision: Option<Vision>,
}


/// Represents the player's board and their holdings
/// Is it better to have 3 types of boards (one for each type of player) or just use one
/// (they work the same in a lot of ways but could benefit from split logic)
impl Board {
    fn new(alignment: Alignment, color: Color) -> Self {
        Self {
            favor: if alignment == Chancellor { 2 } else { 1 },
            secrets: 1,
            warbands: Bank::new( if alignment == Citizen { Color::Purple } else { color }, 3),
            advisers: Vec<Card>::with_capacity(3),
            relics: Vec<Relic>::new(),
            banners: Vec<Banner>::with_capacity(2),
            title: if alignment == Chancellor { Oathkeeper } else { Option::None },
            vision: Option::None,
        }
    }
}
