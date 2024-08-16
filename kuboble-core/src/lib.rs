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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub piece: Piece,
    pub from: Vector<u8>,
    pub to: Vector<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveStatus {
    NoEffect,
    MaxMoves,
    MoveMade(Move),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardStatus {
    pub num_moves: usize,
    pub at_max_moves: bool,
    pub winning_position: bool,
}

// Note: We do not store a reference to the Level to keep the size down
#[derive(Clone)]
pub struct BoardState {
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

    pub fn make_move(&mut self, level: &Level, piece: Piece, direction: Direction) -> Option<Move> {
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
            Some(Move {
                piece,
                from: starting_position,
                to: position,
            })
        } else {
            None
        }
    }
}

#[cfg(not(feature = "std"))]
pub const MAX_MOVE_STACK: usize = 50 + 1;

pub struct Board<'a> {
    pub level: &'a Level,
    #[cfg(feature = "std")]
    move_stack: Vec<BoardState>,
    #[cfg(not(feature = "std"))]
    move_stack: SmallVec<[BoardState; MAX_MOVES]>,
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
    pub fn current_board_state(&self) -> &BoardState {
        self.move_stack.last().unwrap()
    }

    #[cfg(feature = "std")]
    fn move_stack_full(&self) -> bool {
        false
    }

    #[cfg(not(feature = "std"))]
    fn move_stack_full(&self) -> bool {
        self.move_stack.len() >= MAX_MOVES
    }

    pub fn change_piece(&mut self) -> Piece {
        let new_piece = match self.current_piece {
            Piece::Green => Piece::Orange,
            Piece::Orange => Piece::Green,
        };

        self.current_piece = new_piece;
        new_piece
    }

    pub fn make_move(&mut self, direction: Direction) -> MoveStatus {
        let mut new_state = self.current_board_state().clone();
        let muv = new_state.make_move(self.level, self.current_piece, direction);

        match muv {
            Some(m) => {
                if self.move_stack_full() {
                    MoveStatus::MaxMoves
                } else {
                    self.move_stack.push(new_state);
                    MoveStatus::MoveMade(m)
                }
            }
            None => MoveStatus::NoEffect,
        }
    }

    pub fn board_status(&self) -> BoardStatus {
        BoardStatus {
            num_moves: self.move_stack.len() - 1,
            at_max_moves: self.move_stack_full(),
            winning_position: self.current_board_state().is_winning(self.level),
        }
    }

    fn piece_change(
        &self,
        old_state: &BoardState,
        new_state: &BoardState,
        piece: Piece,
    ) -> Option<Move> {
        let old_position = old_state.positions.get(piece);
        let new_position = new_state.positions.get(piece);

        (old_position != new_position).then(|| Move {
            piece,
            from: *old_position,
            to: *new_position,
        })
    }

    pub fn undo(&mut self) -> Option<Move> {
        if self.move_stack.len() > 1 {
            // Find which piece moved back
            let old_state = self.move_stack.pop().unwrap();
            let new_state = self.current_board_state();

            for piece in Piece::iter() {
                let muv = self.piece_change(&old_state, new_state, piece);
                if muv.is_some() {
                    return muv;
                }
            }
        }

        None
    }

    pub fn restart(&mut self) -> SmallVec<[Move; variant_count::<Piece>()]> {
        if self.move_stack.len() > 1 {
            // We need to clear the stack
            let old_state = self.move_stack.pop().unwrap();
            let new_state = BoardState::from(self.level);
            self.move_stack.clear();

            // Determine which pieces have changed locations
            let moves = Piece::iter()
                .filter_map(|piece| self.piece_change(&old_state, &new_state, piece))
                .collect();

            self.move_stack.push(new_state);

            moves
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make some tests
    #[test]
    fn winning() {
        /* let moves = [
            Direction::
        ] */
        assert!(false)
    }
}
