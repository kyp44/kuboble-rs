use crate::{Level, Piece, Space, Vector};
use const_for::const_for;

const fn char_to_piece(c: char) -> Piece {
    match c {
        'G' => Piece::Green,
        'O' => Piece::Orange,
        'B' => Piece::Blue,
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
           starting_positions: $positions,
            optimal_moves: $optimal,
        }
    };
}

pub const NUM_LEVELS: usize = 15;
pub const MAX_OPTIMAL_MOVES: usize = 17;

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
        positions: &[Vector::new(1, 1), Vector::new(2, 1)],
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
        positions: &[Vector::new(1, 1), Vector::new(2, 1)],
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
        positions: &[Vector::new(1, 1), Vector::new(2, 1)],
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
        positions: &[Vector::new(1, 1), Vector::new(2, 1)],
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
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
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
        positions: &[Vector::new(4, 3), Vector::new(3, 3)],
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
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
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
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 7,
    },
    // Level 9
    level! {
        spaces: &[
            "_#####",
            "_#   #",
            "## O##",
            "#G   #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
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
        positions: &[Vector::new(4, 3), Vector::new(3, 3)],
        optimal: 8,
    },
    // Level 11
    level! {
        spaces: &[
            "_####_",
            "##  ##",
            "# GO #",
            "##   #",
            "_#####",
        ],
        positions: &[Vector::new(4, 3), Vector::new(3, 3)],
        optimal: 8,
    },
    // Level 12
    level! {
        spaces: &[
            "######",
            "# G  #",
            "##   #",
            "#  O #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1)],
        optimal: 8,
    },
    // Level 13
    level! {
        spaces: &[
            "_#####",
            "_# G #",
            "###  #",
            "# O  #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 8,
    },
    // Level 14
    level! {
        spaces: &[
            "_####_",
            "##  ##",
            "# OG #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 3), Vector::new(3, 3)],
        optimal: 9,
    },
    // Level 15
    level! {
        spaces: &[
            "######",
            "# G  #",
            "## # #",
            "#  O #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1)],
        optimal: 10,
    },
    /* // Level 16
    level! {
        spaces: &[
            "_#####",
            "##O# #",
            "#  G #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 3), Vector::new(3, 3)],
        optimal: 10,
    },
    // Level 17
    level! {
        spaces: &[
            "_#####",
            "## O #",
            "# G ##",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 10,
    },
    // Level 18
    level! {
        spaces: &[
            "######",
            "# #  #",
            "#   G#",
            "#O#  #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 8,
    },
    // Level 19
    level! {
        spaces: &[
            "_#####",
            "##O# #",
            "#    #",
            "# #  #",
            "#  G #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 9,
    },
    // Level 20
    level! {
        spaces: &[
            "__###_",
            "### ##",
            "#  G #",
            "#    #",
            "# O  #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 9,
    },
    // Level 21
    level! {
        spaces: &[
            "_#####",
            "## O #",
            "#   ##",
            "#  G #",
            "# #  #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 9,
    },
    // Level 22
    level! {
        spaces: &[
            "_#####",
            "## # #",
            "#G   #",
            "##O  #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 9,
    },
    // Level 23
    level! {
        spaces: &[
            "_#####",
            "## # #",
            "#    #",
            "#  O #",
            "#G#  #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 9,
    },
    // Level 24
    level! {
        spaces: &[
            "_#####",
            "##O# #",
            "#  G #",
            "#    #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 10,
    },
    // Level 25
    level! {
        spaces: &[
            "_#####",
            "## G #",
            "# #  #",
            "# O  #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 10,
    },
    // Level 26
    level! {
        spaces: &[
            "######",
            "# ## #",
            "#  O #",
            "# G  #",
            "# #  #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 10,
    },
    // Level 27
    level! {
        spaces: &[
            "######",
            "# #  #",
            "#    #",
            "#G# ##",
            "#  O #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 11,
    },
    // Level 28
    level! {
        spaces: &[
            "__####",
            "###  #",
            "#  G #",
            "#O  ##",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 11,
    },
    // Level 29
    level! {
        spaces: &[
            "######",
            "# ## #",
            "# # G#",
            "# O  #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 11,
    },
    // Level 30
    level! {
        spaces: &[
            "######",
            "# #  #",
            "#  O #",
            "#   ##",
            "# G  #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 11,
    },
    // Level 31
    level! {
        spaces: &[
            "######",
            "#  O #",
            "# #  #",
            "#G   #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1)],
        optimal: 11,
    },
    // Level 32
    level! {
        spaces: &[
            "######",
            "#G#  #",
            "#    #",
            "#    #",
            "#O#  #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 11,
    },
    // Level 33
    level! {
        spaces: &[
            "######",
            "# ## #",
            "# G O#",
            "# #  #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 12,
    },
    // Level 34
    level! {
        spaces: &[
            "_#####",
            "##O# #",
            "#    #",
            "#  G #",
            "#   ##",
            "#####_",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4)],
        optimal: 12,
    },
    // Level 35
    level! {
        spaces: &[
            "__####",
            "###  #",
            "#    #",
            "# G  #",
            "#O#  #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 12,
    },
    // Level 36
    level! {
        spaces: &[
            "_#####",
            "## # #",
            "# G  #",
            "# O  #",
            "##   #",
            "_#####",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 13,
    },
    // Level 37
    level! {
        spaces: &[
            "__####",
            "###  #",
            "# G  #",
            "#  O #",
            "#   ##",
            "#####_",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 38
    level! {
        spaces: &[
            "_####_",
            "##  ##",
            "# O  #",
            "# #G #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 39
    level! {
        spaces: &[
            "######",
            "#    #",
            "#B O #",
            "#   G#",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 9,
    }, */
];
