use std::convert::From;
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    List,
    Map,
}

/// A single Cell of a Room
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
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
pub struct Room {
    pub rows: Vec<Vec<Cell>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coordinate {
    pub room: u32,
    pub row: u32,
    pub column: u32,
}

pub struct Maze {
    pub rooms: Vec<Room>,
    pub starting_position: Option<Coordinate>,
}

impl Maze {
    fn set_start_position(&mut self, room: u32, row: u32, column: u32) {
        if self.starting_position.is_some() {
            panic!("Multiple start positions found in maze");
        }

        self.starting_position = Some(Coordinate { room, row, column });
    }

    fn parse_list<T: BufRead>(mut self, reader: T) -> Self {
        for line in reader.lines() {
            let line = line.unwrap();

            // Ignore comments
            if line.starts_with('/') {
                continue;
            }

            // Given input should be of the form:
            // ({room number},{row},{column},{cell type})
            let room_number = line
                .chars()
                .nth(1)
                .unwrap()
                .to_digit(10 /* radix */)
                .unwrap();
            let row = line
                .chars()
                .nth(3)
                .unwrap()
                .to_digit(10 /* radix */)
                .unwrap();
            let column = line
                .chars()
                .nth(5)
                .unwrap()
                .to_digit(10 /* radix */)
                .unwrap();
            let cell_type = Cell::from(line.chars().nth(7).unwrap());

            if cell_type == Cell::StartPosition {
                self.set_start_position(room_number, row, column);
            }

            self.rooms[room_number as usize].rows[row as usize][column as usize] = cell_type;
        }

        self
    }

    fn parse_map<T: BufRead>(mut self, reader: T) -> Self {
        let mut row_index = 0;
        let mut room_index = 0;
        for line in reader.lines() {
            let line = line.unwrap();

            // Ignore comments
            if line.starts_with('/') {
                continue;
            }

            let mut column_index = 0;
            for character in line.chars() {
                let cell_type = Cell::from(character);
                self.rooms[room_index].rows[row_index][column_index] = cell_type;

                if cell_type == Cell::StartPosition {
                    self.set_start_position(
                        room_index as u32,
                        row_index as u32,
                        column_index as u32,
                    );
                }

                column_index += 1;
            }

            row_index += 1;
            // Continue to new room once processing all N rows
            if row_index == self.rooms[0].rows.len() {
                row_index = 0;
                room_index += 1;
            }
        }

        self
    }

    pub fn from_reader<T: BufRead>(mut reader: T) -> Self {
        let mut header = [0u8; 6];
        reader.read(&mut header).unwrap();

        let (input_format, number_of_rooms, room_size) = match header {
            [
                input_format,
                10 /* \n */,
                number_of_rooms_char,
                10 /* \n */,
                room_size_char,
                10 /* \n */
            ] => {
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

        let maze = Maze {
            rooms: vec![
                Room {
                    rows: vec![vec![Cell::Empty; room_size as usize]; room_size as usize]
                };
                number_of_rooms as usize
            ],
            starting_position: None,
        };

        match input_format {
            Format::List => maze.parse_list(reader),
            Format::Map => maze.parse_map(reader),
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
        for room in &maze.rooms {
            assert_eq!(room.rows.len(), 4);
            for row in &room.rows {
                assert_eq!(row.len(), 4);
            }
        }

        assert_eq!(maze.rooms[0].rows[0][1], Cell::Exit);
        assert_eq!(maze.rooms[0].rows[2][3], Cell::StartPosition);
        assert_eq!(maze.rooms[0].rows[3][0], Cell::Wall);
        assert_eq!(maze.rooms[0].rows[3][2], Cell::Teleporter(1));
        assert_eq!(
            maze.starting_position,
            Some(Coordinate {
                room: 0,
                row: 2,
                column: 3
            })
        );
    }

    #[test]
    fn maze_from_reader_map() {
        let input = include_bytes!("test_input/sample-map.maze");
        let maze = Maze::from_reader(input.as_ref());

        assert_eq!(maze.rooms.len(), 2);
        for room in &maze.rooms {
            assert_eq!(room.rows.len(), 4);
            for row in &room.rows {
                assert_eq!(row.len(), 4);
            }
        }

        assert_eq!(maze.rooms[0].rows[0][1], Cell::Exit);
        assert_eq!(maze.rooms[0].rows[2][3], Cell::StartPosition);
        assert_eq!(maze.rooms[0].rows[3][0], Cell::Wall);
        assert_eq!(maze.rooms[0].rows[3][2], Cell::Teleporter(1));
        assert_eq!(
            maze.starting_position,
            Some(Coordinate {
                room: 0,
                row: 2,
                column: 3
            })
        );
    }
}
