#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Suit {
    Arcane,
    Beast,
    Discord,
    Hearth,
    Nomad,
    Order,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Blue,
    Orange,
    Purple,
    White,
    Yellow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Alignment {
    Chancellor,
    Citizen,
    Exile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Banner {
    DarkestSecret { secrets: u8 },
    PeoplesFavor { favor: u8, is_mob: bool },
}

enum Title {
    Oathkeeper,
    Usurper,
}

enum Vision {
    Conspiracy,
    Faith,     // Holds Darkest Secret
    Rebellion, // Holds Peoples Favor
    Sanctuary, // Most Relics + Banners
    Conquest,  // Most Sites
}
