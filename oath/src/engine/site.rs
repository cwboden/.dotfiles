use crate::engine::card::Card;
use crate::engine::relic::Relic;
use crate::engine::types::Suit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cost {
    Favor { amount: u8, suit: Suit },
    Secrets { amount: u8 },
}

// We need some good generic way to represent the Action for each site
// I am thinking we can have an Action class that holds all of the possible
// actions e.g. GetWarbandsAction would be getting X warbands when playing a
// suit of type Suit ReplaceCardAction would allow removing a card before
// playing a new one I think this would probably be a Bevy thing since that
// sounds like a system
pub struct Site {
    name: String,
    max_cards: u8,
    cards: Vec<Card>,
    relics: Vec<Relic>,
    relic_cost: Cost,
    starting_relics: u8,
    starting_favor: u8,
    starting_secrets: u8,
}

impl Site {
    pub fn new(
        name: String,
        max_cards: u8,
        relic_cost: Cost,
        starting_relics: Option<u8>,
        starting_favor: Option<u8>,
        starting_secrets: Option<u8>,
    ) -> Self {
        let starting_relics = starting_relics.unwrap_or(0);
        let starting_favor = starting_favor.unwrap_or(0);
        let starting_secrets = starting_secrets.unwrap_or(0);

        let relics: Vec<Relic> = Vec::new();

        for _ in 1..=starting_relics {
            // Pull a Relic from the Relic Deck and add to relics on this Site
            // relics.append(...);
        }

        Self {
            name,
            max_cards,
            cards: Vec::new(),
            relics,
            relic_cost,
            starting_relics,
            starting_favor,
            starting_secrets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_with_secret_cost_and_no_starting_resources() {
        let relic_cost = Cost::Secrets { amount: 1 };
        let site = Site::new("Test Site".to_string(), 3, relic_cost, None, None, None);

        assert_eq!(site.starting_favor, 0);
        assert_eq!(site.starting_relics, 0);
        assert_eq!(site.starting_secrets, 0);
        assert_eq!(site.relic_cost, relic_cost);
    }

    #[test]
    fn site_with_favor_cost_and_no_starting_resources() {
        let relic_cost = Cost::Favor {
            amount: 3,
            suit: Suit::Discord,
        };
        let site = Site::new("Test Site".to_string(), 3, relic_cost, None, None, None);

        assert_eq!(site.starting_favor, 0);
        assert_eq!(site.starting_relics, 0);
        assert_eq!(site.starting_secrets, 0);
        assert_eq!(site.relic_cost, relic_cost);
    }
}
