use std::convert::TryFrom;
use assert_matches::assert_matches;

#[derive(Debug, Eq, PartialEq)]
enum Error {
    InvalidCellType(String),
}

type Result<T> = std::result::Result<T, Error>;

/// A single Cell of a Room
#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    StartPosition,
    Wall,
    Teleporter(u8),
    Exit,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Empty),
            'S' => Ok(Self::StartPosition),
            '#' => Ok(Self::Wall),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Ok(Self::Teleporter(c.to_digit(10 /* radix */).unwrap() as u8))
            },
            'E' => Ok(Self::Exit),
            _ => Err(Error::InvalidCellType(format!("Invalid cell type: '{}'", c))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_try_from() {
        assert_eq!(Cell::try_from('.').unwrap(), Cell::Empty);
        assert_eq!(Cell::try_from('S').unwrap(), Cell::StartPosition);
        assert_eq!(Cell::try_from('#').unwrap(), Cell::Wall);
        assert_eq!(Cell::try_from('0').unwrap(), Cell::Teleporter(0));
        assert_eq!(Cell::try_from('E').unwrap(), Cell::Exit);
        assert_matches!(Cell::try_from('?'), Err(Error::InvalidCellType(_)));
    }
}
