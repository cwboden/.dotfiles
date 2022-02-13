use std::convert::From;
use std::convert::TryInto;
use std::io::BufRead;
use serde::Serialize;

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
    Teleporter(usize),
    Exit,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'S' => Self::StartPosition,
            '#' => Self::Wall,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Self::Teleporter(c.to_digit(10 /* radix */).unwrap() as usize)
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Coordinate {
    pub room: usize,
    pub row: usize,
    pub column: usize,
}

impl Coordinate {
    /// Returns a new, valid space in the [`Maze`] in the corresponding [`Direction`]
    pub fn move_in(self, direction: &Direction) -> Coordinate {
        match direction {
            Direction::North => Coordinate {
                row: self.row.checked_sub(1).unwrap_or(usize::MAX),
                ..self
            },
            Direction::East => Coordinate {
                column: self.column.checked_add(1).unwrap_or(usize::MIN),
                ..self
            },
            Direction::South => Coordinate {
                row: self.row.checked_add(1).unwrap_or(usize::MIN),
                ..self
            },
            Direction::West => Coordinate {
                column: self.column.checked_sub(1).unwrap_or(usize::MAX),
                ..self
            },
        }
    }
}


pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Maze {
    pub rooms: Vec<Room>,
    pub starting_position: Option<Coordinate>,
}

impl Maze {
    fn set_start_position(&mut self, room: usize, row: usize, column: usize) {
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
                self.set_start_position(
                    room_number.try_into().unwrap(),
                    row.try_into().unwrap(),
                    column.try_into().unwrap(),
                );
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
                    self.set_start_position(room_index, row_index, column_index);
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
                room_size_char,
                10 /* \n */,
                number_of_rooms_char,
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

    pub fn at(&self, coordinate: Coordinate) -> Option<Cell> {
        if self.rooms.len() <= coordinate.room
            || self.rooms[coordinate.room].rows.len() <= coordinate.row
            || self.rooms[coordinate.room].rows[coordinate.row].len() <= coordinate.column
        {
            None
        } else {
            Some(self.rooms[coordinate.room].rows[coordinate.row][coordinate.column])
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

    fn assert_sample_list_valid(maze: &Maze) {
        assert_eq!(
            maze.at(Coordinate {
                room: 0,
                row: 0,
                column: 1
            }),
            Some(Cell::Exit)
        );
        assert_eq!(
            maze.at(Coordinate {
                room: 0,
                row: 2,
                column: 3
            }),
            Some(Cell::StartPosition)
        );
        assert_eq!(
            maze.at(Coordinate {
                room: 0,
                row: 3,
                column: 0
            }),
            Some(Cell::Wall)
        );
        assert_eq!(
            maze.at(Coordinate {
                room: 0,
                row: 3,
                column: 2
            }),
            Some(Cell::Teleporter(1))
        );
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

        assert_sample_list_valid(&maze);
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

        assert_sample_list_valid(&maze);
    }

    #[test]
    fn maze_at_out_of_bounds_room() {
        let input = include_bytes!("test_input/sample-map.maze");
        let maze = Maze::from_reader(input.as_ref());

        assert_eq!(
            maze.at(Coordinate {
                room: 2,
                row: 0,
                column: 0
            }),
            None
        );
    }

    #[test]
    fn maze_at_out_of_bounds_row() {
        let input = include_bytes!("test_input/sample-map.maze");
        let maze = Maze::from_reader(input.as_ref());

        assert_eq!(
            maze.at(Coordinate {
                room: 1,
                row: 42,
                column: 0
            }),
            None
        );
    }

    #[test]
    fn maze_at_out_of_bounds_column() {
        let input = include_bytes!("test_input/sample-map.maze");
        let maze = Maze::from_reader(input.as_ref());

        assert_eq!(
            maze.at(Coordinate {
                room: 1,
                row: 0,
                column: 42,
            }),
            None
        );
    }
}
