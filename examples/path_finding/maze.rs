use assert_matches::assert_matches;
use std::convert::TryFrom;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
enum Error {
    InvalidCellType(String),
    InvalidHeader(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    List,
    Map,
}

/// A single Cell of a Room
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
            }
            'E' => Ok(Self::Exit),
            _ => Err(Error::InvalidCellType(format!(
                "Invalid cell type: '{}'",
                c
            ))),
        }
    }
}

#[derive(Clone)]
struct Room {
    rows: Vec<Vec<Cell>>,
}

pub struct Maze {
    rooms: Vec<Room>,
}

impl Maze {
    fn from_reader<T: Read>(mut reader: T) -> Result<Self> {
        let mut header = [0u8; 5];
        reader.read(&mut header).unwrap();

        let (input_format, number_of_rooms, room_size) = match header {
            [input_format, 10 /* \n */, number_of_rooms_char, 10 /* \n */, room_size_char] => {
                (
                    match input_format as char {
                        'L' => Format::List,
                        'M' => Format::Map,
                        _ => return Err(Error::InvalidHeader(format!("Invalid input format mode: '{}'", input_format))),
                    },
                    match (number_of_rooms_char as char).to_digit(10 /* radix */) {
                        Some(number_of_rooms) => number_of_rooms,
                        None => return Err(Error::InvalidHeader(format!("Invalid number of rooms: '{}'", number_of_rooms_char))),
                    },
                    match (room_size_char as char).to_digit(10 /* radix */) {
                        Some(room_size) => room_size,
                        None => return Err(Error::InvalidHeader(format!("Invalid room size: '{}'", room_size_char))),
                    },
                )
            }
            _ => return Err(Error::InvalidHeader(format!("Invalid header format: {:?}", header))),
        };

        Ok(Maze {
            rooms: vec![
                Room {
                    rows: vec![vec![Cell::Empty; room_size as usize]; room_size as usize]
                };
                number_of_rooms as usize
            ],
        })
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

    #[test]
    fn maze_from_reader_list() {
        let input = include_bytes!("test_input/sample-list.maze");
        let maze = Maze::from_reader(input.as_ref()).unwrap();

        assert_eq!(maze.rooms.len(), 2);
        for room in maze.rooms {
            assert_eq!(room.rows.len(), 4);
            for row in room.rows {
                assert_eq!(row.len(), 4);
            }
        }
    }
}
