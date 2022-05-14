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

impl Resource {
    pub fn conversion_rate(&self, into: Resource) -> u8 {
        match self {
            Resource::Ore => match into {
                Resource::Ore => 1,
                Resource::Credit => 1,
                Resource::Knowledge => 0,
                Resource::Qic => 0,
                Resource::PowerCharge => 0,
                Resource::PowerTokens => 1,
            },
            Resource::Credit => match into {
                Resource::Ore => 0,
                Resource::Credit => 1,
                Resource::Knowledge => 0,
                Resource::Qic => 0,
                Resource::PowerCharge => 0,
                Resource::PowerTokens => 0,
            },
            Resource::Knowledge => match into {
                Resource::Ore => 0,
                Resource::Credit => 1,
                Resource::Knowledge => 1,
                Resource::Qic => 0,
                Resource::PowerCharge => 0,
                Resource::PowerTokens => 0,
            },
            Resource::Qic => match into {
                Resource::Ore => 1,
                Resource::Credit => 1,
                Resource::Knowledge => 0,
                Resource::Qic => 1,
                Resource::PowerCharge => 0,
                Resource::PowerTokens => 1,
            },

            Resource::PowerCharge => match into {
                Resource::Ore => 3,
                Resource::Credit => 1,
                Resource::Knowledge => 4,
                Resource::Qic => 4,
                Resource::PowerCharge => 0,
                Resource::PowerTokens => 0,
            },
            Resource::PowerTokens => match into {
                Resource::Ore => 0,
                Resource::Credit => 0,
                Resource::Knowledge => 0,
                Resource::Qic => 0,
                Resource::PowerCharge => 2,
                Resource::PowerTokens => 0,
            },
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Amount {
    units: [u8; 6],
}

impl Amount {
    pub fn new() -> Self {
        Default::default()
    }

    fn map_index(resource: Resource) -> usize {
        resource as usize
    }

    pub fn with(mut self, resource: Resource, amount: u8) -> Self {
        self.units[Self::map_index(resource)] += amount;
        self
    }

    pub fn new_singular(resource: Resource, amount: u8) -> Self {
        let self_ = Self::new().with(resource, amount);
        self_
    }

    pub fn get(&self, resource: Resource) -> u8 {
        self.units[Self::map_index(resource)]
    }

    pub fn multiply(&mut self, factor: u8) {
        self.units.iter_mut().for_each(|unit| *unit *= factor);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CoverActionEvent {
    Cover(CoverActionType),
    Reset,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PaymentEvent {
    Gain(Amount),
    CoverAction(CoverActionEvent),
    Research(ResearchType),
    Terraform((PlanetType, PlanetType)),
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
            FederationToken::EightPointsQic => Amount::new_singular(Resource::Qic, 1),
            FederationToken::EightPointsTwoPower => Amount::new_singular(Resource::PowerTokens, 2),
            FederationToken::SevenPointsSixCredits => Amount::new_singular(Resource::Credit, 6),
            FederationToken::SevenPointsTwoOre => Amount::new_singular(Resource::Ore, 2),
            FederationToken::SixPointsTwoKnowledge => Amount::new_singular(Resource::Knowledge, 2),
            FederationToken::TwelvePoints => Amount::new_singular(Resource::Ore, 0),
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
    fn resource_conversion_rate_ore() {
        assert_eq!(Resource::Ore.conversion_rate(Resource::Ore), 1);
        assert_eq!(Resource::Ore.conversion_rate(Resource::Credit), 1);
        assert_eq!(Resource::Ore.conversion_rate(Resource::Knowledge), 0);
        assert_eq!(Resource::Ore.conversion_rate(Resource::Qic), 0);
        assert_eq!(Resource::Ore.conversion_rate(Resource::PowerCharge), 0);
        assert_eq!(Resource::Ore.conversion_rate(Resource::PowerTokens), 1);
    }

    #[test]
    fn resource_conversion_rate_credit() {
        assert_eq!(Resource::Credit.conversion_rate(Resource::Ore), 0);
        assert_eq!(Resource::Credit.conversion_rate(Resource::Credit), 1);
        assert_eq!(Resource::Credit.conversion_rate(Resource::Knowledge), 0);
        assert_eq!(Resource::Credit.conversion_rate(Resource::Qic), 0);
        assert_eq!(Resource::Credit.conversion_rate(Resource::PowerCharge), 0);
        assert_eq!(Resource::Credit.conversion_rate(Resource::PowerTokens), 0);
    }

    #[test]
    fn resource_conversion_rate_knowledge() {
        assert_eq!(Resource::Knowledge.conversion_rate(Resource::Ore), 0);
        assert_eq!(Resource::Knowledge.conversion_rate(Resource::Credit), 1);
        assert_eq!(Resource::Knowledge.conversion_rate(Resource::Knowledge), 1);
        assert_eq!(Resource::Knowledge.conversion_rate(Resource::Qic), 0);
        assert_eq!(
            Resource::Knowledge.conversion_rate(Resource::PowerCharge),
            0
        );
        assert_eq!(
            Resource::Knowledge.conversion_rate(Resource::PowerTokens),
            0
        );
    }

    #[test]
    fn resource_conversion_rate_qic() {
        assert_eq!(Resource::Qic.conversion_rate(Resource::Ore), 1);
        assert_eq!(Resource::Qic.conversion_rate(Resource::Credit), 1);
        assert_eq!(Resource::Qic.conversion_rate(Resource::Knowledge), 0);
        assert_eq!(Resource::Qic.conversion_rate(Resource::Qic), 1);
        assert_eq!(Resource::Qic.conversion_rate(Resource::PowerCharge), 0);
        assert_eq!(Resource::Qic.conversion_rate(Resource::PowerTokens), 1);
    }

    #[test]
    fn resource_conversion_rate_power_charge() {
        assert_eq!(Resource::PowerCharge.conversion_rate(Resource::Ore), 3);
        assert_eq!(Resource::PowerCharge.conversion_rate(Resource::Credit), 1);
        assert_eq!(
            Resource::PowerCharge.conversion_rate(Resource::Knowledge),
            4
        );
        assert_eq!(Resource::PowerCharge.conversion_rate(Resource::Qic), 4);
        assert_eq!(
            Resource::PowerCharge.conversion_rate(Resource::PowerCharge),
            0
        );
        assert_eq!(
            Resource::PowerCharge.conversion_rate(Resource::PowerTokens),
            0
        );
    }

    #[test]
    fn resource_conversion_rate_power_tokens() {
        assert_eq!(Resource::PowerTokens.conversion_rate(Resource::Ore), 0);
        assert_eq!(Resource::PowerTokens.conversion_rate(Resource::Credit), 0);
        assert_eq!(
            Resource::PowerTokens.conversion_rate(Resource::Knowledge),
            0
        );
        assert_eq!(Resource::PowerTokens.conversion_rate(Resource::Qic), 0);
        assert_eq!(
            Resource::PowerTokens.conversion_rate(Resource::PowerCharge),
            2
        );
        assert_eq!(
            Resource::PowerTokens.conversion_rate(Resource::PowerTokens),
            0
        );
    }

    #[test]
    fn amount_multiply() {
        let mut amount = Amount::new_singular(Resource::Ore, 1);
        amount.multiply(3);
        assert_eq!(amount.get(Resource::Ore), 3);
    }

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
