use crate::maze::{Coordinate, Maze};
use std::collections::VecDeque;

struct Solver {
    maze: Maze,
    to_search: VecDeque<Coordinate>,
}

impl Solver {
    pub fn new(maze: Maze) -> Self {
        let mut to_search = VecDeque::new();
        to_search.push_back(maze.starting_position.unwrap());

        Self { maze, to_search }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_sets_up_vecdeque_from_maze() {
        let coordinate = Coordinate {
            room: 0,
            row: 1,
            column: 2,
        };
        let maze = Maze {
            rooms: Vec::new(),
            starting_position: Some(coordinate),
        };
        let mut solver = Solver::new(maze);

        assert_eq!(solver.to_search.pop_front(), Some(coordinate));
    }
}
