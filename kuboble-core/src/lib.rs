#![cfg_attr(not(feature = "std"), no_std)]
#![feature(variant_count)]

use core::mem::variant_count;
use itertools::iproduct;
pub use levels::LEVELS;
use smallvec::{smallvec, SmallVec};
use strum::{EnumIter, IntoEnumIterator};

#[cfg(feature = "std")]
use std::vec::Vec;

mod levels;

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
    const fn new(x: T, y: T) -> Self {
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

    pub fn positions(&self) -> impl Iterator<Item = Vector<u8>> {
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
impl Direction {
    fn as_vector(&self) -> Vector<i8> {
        match self {
            Direction::Up => Vector::new(0, -1),
            Direction::Down => Vector::new(0, 1),
            Direction::Left => Vector::new(-1, 0),
            Direction::Right => Vector::new(1, 0),
        }
    }
}

// NOTE: Only the active piece ever moved
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceMoved {
    pub piece: Piece,
    pub from: Vector<u8>,
    pub to: Vector<u8>,
}

// Note: We do not store a reference to the Level to keep the size down
#[derive(Clone)]
struct BoardState {
    pub positions: PieceMap<Vector<u8>>,
}
impl From<&Level> for BoardState {
    fn from(value: &Level) -> Self {
        Self {
            positions: value.starting_positions.clone(),
        }
    }
}
impl BoardState {
    pub fn is_winning(&self, level: &Level) -> bool {
        Piece::iter().all(|piece| level.get_space(*self.positions.get(piece)) == Space::Goal(piece))
    }

    pub fn make_move(
        &mut self,
        level: &Level,
        piece: Piece,
        direction: Direction,
    ) -> Option<PieceMoved> {
        let starting_position = *self.positions.get(piece);
        let mut position = starting_position.clone();

        // Move one space at a time until we hit a wall or another piece.
        loop {
            let new_position = position + direction.as_vector();

            if level.get_space(new_position) == Space::Wall
                || Piece::iter().any(|piece| new_position == *self.positions.get(piece))
            {
                break;
            }

            position = new_position;
        }

        if starting_position != position {
            *self.positions.get_mut(piece) = position;
            Some(PieceMoved {
                piece,
                true,
                from: starting_position,
                to: position,
            })
        } else {
            None
        }
    }
}

#[cfg(feature = "std")]
pub const MAX_MOVE_STACK: usize = u8::MAX as usize;
#[cfg(not(feature = "std"))]
pub const MAX_MOVE_STACK: usize = 50 + 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Move(Direction),
    ChangePiece,
    Undo,
    Restart,
}

#[derive(Debug, Clone)]
pub struct PieceChanged<'a> {
    pub active_piece: Piece,
    pub positions: &'a PieceMap<Vector<u8>>,
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

#[derive(Debug, Default, Clone)]
pub struct BoardChange<'a> {
    pub moved_pieces: SmallVec<[PieceMoved; variant_count::<Piece>()]>,
    pub active_piece_changed: Option<PieceChanged<'a>>,
    pub num_moves_changed: Option<u8>,
    pub winning_position: Option<LevelRating>,
    pub at_max_moves: bool,
}

