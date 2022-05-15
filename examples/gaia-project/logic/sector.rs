use crate::types::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Space {
    Empty,
    Planet(PlanetType),
}

#[derive(Debug, Eq, PartialEq)]
struct Sector {
    spaces: [Space; 19],
}

#[derive(Debug, Eq, PartialEq)]
struct Coordinate {
    pub x: i8,
    pub y: i8,
}

impl Coordinate {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}

impl Sector {
    const NUM_01: Self = Self {
        spaces: [
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Planet(PlanetType::Brown),
            Space::Planet(PlanetType::Yellow),
            Space::Empty,
            Space::Planet(PlanetType::Blue),
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Planet(PlanetType::Transdim),
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Planet(PlanetType::Orange),
            Space::Planet(PlanetType::Red),
        ],
    };

    pub fn new(spaces: [Space; 19]) -> Self {
        Self { spaces }
    }

    pub fn at(&self, coordinate: Coordinate) -> Space {
        let index = match (coordinate.x, coordinate.y) {
            (-2, 2) => 0,
            (-1, 2) => 1,
            (0, 2) => 2,
            (-2, 1) => 3,
            (-1, 1) => 4,
            (0, 1) => 5,
            (1, 1) => 6,
            (-2, 0) => 7,
            (-1, 0) => 8,
            (0, 0) => 9,
            (1, 0) => 10,
            (2, 0) => 11,
            (-1, -1) => 12,
            (0, -1) => 13,
            (1, -1) => 14,
            (2, -1) => 15,
            (0, -2) => 16,
            (1, -2) => 17,
            (2, -2) => 18,
            _ => panic!("Invalid coordinate: {coordinate:?}"),
        };

        self.spaces[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sector_num_01_at() {
        assert_eq!(Sector::NUM_01.at(Coordinate::new(0, 0)), Space::Empty);
        assert_eq!(
            Sector::NUM_01.at(Coordinate::new(2, -2)),
            Space::Planet(PlanetType::Red)
        );
        assert_eq!(
            Sector::NUM_01.at(Coordinate::new(1, -2)),
            Space::Planet(PlanetType::Orange)
        );
        assert_eq!(
            Sector::NUM_01.at(Coordinate::new(1, 1)),
            Space::Planet(PlanetType::Yellow)
        );
        assert_eq!(
            Sector::NUM_01.at(Coordinate::new(0, 1)),
            Space::Planet(PlanetType::Brown)
        );
        assert_eq!(
            Sector::NUM_01.at(Coordinate::new(-1, 0)),
            Space::Planet(PlanetType::Blue)
        );
        assert_eq!(
            Sector::NUM_01.at(Coordinate::new(-1, -1)),
            Space::Planet(PlanetType::Transdim)
        );
    }
}
