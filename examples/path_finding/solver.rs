use crate::{Format, PathFindingArgs, Algorithm};
use crate::maze::{Coordinate, Maze};
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
    pub fn new(args: PathFindingArgs, maze: Maze) -> Self {
        let mut search_list = SearchList::new(args.algorithm);
        search_list.push(maze.starting_position.unwrap());

        Self { maze, search_list }
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

    #[test]
    fn solver_integration_test_queue() {
        let coordinate = Coordinate {
            room: 0,
            row: 1,
            column: 2,
        };
        let maze = Maze {
            rooms: Vec::new(),
            starting_position: Some(coordinate),
        };
        let args = PathFindingArgs {
            algorithm: Algorithm::Queue,
            output_format: Format::List,
        };
        let mut solver = Solver::new(args, maze);

        assert_eq!(solver.search_list.pop(), Some(coordinate));
    }
}
