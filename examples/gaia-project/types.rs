pub enum Resource {
    Ore,
    Credit,
    Knowledge,
    Qic,
    Power,
}

pub enum PowerBowl {
    One,
    Two,
    Three,
    Gaia,
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