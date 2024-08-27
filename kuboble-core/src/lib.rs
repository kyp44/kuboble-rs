#![cfg_attr(not(feature = "std"), no_std)]
#![feature(variant_count)]
#![feature(let_chains)]

use arrayvec::ArrayVec;
use core::{mem::variant_count, ops::Neg};
use itertools::iproduct;
pub use levels::LEVELS;
pub use render::{Alert, RenderAction};
use strum::{EnumIter, IntoEnumIterator};

#[cfg(feature = "std")]
use std::vec::Vec;

mod levels;
mod render;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Piece {
    #[default]
    Green = 0,
    Orange = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Void,
    Wall,
    Free,
    Goal(Piece),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}
impl<T> Vector<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl core::ops::Add<Vector<i8>> for Vector<u8> {
    type Output = Vector<u8>;

    fn add(self, rhs: Vector<i8>) -> Self::Output {
        Self::new(
            (self.x as i8 + rhs.x).try_into().unwrap(),
            (self.y as i8 + rhs.y).try_into().unwrap(),
        )
    }
}
impl core::ops::Mul<i8> for Vector<i8> {
    type Output = Vector<i8>;

    fn mul(self, rhs: i8) -> Self::Output {
        Self::new(rhs * self.x, rhs * self.y)
    }
}
impl From<Vector<u8>> for Vector<usize> {
    fn from(value: Vector<u8>) -> Self {
        Self::new(value.x.into(), value.y.into())
    }
}

#[derive(Debug, Clone)]
pub struct PieceMap<T>([T; variant_count::<Piece>()]);
impl<T> PieceMap<T> {
    pub fn get(&self, piece: Piece) -> &T {
        &self.0[piece as usize]
    }

    pub fn get_mut(&mut self, piece: Piece) -> &mut T {
        &mut self.0[piece as usize]
    }
}

pub struct Level {
    pub size: Vector<u8>,
    spaces: &'static [Space],
    pub starting_positions: PieceMap<Vector<u8>>,
    pub optimal_moves: u8,
}
impl Level {
    pub fn get_space(&self, position: Vector<u8>) -> Space {
        self.spaces[self.size.x as usize * position.y as usize + position.x as usize]
    }

