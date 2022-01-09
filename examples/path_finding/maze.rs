use assert_matches::assert_matches;
use std::convert::From;
use std::io::Read;

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

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'S' => Self::StartPosition,
            '#' => Self::Wall,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Self::Teleporter(c.to_digit(10 /* radix */).unwrap() as u8)
            }
            'E' => Self::Exit,
            _ => panic!("Invalid cell type: '{}'", c),
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
    fn from_reader<T: Read>(mut reader: T) -> Self {
        let mut header = [0u8; 5];
        reader.read(&mut header).unwrap();

        let (input_format, number_of_rooms, room_size) = match header {
            [input_format, 10 /* \n */, number_of_rooms_char, 10 /* \n */, room_size_char] => {
                (
                    match input_format as char {
                        'L' => Format::List,
                        'M' => Format::Map,
                        _ => panic!("Invalid input format mode: '{}'", input_format),
                    },
                    match (number_of_rooms_char as char).to_digit(10 /* radix */) {
                        Some(number_of_rooms) => number_of_rooms,
                        None => panic!("Invalid number of rooms: '{}'", number_of_rooms_char),
                    },
                    match (room_size_char as char).to_digit(10 /* radix */) {
                        Some(room_size) => room_size,
                        None => panic!("Invalid room size: '{}'", room_size_char),
                    },
                )
            }
            _ => panic!("Invalid header format: {:?}", header),
        };

        Maze {
            rooms: vec![
                Room {
                    rows: vec![vec![Cell::Empty; room_size as usize]; room_size as usize]
                };
                number_of_rooms as usize
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_from() {
        assert_eq!(Cell::from('.'), Cell::Empty);
        assert_eq!(Cell::from('S'), Cell::StartPosition);
        assert_eq!(Cell::from('#'), Cell::Wall);
        assert_eq!(Cell::from('0'), Cell::Teleporter(0));
        assert_eq!(Cell::from('E'), Cell::Exit);
    }

    #[test]
    fn maze_from_reader_list() {
        let input = include_bytes!("test_input/sample-list.maze");
        let maze = Maze::from_reader(input.as_ref());

        assert_eq!(maze.rooms.len(), 2);
        for room in maze.rooms {
            assert_eq!(room.rows.len(), 4);
            for row in room.rows {
                assert_eq!(row.len(), 4);
            }
        }
    }
}
