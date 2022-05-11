use strum_macros::EnumIter;

pub use crate::logic::cover_action::Type as CoverActionType;
pub use crate::logic::power::Bowl as PowerBowl;
pub use crate::logic::research::Type as ResearchType;

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq)]
pub enum Resource {
    Ore,
    Credit,
    Knowledge,
    Qic,
    PowerCharge,
    PowerTokens,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Amount {
    pub resource: Resource,
    pub amount: u8,
}

impl Amount {
    pub fn new(resource: Resource, amount: u8) -> Self {
        Self { resource, amount }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CoverActionEvent {
    Cover(CoverActionType),
    Reset,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentEvent {
    Gain(Amount),
    CoverAction(CoverActionEvent),
    Research(ResearchType),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ResearchEvent {
    Advance(ResearchType),
}

pub enum Building {
    PlanetaryInstitute,
    Academy,
    TradingStation,
    ResearchLab,
    Mine,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq)]
pub enum FederationToken {
    EightPointsQic,
    EightPointsTwoPower,
    SevenPointsSixCredits,
    SevenPointsTwoOre,
    SixPointsTwoKnowledge,
    TwelvePoints,

    // Special token, for the Gleens
    OneOreOneKnowledgeTwoCredits,
}

impl Into<Amount> for FederationToken {
    fn into(self) -> Amount {
        match self {
            FederationToken::EightPointsQic => Amount::new(Resource::Qic, 1),
            FederationToken::EightPointsTwoPower => Amount::new(Resource::PowerTokens, 2),
            FederationToken::SevenPointsSixCredits => Amount::new(Resource::Credit, 6),
            FederationToken::SevenPointsTwoOre => Amount::new(Resource::Ore, 2),
            FederationToken::SixPointsTwoKnowledge => Amount::new(Resource::Knowledge, 2),
            FederationToken::TwelvePoints => Amount::new(Resource::Ore, 0),
            FederationToken::OneOreOneKnowledgeTwoCredits => {
                panic!("XXX: Cannot convert into an amount with multiple resources yet");
            }
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq)]
pub enum PlanetType {
    Yellow,
    Orange,
    Brown,
    White,
    Gray,
    Blue,
    Red,
    Gaia,
    LostPlanet,
}

impl PlanetType {
    const STANDARD_PLANETS: usize = 7;

    pub fn terraforms_from(self, other: PlanetType) -> u8 {
        if self == PlanetType::Gaia || self == PlanetType::LostPlanet {
            panic!("Invalid `self` type: {self:?}!");
        }
        if other == PlanetType::Gaia || other == PlanetType::LostPlanet {
            panic!("Invalid `other` type: {self:?}!");
        }

        let self_u8 = self as u8;
        let other_u8 = other as u8;

        println!("{self:?}: {self_u8}, {other:?}: {other_u8}");

        let max = std::cmp::max(self_u8, other_u8);
        let min = std::cmp::min(self_u8, other_u8);

        max - min
    }
}

pub enum StandardTechTile {
    OneOreOneQic,
    OneKnowledgePerPlanetType,
    ThreePowerBuildingsToFourPower,
    SevenPoints,
    OneOreProductionOnePowerCharge,
    OneKnowledgeAndOneCreditProduction,
    ThreePointsWhenSettlingGaiaPlanets,
    FourCreditProduction,
    ChargeFourPowerAction,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FederationTokenEvent {
    Take(FederationToken),
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn terraforms_from_zero_steps_from_self() {
        PlanetType::iter()
            .take(
                PlanetType::STANDARD_PLANETS, // ignore Gaia and Lost Planet
            )
            .for_each(|t| {
                assert_eq!(t.terraforms_from(t), 0);
            });
    }

    fn do_terraform_from_n_steps_away_test(n: u8) {
        let standard_planets = PlanetType::iter().take(
            PlanetType::STANDARD_PLANETS, // ignore Gaia and Lost Planet
        );
        zip(standard_planets.clone(), standard_planets.skip(n.into())).for_each(|(t, t_plus_n)| {
            assert_eq!(t.terraforms_from(t_plus_n), n);
            assert_eq!(t_plus_n.terraforms_from(t), n);
        });
    }

    #[test]
    fn terraforms_from_one_step_away() {
        do_terraform_from_n_steps_away_test(1);
    }

    #[test]
    fn terraforms_from_two_steps_away() {
        do_terraform_from_n_steps_away_test(2);
    }

    #[test]
    fn terraforms_from_three_steps_away() {
        do_terraform_from_n_steps_away_test(3);
    }
}
