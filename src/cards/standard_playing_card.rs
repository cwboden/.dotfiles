use std::string::ToString;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Eq, Hash, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Spades,
    Hearts,
}

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Eq, Hash, PartialEq)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StandardPlayingCard {
    pub suit: Suit,
    pub value: Value,
}

impl StandardPlayingCard {
    pub const ALL: [Self; 52] = [
        Self {
            suit: Suit::Clubs,
            value: Value::Ace,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Two,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Three,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Four,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Five,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Six,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Seven,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Eight,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Nine,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Ten,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Jack,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::Queen,
        },
        Self {
            suit: Suit::Clubs,
            value: Value::King,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Ace,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Two,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Three,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Four,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Five,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Six,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Seven,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Eight,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Nine,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Ten,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Jack,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::Queen,
        },
        Self {
            suit: Suit::Diamonds,
            value: Value::King,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Ace,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Two,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Three,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Four,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Five,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Six,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Seven,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Eight,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Nine,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Ten,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Jack,
        },
        Self {
            suit: Suit::Spades,
            value: Value::Queen,
        },
        Self {
            suit: Suit::Spades,
            value: Value::King,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Ace,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Two,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Three,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Four,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Five,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Six,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Seven,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Eight,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Nine,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Ten,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Jack,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::Queen,
        },
        Self {
            suit: Suit::Hearts,
            value: Value::King,
        },
    ];

    pub fn new(suit: Suit, value: Value) -> Self {
        Self { suit, value }
    }
}

impl ToString for StandardPlayingCard {
    fn to_string(&self) -> String {
        format!("{} of {}", self.value.to_string(), self.suit.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use strum::IntoEnumIterator;

    #[test]
    fn standard_playing_card_all_contains_every_suit_and_value() {
        let cards_set: HashSet<StandardPlayingCard> =
            StandardPlayingCard::ALL.iter().cloned().collect();

        Value::iter().for_each(|v| {
            Suit::iter().for_each(|s| {
                assert!(cards_set.contains(&StandardPlayingCard::new(s, v)));
            });
        });
    }
}
