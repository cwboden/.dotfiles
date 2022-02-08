use crate::maze::{Cell, Coordinate, Direction, Maze};
use crate::{Algorithm, Format, PathFindingArgs};
use std::collections::VecDeque;

/// Abstracts the settings into an arbitrary [`SearchList`] where users can [`pop`] or [`push`] to
/// insert and remove elements respectively in an order determined by the input [`Algorithm`].
struct SearchList<T> {
    algorithm: Algorithm,
    search_list: VecDeque<T>,
}

impl<T> SearchList<T> {
    pub fn new(algorithm: Algorithm) -> Self {
        Self {
            algorithm,
            search_list: VecDeque::new(),
        }
    }

    pub fn push(&mut self, element: T) {
        self.search_list.push_back(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.algorithm {
            Algorithm::Queue => self.search_list.pop_front(),
            Algorithm::Stack => self.search_list.pop_back(),
        }
    }
}

struct Solver {
    maze: Maze,
    search_list: SearchList<Coordinate>,
}

impl Solver {
    pub fn new(algorithm: Algorithm, maze: Maze) -> Self {
        let mut search_list = SearchList::new(algorithm);
        search_list.push(maze.starting_position.unwrap());

        Self { maze, search_list }
    }

    pub fn run(mut self) -> Vec<Coordinate> {
        while let Some(origin) = self.search_list.pop() {
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .iter()
            {
                let destination = origin.move_in(direction);
                match self.maze.at(destination) {
                    Some(cell) => match cell {
                        Cell::Empty => {
                            self.search_list.push(destination);
                        }
                        Cell::StartPosition => {
                            unimplemented!();
                        }
                        Cell::Wall => {
                            // Do nothing
                        }
                        Cell::Teleporter(floor) => {
                            self.search_list.push(Coordinate {
                                room: floor,
                                ..destination
                            });
                        }
                        Cell::Exit => {
                            // TODO: This only works in the easiest possible solution
                            return vec![origin, destination];
                        }
                    },
                    None => {}
                }
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_list_queue() {
        let list = vec![1, 2, 3];
        let mut search_list = SearchList::new(Algorithm::Queue);
        for element in list.iter() {
            search_list.push(element);
        }

        for element in list.iter() {
            assert_eq!(search_list.pop(), Some(element));
        }
    }

    #[test]
    fn search_list_stack() {
        let list = vec![1, 2, 3];
        let mut search_list = SearchList::new(Algorithm::Stack);
        for element in list.iter() {
            search_list.push(element);
        }

        for element in list.iter().rev() {
            assert_eq!(search_list.pop(), Some(element));
        }
    }

    fn do_solver_integration_test(algorithm: Algorithm) {
        let input = include_bytes!("test_input/easiest-possible.maze");
        let maze = Maze::from_reader(input.as_ref());
        let mut solver = Solver::new(algorithm, maze);

        let path = solver.run();
        let expected = vec![
            Coordinate {
                room: 0,
                row: 0,
                column: 0,
            },
            Coordinate {
                room: 0,
                row: 0,
                column: 1,
            },
        ];
        assert_eq!(path, expected);
    }

    #[test]
    fn solver_integration_test_queue() {
        do_solver_integration_test(Algorithm::Queue);
    }

    #[test]
    fn solver_integration_test_stack() {
        do_solver_integration_test(Algorithm::Stack);
    }
}
