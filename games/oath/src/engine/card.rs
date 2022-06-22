use crate::engine::types::Suit;

struct Cost {
    favor: u8,
    secrets: u8,
    burn_secrets: u8,
    burn_favor: u8,
}

enum CardRestrictionType {
    AdvisorOnly,
    SiteOnly,
}

struct CardRestriction {
    restriction: CardRestrictionType,
    locked: bool,
}

pub struct Card {
    name: String,
    suit: Suit,
    restriction: Option<CardRestriction>,
    description: String,
    cost: Cost,
}
