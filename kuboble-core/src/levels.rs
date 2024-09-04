use core::mem::variant_count;

use crate::{Level, Piece, PieceMap, Space, Vector};
use const_for::const_for;

const fn char_to_piece(c: char) -> Piece {
    match c {
        'G' => Piece::Green,
        'O' => Piece::Orange,
        _ => {
            panic!("invalid character for piece or space")
        }
    }
}

const fn char_to_space(c: char) -> Space {
    match c {
        '_' => Space::Void,
        '#' => Space::Wall,
        ' ' => Space::Free,
        _ => Space::Goal(char_to_piece(c)),
    }
}

const fn convert_spaces<const W: usize, const H: usize>(rows: &[&str]) -> [Space; W * H] {
    if rows.len() != H {
        panic!("incorrect number of rows");
    }
    let mut spaces = [Space::Void; W * H];

    const_for!(ri in 0..H => {
        let row = rows[ri].as_bytes();
        if row.len() != W {
            panic!("incorrect number of columns");
        }

        const_for!(ci in 0..W => {
            spaces[ri*W + ci] = char_to_space(row[ci] as char);
        });
    });

    spaces
}

const fn convert_positions(
    positions_tuples: [(u8, u8); variant_count::<Piece>()],
) -> PieceMap<Vector<u8>> {
    let mut positions = [Vector::new(0, 0); variant_count::<Piece>()];

    const_for!(i in 0..positions_tuples.len() => {
        let t = positions_tuples[i];
        positions[i] = Vector::new(t.0, t.1);
    });

    PieceMap(positions)
}

// This makes defining levels much easier and more compact
macro_rules! level {
    {
        spaces: $spaces:expr,
        positions: $positions:expr,
        optimal: $optimal:literal,
     } => {
        Level {
           size: Vector::new($spaces[0].len() as u8, $spaces.len() as u8),
           spaces: &convert_spaces::<{$spaces[0].len()}, {$spaces.len()}>($spaces),
           starting_positions: convert_positions($positions),
            optimal_moves: $optimal,
        }
    }
}

pub const NUM_LEVELS: usize = 10;
pub const MAX_OPTIMAL_MOVES: usize = 10;

pub static LEVELS: [Level; NUM_LEVELS] = [
    // Level 1
    level! {
        spaces: &[
            "#####",
            "#   #",
            "#   #",
            "#OG #",
            "#####",
        ],
        positions: [(1, 1), (2, 1)],
        optimal: 5,
    },
    // Level 2
    level! {
        spaces: &[
            "#####",
            "#  G#",
            "# # #",
            "# O #",
            "#####",
        ],
        positions: [(1, 1), (2, 1)],
        optimal: 6,
    },
    // Level 3
    level! {
        spaces: &[
            "#####",
            "#   #",
            "#GO #",
            "##  #",
            "_####",
        ],
        positions: [(1, 1), (2, 1)],
        optimal: 7,
    },
    // Level 4
    level! {
        spaces: &[
            "#####",
            "#   #",
            "# GO#",
            "##  #",
            "_####",
        ],
        positions: [(1, 1), (2, 1)],
        optimal: 7,
    },
    // Level 5
    level! {
        spaces: &[
            "_#####",
            "##   #",
            "#G   #",
            "##O  #",
            "_#####",
        ],
        positions: [(4, 1), (3, 1)],
        optimal: 5,
    },
    // Level 6
    level! {
        spaces: &[
            "_#####",
            "_#G# #",
            "##O  #",
            "#    #",
            "######",
        ],
        positions: [(3, 3), (4, 3)],
        optimal: 7,
    },
    // Level 7
    level! {
        spaces: &[
            "__####",
            "###  #",
            "#G O #",
            "##   #",
            "_#####",
        ],
        positions: [(3, 1), (4, 1)],
        optimal: 7,
    },
    // Level 8
    level! {
        spaces: &[
                "######",
                "#G#  #",
                "#  O #",
                "#    #",
                "######",
            ],
            positions: [(3, 1), (4, 1)],
            optimal: 7,
    },
    // Level 9
    level! {
        spaces: &[
            "_#####",
            "_#   #",
            "## O##",
            "#G   #",
            "######"
        ],
        positions: [(3, 1), (4, 1)],
        optimal: 7,
    },
    // Level 10
    level! {
        spaces: &[
            "_#####",
            "## #O#",
            "# G  #",
            "##   #",
            "_#####",
        ],
        positions: [(3, 3), (4, 3)],
        optimal: 8,
    },
];
