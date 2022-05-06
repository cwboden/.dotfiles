pub use crate::logic::power::Bowl as PowerBowl;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PowerEvent {
    Add(u8),
    Charge(u8),
    Discard(u8),
    Force(u8),
    Reserve(u8),
    Spend(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    Ore,
    Credit,
    Knowledge,
    Qic,
    Power,
}

pub enum Building {
    PlanetaryInstitute,
    Academy,
    TradingStation,
    ResearchLab,
    Mine,
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoverActionType {
    // Power Actions
    GainThreePower,
    SingleTerraform,
    TwoKnowledge,
    SevenCredits,
    TwoOre,
    DoubleTerraform,
    ThreeKnowledge,

    // QIC Actions
    PointsForPlanetTypes,
    RescoreFederationToken,
    GainTechTile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResearchType {
    Terraforming,
    Flight,
    ArtificialIntelligence,
    Gaiaforming,
    Economics,
    Science,
}
