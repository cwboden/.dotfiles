use crate::maze::{Cell, Coordinate, Direction, Maze};
use crate::Algorithm;
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

pub struct Solver {
    maze: Maze,
    search_list: SearchList<Vec<Coordinate>>,
}

impl Solver {
    pub fn new(algorithm: Algorithm, maze: Maze) -> Self {
        let mut search_list = SearchList::new(algorithm);
        search_list.push(vec![maze.starting_position.unwrap()]);

        Self { maze, search_list }
    }

    pub fn run(mut self) -> Vec<Coordinate> {
        while let Some(path) = self.search_list.pop() {
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .iter()
            {
                // XXX: Have to clone due to non-lexical lifetimes?
                // https://stackoverflow.com/questions/38023871/returning-a-reference-from-a-hashmap-or-vec-causes-a-borrow-to-last-beyond-the-s
                let mut path = path.clone();
                let origin = path.last().unwrap();
                let destination = origin.move_in(direction);
                if path.contains(&destination) {
                    continue;
                }

                if let Some(cell) = self.maze.at(destination) {
                    match cell {
                        // Apply special movement rules:
                        // - Teleport to the corresponding floor for a Teleporter
                        // - Return the path if the exit is found
                        Cell::Empty => {
                            path.push(destination);
                            self.search_list.push(path);
                        }
                        Cell::StartPosition => panic!(
                            "Unexpected Cell '{:?}' should have been filtered out.",
                            cell
                        ),
                        Cell::Wall => (/* do nothing */),
                        Cell::Teleporter(floor) => {
                            path.push(Coordinate {
                                room: floor,
                                ..destination
                            });
                            self.search_list.push(path);
                        }
                        Cell::Exit => {
                            path.push(destination);
                            return path;
                        }
                    }
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
}
