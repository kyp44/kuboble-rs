use crate::{Level, Piece, PieceMap, Space, Vector};

pub static LEVELS: [Level; 1] = [Level {
    size: Vector::new(5, 5),
    spaces: &[
        // Row
        Space::Wall,
        Space::Wall,
        Space::Wall,
        Space::Wall,
        Space::Wall,
        // Row
        Space::Wall,
        Space::Free,
        Space::Free,
        Space::Free,
        Space::Wall,
        // Row
        Space::Wall,
        Space::Free,
        Space::Free,
        Space::Free,
        Space::Wall,
        // Row
        Space::Wall,
        Space::Goal(Piece::Orange),
        Space::Goal(Piece::Green),
        Space::Free,
        Space::Wall,
        // Row
        Space::Wall,
        Space::Wall,
        Space::Wall,
        Space::Wall,
        Space::Wall,
    ],
    starting_positions: PieceMap([Vector::new(1, 1), Vector::new(2, 1)]),
    optimal_moves: 5,
}];
