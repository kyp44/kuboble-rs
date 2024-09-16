use crate::{Level, Space, Vector};
use const_for::const_for;

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
            spaces[ri*W + ci] = match Space::from_char(row[ci] as char) {
                Some(s) => s,
                None => panic!("invalid character for piece or space"),
            };
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

pub const NUM_LEVELS: usize = 287;
pub const MAX_OPTIMAL_MOVES: usize = 54;
pub const MAX_LEVEL_SIZE: usize = 8;
pub const MAX_STRIP_SIZE: usize = MAX_LEVEL_SIZE - 2;

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
    // Level 16
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
    },
    // Level 40
    level! {
        spaces: &[
            "######",
            "#  G #",
            "#   O#",
            "# B  #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 9,
    },
    // Level 41
    level! {
        spaces: &[
            "######",
            "#    #",
            "# G  #",
            "# B O#",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 9,
    },
    // Level 42
    level! {
        spaces: &[
            "######",
            "#   O#",
            "#  G #",
            "# B  #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 9,
    },
    // Level 43
    level! {
        spaces: &[
            "######",
            "#    #",
            "# BO #",
            "#G   #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 9,
    },
    // Level 44
    level! {
        spaces: &[
            "######",
            "#  G #",
            "#B#  #",
            "# O  #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 10,
    },
    // Level 45
    level! {
        spaces: &[
            "######",
            "#O#  #",
            "#  B #",
            "#  G #",
            "######",
        ],
        positions: &[Vector::new(4, 3), Vector::new(3, 3), Vector::new(2, 3)],
        optimal: 10,
    },
    // Level 46
    level! {
        spaces: &[
            "######",
            "# # B#",
            "# O  #",
            "# G  #",
            "######",
        ],
        positions: &[Vector::new(4, 3), Vector::new(3, 3), Vector::new(2, 3)],
        optimal: 10,
    },
    // Level 47
    level! {
        spaces: &[
            "######",
            "#  O #",
            "# # G#",
            "# B  #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 10,
    },
    // Level 48
    level! {
        spaces: &[
            "_#####",
            "## #O#",
            "# G  #",
            "#  B##",
            "#####_",
        ],
        positions: &[Vector::new(1, 3), Vector::new(2, 3), Vector::new(3, 3)],
        optimal: 10,
    },
    // Level 49
    level! {
        spaces: &[
            "######",
            "# O G#",
            "# #  #",
            "# B  #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 10,
    },
    // Level 50
    level! {
        spaces: &[
            "_#####",
            "##   #",
            "#O B #",
            "#G#  #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1), Vector::new(2, 1)],
        optimal: 11,
    },
    // Level 51
    level! {
        spaces: &[
            "_#####",
            "## B #",
            "# O  #",
            "#  G #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1), Vector::new(2, 1)],
        optimal: 11,
    },
    // Level 52
    level! {
        spaces: &[
            "######",
            "# # B#",
            "#  O #",
            "#  #G#",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(1, 2), Vector::new(1, 3)],
        optimal: 11,
    },
    // Level 53
    level! {
        spaces: &[
            "######",
            "# #  #",
            "# G  #",
            "# O#B#",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(1, 2), Vector::new(1, 3)],
        optimal: 11,
    },
    // Level 54
    level! {
        spaces: &[
            "_#####",
            "## G #",
            "# B# #",
            "#  O #",
            "######",
        ],
        positions: &[Vector::new(4, 1), Vector::new(3, 1), Vector::new(2, 1)],
        optimal: 13,
    },
    // Level 55
    level! {
        spaces: &[
            "######",
            "# #  #",
            "#G#O #",
            "#  B #",
            "######",
        ],
        positions: &[Vector::new(4, 3), Vector::new(3, 3), Vector::new(2, 3)],
        optimal: 14,
    },
    // Level 56
    level! {
        spaces: &[
            "######",
            "# #  #",
            "#G #B#",
            "#  O #",
            "######",
        ],
        positions: &[Vector::new(4, 3), Vector::new(3, 3), Vector::new(2, 3)],
        optimal: 15,
    },
    // Level 57
    level! {
        spaces: &[
            "######",
            "#O## #",
            "#    #",
            "#  G #",
            "#  B #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4), Vector::new(2, 4)],
        optimal: 11,
    },
    // Level 58
    level! {
        spaces: &[
            "######",
            "# ## #",
            "# O G#",
            "#  B #",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4), Vector::new(2, 4)],
        optimal: 11,
    },
    // Level 59
    level! {
        spaces: &[
            "_#####",
            "##G# #",
            "#    #",
            "# O  #",
            "#  B #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4), Vector::new(2, 4)],
        optimal: 12,
    },
    // Level 60
    level! {
        spaces: &[
            "######",
            "# # B#",
            "#    #",
            "# G O#",
            "# #  #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(1, 2), Vector::new(1, 3)],
        optimal: 13,
    },
    // Level 61
    level! {
        spaces: &[
            "######",
            "#    #",
            "# B  #",
            "#G   #",
            "#  O #",
            "######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 62
    level! {
        spaces: &[
            "_####_",
            "##  ##",
            "#  O #",
            "# G B#",
            "#    #",
            "######",
        ],
        positions: &[Vector::new(4, 4), Vector::new(3, 4), Vector::new(2, 4)],
        optimal: 14,
    },
    // Level 63
    level! {
        spaces: &[
            "__#####",
            "###   #",
            "# #  O#",
            "# BG  #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 12,
    },
    // Level 64
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#  G  #",
            "# B  O#",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 65
    level! {
        spaces: &[
            "_######",
            "##    #",
            "#  G  #",
            "# #  O#",
            "# B   #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 66
    level! {
        spaces: &[
            "__#####",
            "###   #",
            "#   G #",
            "# O B #",
            "#  #  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 67
    level! {
        spaces: &[
            "#######",
            "#G#   #",
            "#     #",
            "# B   #",
            "#   O #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 68
    level! {
        spaces: &[
            "#######",
            "# #B# #",
            "#   O #",
            "# G   #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 13,
    },
    // Level 69
    level! {
        spaces: &[
            "_#####_",
            "##   ##",
            "# G O #",
            "### B #",
            "_#    #",
            "_######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 13,
    },
    // Level 70
    level! {
        spaces: &[
            "#######",
            "#B#G# #",
            "#     #",
            "#  O  #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 13,
    },
    // Level 71
    level! {
        spaces: &[
            "#######",
            "#O##  #",
            "#     #",
            "## BG #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 14,
    },
    // Level 72
    level! {
        spaces: &[
            "#######",
            "# #B# #",
            "#     #",
            "#  #G #",
            "#   O #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 14,
    },
    // Level 73
    level! {
        spaces: &[
            "_######",
            "##    #",
            "#  # O#",
            "# # B #",
            "# G  ##",
            "######_",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 15,
    },
    // Level 74
    level! {
        spaces: &[
            "_######",
            "##    #",
            "#O #  #",
            "##  B #",
            "#  G  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 15,
    },
    // Level 75
    level! {
        spaces: &[
            "#######",
            "# #G# #",
            "# B   #",
            "#O#   #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 16,
    },
    // Level 76
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#B G  #",
            "# #  ##",
            "# O   #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 77
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#   #O#",
            "#  #  #",
            "# B G #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 78
    level! {
        spaces: &[
            "#######",
            "#O#   #",
            "#   B #",
            "##  G #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 79
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#  B  #",
            "# G## #",
            "#  O  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 80
    level! {
        spaces: &[
            "#######",
            "# B   #",
            "# #O# #",
            "#G    #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 81
    level! {
        spaces: &[
            "__#####",
            "###   #",
            "#   B #",
            "#   G #",
            "#O#   #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 82
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "# O  ##",
            "#     #",
            "# B #G#",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 83
    level! {
        spaces: &[
            "_######",
            "##  # #",
            "#  G  #",
            "# O  B#",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 16,
    },
    // Level 84
    level! {
        spaces: &[
            "_#####_",
            "##   ##",
            "#  O  #",
            "#   B #",
            "## G ##",
            "_#####_",
        ],
        positions: &[Vector::new(1, 2), Vector::new(2, 1), Vector::new(5, 2)],
        optimal: 16,
    },
    // Level 85
    level! {
        spaces: &[
            "_######",
            "##    #",
            "# G   #",
            "##  O #",
            "#  B  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 86
    level! {
        spaces: &[
            "__#####",
            "###   #",
            "#  G  #",
            "# B   #",
            "#   O #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 87
    level! {
        spaces: &[
            "#######",
            "# O   #",
            "##  B #",
            "# #  G#",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 88
    level! {
        spaces: &[
            "__####_",
            "### O##",
            "#     #",
            "# GB  #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 16,
    },
    // Level 89
    level! {
        spaces: &[
            "_######",
            "##    #",
            "# GB  #",
            "#   O #",
            "#  #  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 90
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "# ##  #",
            "#  G O#",
            "#  B  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 91
    level! {
        spaces: &[
            "_######",
            "## #  #",
            "# B   #",
            "#  G  #",
            "## O  #",
            "_######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 16,
    },
    // Level 92
    level! {
        spaces: &[
            "__#####",
            "### O #",
            "#  B  #",
            "##  G #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 93
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#  O  #",
            "#G#  ##",
            "#  B  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 94
    level! {
        spaces: &[
            "#######",
            "# ##  #",
            "#  B O#",
            "#  #  #",
            "#  G  #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 16,
    },
    // Level 95
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "# O  ##",
            "# B   #",
            "# #  G#",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 16,
    },
    // Level 96
    level! {
        spaces: &[
            "_######",
            "##  # #",
            "# O   #",
            "#  G ##",
            "#   B #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 16,
    },
    // Level 97
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#   B #",
            "# G  ##",
            "#O    #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 17,
    },
    // Level 98
    level! {
        spaces: &[
            "_#####_",
            "##   ##",
            "# G   #",
            "#B#O  #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 99
    level! {
        spaces: &[
            "#######",
            "# O B #",
            "###   #",
            "#  G  #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 17,
    },
    // Level 100
    level! {
        spaces: &[
            "_######",
            "##O#  #",
            "#   B #",
            "# G   #",
            "#  #  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(5, 2), Vector::new(5, 3)],
        optimal: 17,
    },
    // Level 101
    level! {
        spaces: &[
            "#######",
            "# G   #",
            "# ##  #",
            "#   B #",
            "#  O  #",
            "#######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 17,
    },
    // Level 102
    level! {
        spaces: &[
            "_#####_",
            "##   ##",
            "# O B #",
            "#     #",
            "#   G #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 103
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#O#  B#",
            "#   G #",
            "#  #  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 17,
    },
    // Level 104
    level! {
        spaces: &[
            "#######",
            "#B# # #",
            "#  G  #",
            "##  O #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 105
    level! {
        spaces: &[
            "_######",
            "## #  #",
            "#  B  #",
            "#  G  #",
            "#O #  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(5, 2), Vector::new(5, 3)],
        optimal: 17,
    },
    // Level 106
    level! {
        spaces: &[
            "#######",
            "#  #  #",
            "# O   #",
            "## B G#",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 107
    level! {
        spaces: &[
            "#######",
            "#O #  #",
            "#  G  #",
            "#  #  #",
            "#  B  #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 108
    level! {
        spaces: &[
            "_######",
            "##    #",
            "#B#G#O#",
            "#     #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 17,
    },
    // Level 109
    level! {
        spaces: &[
            "#######",
            "#  O  #",
            "# ##  #",
            "#B  G #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 17,
    },
    // Level 110
    level! {
        spaces: &[
            "__#####",
            "###   #",
            "#  O  #",
            "# G B #",
            "#   # #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 18,
    },
    // Level 111
    level! {
        spaces: &[
            "#######",
            "#     #",
            "##  B #",
            "# G   #",
            "#  O  #",
            "#######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 18,
    },
    // Level 112
    level! {
        spaces: &[
            "_######",
            "## #  #",
            "# B G #",
            "##    #",
            "#   O #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 18,
    },
    // Level 113
    level! {
        spaces: &[
            "_######",
            "## O  #",
            "#  #B##",
            "#     #",
            "#   G #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 18,
    },
    // Level 114
    level! {
        spaces: &[
            "___####",
            "####  #",
            "# B G #",
            "#  O  #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 18,
    },
    // Level 115
    level! {
        spaces: &[
            "_######",
            "##    #",
            "#B G  #",
            "#  O  #",
            "##    #",
            "_######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 18,
    },
    // Level 116
    level! {
        spaces: &[
            "_######",
            "## #  #",
            "#   G #",
            "# BO ##",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 18,
    },
    // Level 117
    level! {
        spaces: &[
            "#######",
            "#  G  #",
            "## #B##",
            "##  O #",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 19,
    },
    // Level 118
    level! {
        spaces: &[
            "_######",
            "##  # #",
            "# B   #",
            "#  #O##",
            "# G   #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 19,
    },
    // Level 119
    level! {
        spaces: &[
            "_######",
            "##  G #",
            "#  B# #",
            "#   O #",
            "##    #",
            "_######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 19,
    },
    // Level 120
    level! {
        spaces: &[
            "_######",
            "##  G #",
            "#   # #",
            "#  B  #",
            "##  O #",
            "_######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 19,
    },
    // Level 121
    level! {
        spaces: &[
            "_######",
            "## G  #",
            "# ##  #",
            "#   O #",
            "#  B  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 19,
    },
    // Level 122
    level! {
        spaces: &[
            "_######",
            "##  # #",
            "#  B  #",
            "# G#O##",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 19,
    },
    // Level 123
    level! {
        spaces: &[
            "_######",
            "##  # #",
            "# O   #",
            "## G  #",
            "#   B #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 19,
    },
    // Level 124
    level! {
        spaces: &[
            "#######",
            "#O#   #",
            "# G #B#",
            "# #   #",
            "#   # #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 19,
    },
    // Level 125
    level! {
        spaces: &[
            "_######",
            "## #  #",
            "#   G #",
            "#  O ##",
            "# B   #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 19,
    },
    // Level 126
    level! {
        spaces: &[
            "_######",
            "##    #",
            "#   B #",
            "# G  O#",
            "##    #",
            "_######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 20,
    },
    // Level 127
    level! {
        spaces: &[
            "_######",
            "##  # #",
            "#  O  #",
            "# G# B#",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 20,
    },
    // Level 128
    level! {
        spaces: &[
            "_######",
            "##    #",
            "# G#  #",
            "#  O  #",
            "## B  #",
            "_######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 20,
    },
    // Level 129
    level! {
        spaces: &[
            "_######",
            "##  # #",
            "#  O  #",
            "#B#   #",
            "#  G  #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 20,
    },
    // Level 130
    level! {
        spaces: &[
            "_######",
            "##  # #",
            "#  G  #",
            "#O#  ##",
            "#  B  #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 20,
    },
    // Level 131
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#  G B#",
            "##  # #",
            "#   O #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 20,
    },
    // Level 132
    level! {
        spaces: &[
            "#######",
            "# #   #",
            "#   # #",
            "# B #O#",
            "#  G  #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 20,
    },
    // Level 133
    level! {
        spaces: &[
            "_#####_",
            "##   ##",
            "#   O #",
            "# B#  #",
            "#   G #",
            "#######",
        ],
        positions: &[Vector::new(5, 4), Vector::new(4, 4), Vector::new(3, 4)],
        optimal: 21,
    },
    // Level 134
    level! {
        spaces: &[
            "#######",
            "# #B  #",
            "#  #  #",
            "# G #O#",
            "#     #",
            "#######",
        ],
        positions: &[Vector::new(5, 1), Vector::new(4, 1), Vector::new(3, 1)],
        optimal: 23,
    },
    // Level 135
    level! {
        spaces: &[
            "###_####",
            "# ###  #",
            "#  GO  #",
            "#  #   #",
            "#  B   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 11,
    },
    // Level 136
    level! {
        spaces: &[
            "########",
            "# #    #",
            "# G  # #",
            "# OB   #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 11,
    },
    // Level 137
    level! {
        spaces: &[
            "__######",
            "###    #",
            "# BO   #",
            "##G ## #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 11,
    },
    // Level 138
    level! {
        spaces: &[
            "########",
            "#G#    #",
            "#     ##",
            "##O# # #",
            "# B    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 12,
    },
    // Level 139
    level! {
        spaces: &[
            "_######_",
            "_#    ##",
            "##B  # #",
            "#  OG  #",
            "# #    #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 12,
    },
    // Level 140
    level! {
        spaces: &[
            "_#######",
            "##   # #",
            "# GBO  #",
            "##    ##",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 12,
    },
    // Level 141
    level! {
        spaces: &[
            "_#######",
            "##   # #",
            "#   GB #",
            "# #  O #",
            "#   ## #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 12,
    },
    // Level 142
    level! {
        spaces: &[
            "__######",
            "###    #",
            "#  #B  #",
            "#   GO #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 12,
    },
    // Level 143
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#G#O   #",
            "# B #  #",
            "# #    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 13,
    },
    // Level 144
    level! {
        spaces: &[
            "_#######",
            "##   # #",
            "# # G  #",
            "# BO   #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 13,
    },
    // Level 145
    level! {
        spaces: &[
            "_#######",
            "_#   # #",
            "## O   #",
            "#  GB  #",
            "#     ##",
            "#######_",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 13,
    },
    // Level 146
    level! {
        spaces: &[
            "########",
            "#      #",
            "## OB  #",
            "#  #G# #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 147
    level! {
        spaces: &[
            "_#######",
            "##   # #",
            "# #G#  #",
            "#  BO  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 13,
    },
    // Level 148
    level! {
        spaces: &[
            "_#######",
            "##G# # #",
            "#      #",
            "# # O  #",
            "#  B   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 13,
    },
    // Level 149
    level! {
        spaces: &[
            "__######",
            "###    #",
            "# GB # #",
            "# O    #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 13,
    },
    // Level 150
    level! {
        spaces: &[
            "__######",
            "###O#  #",
            "#      #",
            "#  GB  #",
            "#  #  ##",
            "#######_",
        ],
        positions: &[Vector::new(6, 1), Vector::new(6, 2), Vector::new(6, 3)],
        optimal: 13,
    },
    // Level 151
    level! {
        spaces: &[
            "########",
            "#      #",
            "### OB #",
            "#   #G #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 152
    level! {
        spaces: &[
            "_#######",
            "## #   #",
            "# G    #",
            "#  B   #",
            "# O # ##",
            "#######_",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 13,
    },
    // Level 153
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "# #O   #",
            "#G B#  #",
            "#     ##",
            "#######_",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 13,
    },
    // Level 154
    level! {
        spaces: &[
            "__######",
            "###    #",
            "#      #",
            "# G# # #",
            "# BO # #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 13,
    },
    // Level 155
    level! {
        spaces: &[
            "__######",
            "###G#  #",
            "#      #",
            "#  O   #",
            "#  #B# #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(6, 2), Vector::new(6, 3)],
        optimal: 13,
    },
    // Level 156
    level! {
        spaces: &[
            "_#######",
            "_#   # #",
            "## O   #",
            "# GB   #",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 13,
    },
    // Level 157
    level! {
        spaces: &[
            "########",
            "#      #",
            "## G#  #",
            "#  OB  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 158
    level! {
        spaces: &[
            "_#######",
            "## ##  #",
            "#    G #",
            "# OB   #",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 13,
    },
    // Level 159
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "## BO  #",
            "#   G ##",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 13,
    },
    // Level 160
    level! {
        spaces: &[
            "########",
            "#      #",
            "##  OB #",
            "#   #G #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 13,
    },
    // Level 161
    level! {
        spaces: &[
            "_#######",
            "_# ##  #",
            "## G   #",
            "#  OB  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 13,
    },
    // Level 162
    level! {
        spaces: &[
            "########",
            "#  #   #",
            "#  G # #",
            "#O#    #",
            "# B    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 13,
    },
    // Level 163
    level! {
        spaces: &[
            "_######_",
            "##    ##",
            "# #O#  #",
            "#  GB  #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 14,
    },
    // Level 164
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "# ##   #",
            "#O# G# #",
            "# B    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 165
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "# GB   #",
            "# O# # #",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 166
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#  OG  #",
            "#  B   #",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 167
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "# GB  ##",
            "#      #",
            "# #O#  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 168
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "## GO ##",
            "#   #B##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 169
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "# BG   #",
            "# #  # #",
            "#  O # #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 170
    level! {
        spaces: &[
            "########",
            "#      #",
            "# # G# #",
            "#  OB  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 14,
    },
    // Level 171
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#  G # #",
            "# #  B##",
            "# O    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 172
    level! {
        spaces: &[
            "__######",
            "_##    #",
            "##  G  #",
            "# B    #",
            "#O# #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 173
    level! {
        spaces: &[
            "_######_",
            "##    ##",
            "# GO   #",
            "## B # #",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 14,
    },
    // Level 174
    level! {
        spaces: &[
            "########",
            "# #B#  #",
            "#   O  #",
            "#   G ##",
            "#  ##  #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(1, 2), Vector::new(1, 3)],
        optimal: 14,
    },
    // Level 175
    level! {
        spaces: &[
            "########",
            "# # # O#",
            "#  G  ##",
            "#   B ##",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 14,
    },
    // Level 176
    level! {
        spaces: &[
            "_#######",
            "## B # #",
            "#  #O# #",
            "#   G  #",
            "##     #",
            "_#######",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 14,
    },
    // Level 177
    level! {
        spaces: &[
            "_#######",
            "## ##  #",
            "#G     #",
            "## BO  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 14,
    },
    // Level 178
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#  GB  #",
            "#   O  #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 179
    level! {
        spaces: &[
            "########",
            "# #  # #",
            "# # G  #",
            "# ## B #",
            "#   O  #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 14,
    },
    // Level 180
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#   GB #",
            "#   O  #",
            "# #    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 181
    level! {
        spaces: &[
            "########",
            "# #    #",
            "#   G ##",
            "#   OB #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 182
    level! {
        spaces: &[
            "########",
            "#      #",
            "# #G#  #",
            "#  OB  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 14,
    },
    // Level 183
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "# ##  ##",
            "# GOB  #",
            "#     ##",
            "#######_",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 184
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "# BG   #",
            "###O  ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 14,
    },
    // Level 185
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#  G # #",
            "#  BO  #",
            "##     #",
            "_#######",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 15,
    },
    // Level 186
    level! {
        spaces: &[
            "_#######",
            "##   # #",
            "#  G # #",
            "#B# O  #",
            "##     #",
            "_#######",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 15,
    },
    // Level 187
    level! {
        spaces: &[
            "########",
            "# #    #",
            "#   G ##",
            "# O B  #",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 15,
    },
    // Level 188
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "# O    #",
            "# #  G #",
            "#  B   #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 15,
    },
    // Level 189
    level! {
        spaces: &[
            "########",
            "# #    #",
            "# B   ##",
            "# GO   #",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 15,
    },
    // Level 190
    level! {
        spaces: &[
            "_#######",
            "## #O# #",
            "#      #",
            "# B #  #",
            "#  G   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 15,
    },
    // Level 191
    level! {
        spaces: &[
            "__######",
            "###  # #",
            "#  B   #",
            "# GO  ##",
            "#     #_",
            "#######_",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 15,
    },
    // Level 192
    level! {
        spaces: &[
            "########",
            "# #    #",
            "#O   # #",
            "# G   ##",
            "#    B #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 15,
    },
    // Level 193
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "#  O   #",
            "#  GB  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 15,
    },
    // Level 194
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "# GB   #",
            "#  # O #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 15,
    },
    // Level 195
    level! {
        spaces: &[
            "########",
            "#  ##  #",
            "# B#   #",
            "#  OG  #",
            "#  ##  #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(1, 2), Vector::new(1, 3)],
        optimal: 15,
    },
    // Level 196
    level! {
        spaces: &[
            "__######",
            "### #  #",
            "#     ##",
            "# G OB #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 16,
    },
    // Level 197
    level! {
        spaces: &[
            "_######_",
            "##    ##",
            "#   B  #",
            "#  GO  #",
            "##     #",
            "_#######",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 198
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "#B#   ##",
            "#   OG #",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 199
    level! {
        spaces: &[
            "__#####_",
            "###   ##",
            "#      #",
            "# G O ##",
            "#B#    #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 200
    level! {
        spaces: &[
            "__######",
            "### #  #",
            "# B   ##",
            "#   GO #",
            "# #    #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 201
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "###GB# #",
            "#  O   #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 202
    level! {
        spaces: &[
            "__#####_",
            "###  G##",
            "#      #",
            "# BO ###",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 203
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "# B    #",
            "#  GO  #",
            "# #    #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 204
    level! {
        spaces: &[
            "########",
            "# #    #",
            "#   GB #",
            "##  O  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 205
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "#G ##  #",
            "#  # O #",
            "#  B   #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 206
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "## GO ##",
            "# B    #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 207
    level! {
        spaces: &[
            "__######",
            "###B#  #",
            "#   G  #",
            "##O#   #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 208
    level! {
        spaces: &[
            "__######",
            "###G#  #",
            "#   O  #",
            "#  ##  #",
            "#  B   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 209
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "# ## G #",
            "#   OB #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 210
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "# # GB #",
            "#  O   #",
            "# #    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 211
    level! {
        spaces: &[
            "_#######",
            "##   # #",
            "#  G   #",
            "#  BO  #",
            "# #  # #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(6, 2), Vector::new(6, 3)],
        optimal: 16,
    },
    // Level 212
    level! {
        spaces: &[
            "_######_",
            "## #  ##",
            "#      #",
            "# GB   #",
            "#   O  #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 213
    level! {
        spaces: &[
            "########",
            "#G##   #",
            "#OB#   #",
            "##   # #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 214
    level! {
        spaces: &[
            "_#######",
            "##  O# #",
            "#      #",
            "# BG   #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 215
    level! {
        spaces: &[
            "########",
            "# ##   #",
            "#B# G  #",
            "#    # #",
            "#O#    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 216
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#   #  #",
            "#B OG ##",
            "##     #",
            "_#######",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 217
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "_## O ##",
            "##  GB #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 218
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#O #   #",
            "##  GB #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 219
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "###B  ##",
            "# G  O #",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 220
    level! {
        spaces: &[
            "_#######",
            "##   # #",
            "#  GB  #",
            "#  O   #",
            "##     #",
            "_#######",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 221
    level! {
        spaces: &[
            "__######",
            "###    #",
            "#   GB #",
            "#   O  #",
            "# #    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 222
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "#    B #",
            "# GO   #",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 16,
    },
    // Level 223
    level! {
        spaces: &[
            "_######_",
            "##G   ##",
            "#      #",
            "#  OB  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 16,
    },
    // Level 224
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "# # O ##",
            "#   GB #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 16,
    },
    // Level 225
    level! {
        spaces: &[
            "########",
            "# #  # #",
            "#   OG #",
            "##   #B#",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 226
    level! {
        spaces: &[
            "########",
            "#O#    #",
            "# G  B #",
            "##  #  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 17,
    },
    // Level 227
    level! {
        spaces: &[
            "########",
            "# #  # #",
            "#   B  #",
            "##O# G #",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 228
    level! {
        spaces: &[
            "########",
            "# #    #",
            "#   ## #",
            "# O GB #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 17,
    },
    // Level 229
    level! {
        spaces: &[
            "########",
            "#  ##  #",
            "#B## O #",
            "# G    #",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 17,
    },
    // Level 230
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "# O  # #",
            "#  BG  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 17,
    },
    // Level 231
    level! {
        spaces: &[
            "########",
            "#G#    #",
            "#B  #  #",
            "##   O #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 17,
    },
    // Level 232
    level! {
        spaces: &[
            "########",
            "#O #   #",
            "##   B #",
            "# #  G #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 17,
    },
    // Level 233
    level! {
        spaces: &[
            "_#######",
            "##O#   #",
            "#   G ##",
            "#  B # #",
            "# #    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 17,
    },
    // Level 234
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "### B  #",
            "#  G   #",
            "# O #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 17,
    },
    // Level 235
    level! {
        spaces: &[
            "########",
            "# #B#  #",
            "# O #  #",
            "# # G  #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 17,
    },
    // Level 236
    level! {
        spaces: &[
            "########",
            "#   G  #",
            "##O ## #",
            "#  ##  #",
            "#   B  #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 17,
    },
    // Level 237
    level! {
        spaces: &[
            "########",
            "#  ##  #",
            "# G B  #",
            "##   #O#",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 17,
    },
    // Level 238
    level! {
        spaces: &[
            "########",
            "#  #   #",
            "# ## G #",
            "#O   B #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 17,
    },
    // Level 239
    level! {
        spaces: &[
            "########",
            "# #G#  #",
            "#    B #",
            "# O   ##",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 18,
    },
    // Level 240
    level! {
        spaces: &[
            "###_####",
            "# ###  #",
            "#   G  #",
            "# O #  #",
            "#    B #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 18,
    },
    // Level 241
    level! {
        spaces: &[
            "########",
            "# #  # #",
            "# # OG #",
            "#  B # #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 18,
    },
    // Level 242
    level! {
        spaces: &[
            "########",
            "# # #  #",
            "#    G #",
            "# O  #B#",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 18,
    },
    // Level 243
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "_# GO ##",
            "##   B #",
            "#     ##",
            "#######_",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 18,
    },
    // Level 244
    level! {
        spaces: &[
            "########",
            "# # #  #",
            "#    G #",
            "# O#B  #",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 18,
    },
    // Level 245
    level! {
        spaces: &[
            "__######",
            "### O  #",
            "#   B  #",
            "#   #  #",
            "#G#    #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 19,
    },
    // Level 246
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "_#   G##",
            "## BO ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 19,
    },
    // Level 247
    level! {
        spaces: &[
            "_######_",
            "## G  ##",
            "# ##   #",
            "#   O ##",
            "#  B   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 19,
    },
    // Level 248
    level! {
        spaces: &[
            "########",
            "# # #  #",
            "#    G #",
            "#  BO ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 19,
    },
    // Level 249
    level! {
        spaces: &[
            "_######_",
            "_#    ##",
            "_#    O#",
            "## BG ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 19,
    },
    // Level 250
    level! {
        spaces: &[
            "_######_",
            "_#  G ##",
            "##O##  #",
            "# B    #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 19,
    },
    // Level 251
    level! {
        spaces: &[
            "_######_",
            "_#    ##",
            "##  GB #",
            "#   O  #",
            "##    ##",
            "_######_",
        ],
        positions: &[Vector::new(2, 1), Vector::new(6, 2), Vector::new(5, 1)],
        optimal: 19,
    },
    // Level 252
    level! {
        spaces: &[
            "########",
            "# #    #",
            "#   GO #",
            "# B   ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 19,
    },
    // Level 253
    level! {
        spaces: &[
            "_#######",
            "_#     #",
            "##GB#  #",
            "#   O  #",
            "# #  # #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 19,
    },
    // Level 254
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "#   G  #",
            "#  BO  #",
            "##     #",
            "_#######",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 19,
    },
    // Level 255
    level! {
        spaces: &[
            "__######",
            "###  # #",
            "#      #",
            "# O  G #",
            "# # B  #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 20,
    },
    // Level 256
    level! {
        spaces: &[
            "########",
            "# #  # #",
            "# O G  #",
            "# #B # #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 20,
    },
    // Level 257
    level! {
        spaces: &[
            "_#######",
            "##   # #",
            "# GO   #",
            "#    B #",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 21,
    },
    // Level 258
    level! {
        spaces: &[
            "_#######",
            "##  #G #",
            "# B  #O#",
            "# #    #",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 21,
    },
    // Level 259
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "#      #",
            "##B#G# #",
            "# O    #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 22,
    },
    // Level 260
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "# O    #",
            "# #G #B#",
            "#    # #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 22,
    },
    // Level 261
    level! {
        spaces: &[
            "########",
            "#  # B #",
            "# G  ###",
            "##     #",
            "#  O   #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 22,
    },
    // Level 262
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "# G    #",
            "###BO ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 23,
    },
    // Level 263
    level! {
        spaces: &[
            "_#######",
            "## #   #",
            "#   O  #",
            "# BG  ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 23,
    },
    // Level 264
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "# O  G #",
            "# B#   #",
            "#   ## #",
            "########",
        ],
        positions: &[Vector::new(1, 4), Vector::new(2, 4), Vector::new(3, 4)],
        optimal: 23,
    },
    // Level 265
    level! {
        spaces: &[
            "_#######",
            "_#  #  #",
            "## G   #",
            "# #O#  #",
            "#   B  #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 23,
    },
    // Level 266
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "#    O #",
            "##G#B ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 23,
    },
    // Level 267
    level! {
        spaces: &[
            "_#######",
            "_#   # #",
            "##O# G #",
            "# B  # #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 23,
    },
    // Level 268
    level! {
        spaces: &[
            "########",
            "#  G   #",
            "## #   #",
            "#   #B##",
            "#  O   #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 24,
    },
    // Level 269
    level! {
        spaces: &[
            "########",
            "#      #",
            "##  GO##",
            "##   B##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 24,
    },
    // Level 270
    level! {
        spaces: &[
            "########",
            "# #    #",
            "#G  # ##",
            "#   BO #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 24,
    },
    // Level 271
    level! {
        spaces: &[
            "########",
            "#      #",
            "##G#  ##",
            "##O#   #",
            "#  B   #",
            "########",
        ],
        positions: &[Vector::new(1, 1), Vector::new(2, 1), Vector::new(3, 1)],
        optimal: 24,
    },
    // Level 272
    level! {
        spaces: &[
            "_#######",
            "##     #",
            "# # #G #",
            "#  OB  #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 25,
    },
    // Level 273
    level! {
        spaces: &[
            "########",
            "# # #  #",
            "# # GO #",
            "#   # ##",
            "#  B   #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 25,
    },
    // Level 274
    level! {
        spaces: &[
            "__#####_",
            "###   ##",
            "#   GO #",
            "# B # ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 26,
    },
    // Level 275
    level! {
        spaces: &[
            "_#######",
            "## ##  #",
            "#   OG #",
            "# B # ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 26,
    },
    // Level 276
    level! {
        spaces: &[
            "########",
            "#  #   #",
            "# G   ##",
            "##O# B##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 27,
    },
    // Level 277
    level! {
        spaces: &[
            "_#######",
            "## B   #",
            "#  # ###",
            "#  GO  #",
            "#  #   #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 28,
    },
    // Level 278
    level! {
        spaces: &[
            "########",
            "#  #   #",
            "# OG   #",
            "## #B# #",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 28,
    },
    // Level 279
    level! {
        spaces: &[
            "########",
            "# #  # #",
            "#   B  #",
            "# ##   #",
            "# G O  #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 28,
    },
    // Level 280
    level! {
        spaces: &[
            "_#######",
            "## #   #",
            "#  B  ##",
            "# G# O #",
            "##     #",
            "_#######",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 28,
    },
    // Level 281
    level! {
        spaces: &[
            "_#######",
            "##  G  #",
            "# # # ##",
            "#  OB  #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 29,
    },
    // Level 282
    level! {
        spaces: &[
            "########",
            "# # B  #",
            "#   # ##",
            "##G#   #",
            "#   O  #",
            "########",
        ],
        positions: &[Vector::new(6, 1), Vector::new(5, 1), Vector::new(4, 1)],
        optimal: 30,
    },
    // Level 283
    level! {
        spaces: &[
            "_######_",
            "##O#  ##",
            "#   G  #",
            "##B#   #",
            "_#   # #",
            "_#######",
        ],
        positions: &[Vector::new(6, 4), Vector::new(6, 3), Vector::new(6, 2)],
        optimal: 31,
    },
    // Level 284
    level! {
        spaces: &[
            "__######",
            "###O#  #",
            "#   G  #",
            "# B # ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 31,
    },
    // Level 285
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "#   GO #",
            "##B # ##",
            "#      #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 32,
    },
    // Level 286
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "#  O   #",
            "###B#  #",
            "#   G  #",
            "########",
        ],
        positions: &[Vector::new(6, 4), Vector::new(5, 4), Vector::new(4, 4)],
        optimal: 36,
    },
    // Level 287
    level! {
        spaces: &[
            "_#######",
            "##  #  #",
            "#  B O #",
            "#   # ##",
            "#G     #",
            "#   #  #",
            "########",
        ],
        positions: &[Vector::new(1, 5), Vector::new(2, 5), Vector::new(3, 5)],
        optimal: 54,
    },
];
