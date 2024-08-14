#![no_std]

use cell_grid::{Coord, Grid};
use enum_map::{enum_map, Enum, EnumMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum Piece {
    Green,
    Orange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Void,
    Wall,
    Free,
    Goal(Piece),
}

#[derive(Debug)]
pub struct Level {
    spaces: Grid<Space>,
    starting_positions: EnumMap<Piece, Coord>,
}

#[derive(Debug, Clone)]
pub struct BoardState<'a> {
    level: &'a Level,
    positions: EnumMap<Piece, Coord>,
}

pub fn tester() -> Level {
    Level {
        spaces: Grid::from_row_major_iter(
            3,
            [
                Space::Wall,
                Space::Wall,
                Space::Wall,
                Space::Wall,
                Space::Free,
                Space::Wall,
                Space::Wall,
                Space::Wall,
                Space::Wall,
            ],
        ),
        starting_positions: enum_map! {
            Piece::Green => Coord::new(0, 0),
            Piece::Orange => Coord::new(0, 0),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make some tests
    #[test]
    fn it_works() {
        assert!(false);
    }
}