pub struct Board<'a> {
    pub level: &'a Level,
    #[cfg(feature = "std")]
    move_stack: Vec<BoardState>,
    #[cfg(not(feature = "std"))]
    move_stack: SmallVec<[BoardState; MAX_MOVE_STACK]>,
    current_piece: Piece,
}
impl<'a> From<&'a Level> for Board<'a> {
    fn from(value: &'a Level) -> Self {
        Self {
            level: value,
            #[cfg(feature = "std")]
            move_stack: vec![BoardState::from(value)],
            #[cfg(not(feature = "std"))]
            move_stack: smallvec![BoardState::from(value)],
            current_piece: Default::default(),
        }
    }
}
impl Board<'_> {
    // TODO: If we change to a stack of moves this can just be a reference
    fn current_board_state(&self) -> &BoardState {
        self.move_stack.last().unwrap()
    }

    pub fn piece_positions(&self) -> &PieceMap<Vector<u8>> {
        &self.current_board_state().positions
    }

    fn num_moves(&self) -> u8 {
        (self.move_stack.len() - 1).try_into().unwrap()
    }

    fn move_stack_full(&self) -> bool {
        self.move_stack.len() >= MAX_MOVE_STACK
    }

    pub fn execute_action(&mut self, action: Action) -> BoardChange {
        match action {
            Action::Move(d) => self.make_move(d),
            Action::ChangePiece => BoardChange {
                active_piece_changed: Some(self.change_piece()),
                ..Default::default()
            },
            Action::Undo => match self.undo() {
                Some(moved) => BoardChange {
                    moved_pieces: smallvec![moved],
                    num_moves_changed: Some(self.num_moves()),
                    ..Default::default()
                },
                None => BoardChange::default(),
            },
            Action::Restart => todo!(),
        }
    }

    fn make_move(&mut self, direction: Direction) -> BoardChange {
        let mut new_state = self.current_board_state().clone();
        let moved = new_state.make_move(self.level, self.current_piece, direction);

        // TODO: Implement other piece moves (e.g. if the current piece can't move try the other piece in the same direction)

        match moved {
            Some(m) => {
                if self.move_stack_full() {
                    BoardChange {
                        at_max_moves: true,
                        ..Default::default()
                    }
                } else {
                    self.move_stack.push(new_state);
                    let num_moves = self.num_moves();

                    BoardChange {
                        moved_pieces: smallvec![m],
                        num_moves_changed: Some(num_moves),
                        winning_position: self
                            .current_board_state()
                            .is_winning(self.level)
                            .then(|| LevelRating::new(self.level.optimal_moves, num_moves)),
                        ..Default::default()
                    }
                }
            }
            None => BoardChange::default(),
        }
    }

    fn change_piece(&mut self) -> PieceChanged {
        let new_piece = match self.current_piece {
            Piece::Green => Piece::Orange,
            Piece::Orange => Piece::Green,
        };

        self.current_piece = new_piece;

        PieceChanged {
            active_piece: new_piece,
            positions: &self.current_board_state().positions,
        }
    }

    // TODO: May not need this if the stack is moves instead of board states
    fn piece_change(
        &self,
        old_state: &BoardState,
        new_state: &BoardState,
        piece: Piece,
    ) -> Option<PieceMoved> {
        let old_position = old_state.positions.get(piece);
        let new_position = new_state.positions.get(piece);

        (old_position != new_position).then(|| PieceMoved {
            piece,
            from: *old_position,
            to: *new_position,
        })
    }

    pub fn undo(&mut self) -> Option<PieceMoved> {
        if self.move_stack.len() > 1 {
            // Find which piece moved back
            let old_state = self.move_stack.pop().unwrap();
            let new_state = self.current_board_state();

            for piece in Piece::iter() {
                let moved = self.piece_change(&old_state, new_state, piece);
                if moved.is_some() {
                    return moved;
                }
            }
        }

        None
    }

    pub fn restart(&mut self) -> BoardChange {
        if self.move_stack.len() > 1 {
            // We need to clear the stack
            let old_state = self.move_stack.pop().unwrap();
            let new_state = BoardState::from(self.level);
            self.move_stack.clear();

            // Determine which pieces have changed locations
            let moved_pieces = Piece::iter()
                .filter_map(|piece| self.piece_change(&old_state, &new_state, piece))
                .collect();

            self.move_stack.push(new_state);

            // Select the default piece
            let active_piece_changed = (self.current_piece != Piece::default()).then(|| {
                self.current_piece = Piece::default();
                PieceChanged {
                    active_piece: self.current_piece,
                    positions: &self.current_board_state().positions,
                }
            });

            BoardChange {
                moved_pieces,
                active_piece_changed,
                num_moves_changed: Some(0),
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
