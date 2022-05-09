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