    pub fn all_positions(&self) -> impl Iterator<Item = Vector<u8>> {
        iproduct!(0..self.size.y, 0..self.size.x).map(|(y, x)| Vector::new(x, y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
impl Direction {
    pub fn as_vector(&self) -> Vector<i8> {
        match self {
            Self::Up => Vector::new(0, -1),
            Self::Down => Vector::new(0, 1),
            Self::Left => Vector::new(-1, 0),
            Self::Right => Vector::new(1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub piece: Piece,
    pub direction: Direction,
}
impl Neg for Move {
    type Output = Move;

    fn neg(self) -> Self::Output {
        Self {
            piece: self.piece,
            direction: -self.direction,
        }
    }
}
impl Move {
    pub const fn new(piece: Piece, direction: Direction) -> Self {
        Self { piece, direction }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceSlid {
    pub muv: Move,
    pub starting_position: Vector<u8>,
    pub is_active: bool,
    pub distance: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OldActivePiece {
    piece: Piece,
    position: Vector<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceMoved {
    pub piece: Piece,
    pub is_active: bool,
    pub from: Vector<u8>,
    pub from_space: Space,
    pub to: Vector<u8>,
}

#[derive(Clone)]
struct BoardState<'a> {
    pub level: &'a Level,
    pub positions: PieceMap<Vector<u8>>,
}
impl<'a> From<&'a Level> for BoardState<'a> {
    fn from(value: &'a Level) -> Self {
        Self {
            level: value,
            positions: value.starting_positions.clone(),
        }
    }
}
impl BoardState<'_> {
    pub fn is_winning(&self) -> bool {
        Piece::iter()
            .all(|piece| self.level.get_space(*self.positions.get(piece)) == Space::Goal(piece))
    }

    pub fn make_move(&mut self, muv: Move, is_active: bool) -> Option<PieceSlid> {
        let starting_position = *self.positions.get(muv.piece);
        let mut position = starting_position.clone();
        let vector = muv.direction.as_vector();

        // Move one space at a time until we hit a wall or another piece.
        let mut distance = 0;
        loop {
            let new_position = position + vector;

            if self.level.get_space(new_position) == Space::Wall
                || Piece::iter().any(|piece| new_position == *self.positions.get(piece))
            {
                break;
            }

            distance += 1;
            position = new_position;
        }

        if distance > 0 {
            *self.positions.get_mut(muv.piece) = position;
            Some(PieceSlid {
                muv,
                starting_position,
                is_active,
                distance,
            })
        } else {
            None
        }
    }
}

// TODO: Reduce this to check win and max at the same time
#[cfg(feature = "std")]
pub const MAX_MOVES: usize = u8::MAX as usize;
#[cfg(not(feature = "std"))]
pub const MAX_MOVES: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Move(Direction),
    ChangeActivePiece,
    UndoMove,
    Restart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelRating(u8);
impl LevelRating {
    pub fn new(goal: u8, num_moves: u8) -> Self {
        Self(if num_moves <= goal {
            5
        } else if num_moves <= goal + 3 {
            5 - (num_moves - goal)
        } else {
            1
        })
    }

    pub fn rating(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum PiecesChanged<'a> {
    Slid {
        piece_slid: PieceSlid,
        old_active_piece: Option<OldActivePiece>,
    },
    Moved(ArrayVec<PieceMoved, { variant_count::<Piece>() }>),
    ActivePiece {
        active_piece: Piece,
        positions: &'a PieceMap<Vector<u8>>,
    },
}

#[derive(Debug, Default, Clone)]
pub struct BoardChanged<'a> {
    pub pieces_changed: Option<PiecesChanged<'a>>,
    pub num_moves_changed: Option<u8>,
    pub winning_rating: Option<LevelRating>,
    pub at_max_moves: bool,
}

pub struct Board<'a> {
    board_state: BoardState<'a>,
    #[cfg(feature = "std")]
    move_stack: Vec<BoardState>,
    #[cfg(not(feature = "std"))]
    move_stack: ArrayVec<Move, MAX_MOVES>,
    active_piece: Piece,
}
impl<'a> From<&'a Level> for Board<'a> {
    fn from(value: &'a Level) -> Self {
        Self {
            board_state: BoardState::from(value),
            move_stack: Default::default(),
            active_piece: Default::default(),
        }
    }
}
impl<'a> Board<'a> {
    pub fn level(&self) -> &'a Level {
        self.board_state.level
    }

    pub fn piece_positions(&self) -> &PieceMap<Vector<u8>> {
        &self.board_state.positions
    }

    fn num_moves(&self) -> u8 {
        self.move_stack.len().try_into().unwrap()
    }

    fn winning_rating(&self) -> Option<LevelRating> {
        self.board_state
            .is_winning()
            .then(|| LevelRating::new(self.level().optimal_moves, self.num_moves()))
    }

    pub fn execute_action(&mut self, action: Action) -> BoardChanged {
        match action {
            Action::Move(d) => self.make_move(d),
            Action::ChangeActivePiece => BoardChanged {
                pieces_changed: Some(self.change_active_piece()),
                ..Default::default()
            },
            Action::UndoMove => self.undo_move(),
            Action::Restart => self.restart(),
        }
    }

    fn make_move(&mut self, direction: Direction) -> BoardChanged {
        let muv = Move::new(self.active_piece, direction);
        let mut new_state = self.board_state.clone();
        let moved = new_state.make_move(muv, true);

        let mut board_changed = BoardChanged::default();

        // TODO: Implement other piece moves (e.g. if the current piece can't move try the other piece in the same direction)

        if let Some(piece_slid) = moved
            && !self.move_stack.is_full()
        {
            self.move_stack.push(muv);
            self.board_state = new_state;
            let num_moves = self.num_moves();

            board_changed.pieces_changed = Some(PiecesChanged::Slid {
                piece_slid,
                old_active_piece: None,
            });
            board_changed.num_moves_changed = Some(num_moves);
            board_changed.winning_rating = self.winning_rating();
        }
        board_changed.at_max_moves = self.move_stack.is_full();

        board_changed
    }

    fn change_active_piece(&mut self) -> PiecesChanged {
        let new_piece = match self.active_piece {
            Piece::Green => Piece::Orange,
            Piece::Orange => Piece::Green,
        };

        self.active_piece = new_piece;

        PiecesChanged::ActivePiece {
            active_piece: new_piece,
            positions: &self.board_state.positions,
        }
    }

    pub fn undo_move(&mut self) -> BoardChanged {
        if !self.move_stack.is_empty() {
            // Find which piece moved back
            let undo_move = -self.move_stack.pop().unwrap();

            // Apply the undo move
            let piece_slid = self
                .board_state
                .make_move(undo_move, undo_move.piece == self.active_piece)
                .unwrap();

            BoardChanged {
                pieces_changed: Some(PiecesChanged::Slid {
                    piece_slid,
                    old_active_piece: None,
                }),
                num_moves_changed: Some(self.num_moves()),
                ..Default::default()
            }
        } else {
            BoardChanged::default()
        }
    }

    pub fn restart(&mut self) -> BoardChanged {
        if self.move_stack.len() > 1 {
            // We need to clear the stack
            let old_state = self.board_state.clone();

            // Reset the state
            self.board_state = BoardState::from(self.level());
            self.move_stack.clear();

            // Reset the active piece
            self.active_piece = Piece::default();

            BoardChanged {
                pieces_changed: Some(PiecesChanged::Moved(
                    Piece::iter()
                        .map(|piece| {
                            let from = *old_state.positions.get(piece);
                            PieceMoved {
                                piece,
                                is_active: piece == self.active_piece,
                                from,
                                from_space: self.level().get_space(from),
                                to: *self.board_state.positions.get(piece),
                            }
                        })
                        .collect(),
                )),
                num_moves_changed: Some(self.num_moves()),
                ..Default::default()
            }
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use core::u8;

    use super::*;

    // TODO: Make some tests
    #[test]
    fn level_rating() {
        let goal = 6;
        assert_eq!(LevelRating::new(goal, goal).rating(), 5);
        assert_eq!(LevelRating::new(goal, goal + 1).rating(), 4);
        assert_eq!(LevelRating::new(goal, goal + 2).rating(), 3);
        assert_eq!(LevelRating::new(goal, goal + 3).rating(), 2);
        assert_eq!(LevelRating::new(goal, goal + 4).rating(), 1);
        assert_eq!(LevelRating::new(goal, goal + 50).rating(), 1);
        assert_eq!(LevelRating::new(goal, u8::MAX).rating(), 1);
    }
}
