use crate::error;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum CardType {
    Act,
    Agenda,
    Character,
    Encounter,
    Enemy,
    Location,
    Player,
}

impl FromStr for CardType {
    type Err = error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Act" => Ok(CardType::Act),
            "Agenda" => Ok(CardType::Agenda),
            "Character" => Ok(CardType::Character),
            "Encounter" => Ok(CardType::Encounter),
            "Enemy" => Ok(CardType::Enemy),
            "Location" => Ok(CardType::Location),
            "Player" => Ok(CardType::Player),
            _ => Err(error::ParseError::new(format!(
                "Could not parse CardType from: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum PlayerCardClass {
    Character,
    Researcher,
    Runner,
    Scavenger,
    Seeker,
    Soldier,
}

impl FromStr for PlayerCardClass {
    type Err = error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Character" => Ok(PlayerCardClass::Character),
            "Researcher" => Ok(PlayerCardClass::Researcher),
            "Runner" => Ok(PlayerCardClass::Runner),
            "Scavenger" => Ok(PlayerCardClass::Scavenger),
            "Seeker" => Ok(PlayerCardClass::Seeker),
            "Soldier" => Ok(PlayerCardClass::Soldier),
            _ => Err(error::ParseError::new(format!(
                "Could not parse PlayerCardClass from: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum PlayerCardType {
    Asset,
    Event,
    Skill,
}

impl FromStr for PlayerCardType {
    type Err = error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Asset" => Ok(PlayerCardType::Asset),
            "Event" => Ok(PlayerCardType::Event),
            "Skill" => Ok(PlayerCardType::Skill),
            _ => Err(error::ParseError::new(format!(
                "Could not parse PlayerCardType from: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum PlayerCardSlotType {
    Accessory,
    Body,
    Head,
    Legs,
    OneHand,
    Relic,
    Spell,
    TwoHands,
}

impl FromStr for PlayerCardSlotType {
    type Err = error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Accessory" => Ok(PlayerCardSlotType::Accessory),
            "Body" => Ok(PlayerCardSlotType::Body),
            "Head" => Ok(PlayerCardSlotType::Head),
            "Legs" => Ok(PlayerCardSlotType::Legs),
            "OneHand" => Ok(PlayerCardSlotType::OneHand),
            "Relic" => Ok(PlayerCardSlotType::Relic),
            "Spell" => Ok(PlayerCardSlotType::Spell),
            "TwoHands" => Ok(PlayerCardSlotType::TwoHands),
            _ => Err(error::ParseError::new(format!(
                "Could not parse PlayerCardSlotType from: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn card_type_succeeds_from_valid_strings() {
        assert_eq!(CardType::from_str("Act"), Ok(CardType::Act));
        assert_eq!(CardType::from_str("Agenda"), Ok(CardType::Agenda));
        assert_eq!(CardType::from_str("Encounter"), Ok(CardType::Encounter));
        assert_eq!(CardType::from_str("Enemy"), Ok(CardType::Enemy));
        assert_eq!(CardType::from_str("Location"), Ok(CardType::Location));
        assert_eq!(CardType::from_str("Player"), Ok(CardType::Player));
    }

    #[test]
    fn card_type_fails_from_invalid_strings() {
        assert_eq!(
            CardType::from_str("Invalid"),
            Err(error::ParseError::new(
                "Could not parse CardType from: Invalid".to_string()
            )),
        );
    }

    #[test]
    fn player_card_class_succeeds_from_valid_strings() {
        assert_eq!(
            PlayerCardClass::from_str("Researcher"),
            Ok(PlayerCardClass::Researcher)
        );
        assert_eq!(
            PlayerCardClass::from_str("Runner"),
            Ok(PlayerCardClass::Runner)
        );
        assert_eq!(
            PlayerCardClass::from_str("Scavenger"),
            Ok(PlayerCardClass::Scavenger)
        );
        assert_eq!(
            PlayerCardClass::from_str("Seeker"),
            Ok(PlayerCardClass::Seeker)
        );
        assert_eq!(
            PlayerCardClass::from_str("Soldier"),
            Ok(PlayerCardClass::Soldier)
        );
    }

    #[test]
    fn player_card_class_fails_from_invalid_strings() {
        assert_eq!(
            PlayerCardClass::from_str("Invalid"),
            Err(error::ParseError::new(
                "Could not parse PlayerCardClass from: Invalid".to_string()
            )),
        );
    }

    #[test]
    fn player_card_type_succeeds_from_valid_strings() {
        assert_eq!(
            PlayerCardType::from_str("Asset"),
            Ok(PlayerCardType::Asset)
        );
        assert_eq!(
            PlayerCardType::from_str("Event"),
            Ok(PlayerCardType::Event)
        );
        assert_eq!(
            PlayerCardType::from_str("Skill"),
            Ok(PlayerCardType::Skill)
        );
    }

    #[test]
    fn player_card_type_fails_from_invalid_strings() {
        assert_eq!(
            PlayerCardType::from_str("Invalid"),
            Err(error::ParseError::new(
                "Could not parse PlayerCardType from: Invalid".to_string()
            )),
        );
    }

    #[test]
    fn player_card_slot_type_succeeds_from_valid_strings() {
        assert_eq!(
            PlayerCardSlotType::from_str("Accessory"),
            Ok(PlayerCardSlotType::Accessory)
        );
        assert_eq!(
            PlayerCardSlotType::from_str("Body"),
            Ok(PlayerCardSlotType::Body)
        );
        assert_eq!(
            PlayerCardSlotType::from_str("Head"),
            Ok(PlayerCardSlotType::Head)
        );
        assert_eq!(
            PlayerCardSlotType::from_str("Legs"),
            Ok(PlayerCardSlotType::Legs)
        );
        assert_eq!(
            PlayerCardSlotType::from_str("OneHand"),
            Ok(PlayerCardSlotType::OneHand)
        );
        assert_eq!(
            PlayerCardSlotType::from_str("Relic"),
            Ok(PlayerCardSlotType::Relic)
        );
        assert_eq!(
            PlayerCardSlotType::from_str("Spell"),
            Ok(PlayerCardSlotType::Spell)
        );
        assert_eq!(
            PlayerCardSlotType::from_str("TwoHands"),
            Ok(PlayerCardSlotType::TwoHands)
        );
    }

    #[test]
    fn player_card_slot_type_fails_from_invalid_strings() {
        assert_eq!(
            PlayerCardSlotType::from_str("Invalid"),
            Err(error::ParseError::new(
                "Could not parse PlayerCardSlotType from: Invalid".to_string()
            )),
        );
    }
}
