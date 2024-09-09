#![cfg_attr(not(feature = "std"), no_std)]
#![feature(variant_count)]
#![feature(let_chains)]
#![feature(generic_const_exprs)]

use arrayvec::ArrayVec;
use core::mem::variant_count;
use itertools::iproduct;
use serde::{Deserialize, Serialize};

pub mod board;
pub mod level_select;
mod levels;

// TODO: Should we switch to something like this instead of our own thing?
// https://crates.io/crates/nalgebra
// This is compatible with embedded-graphics point types too in that it provides conversions... might be nice.
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Piece {
    #[default]
    Green = 0,
    Orange = 1,
    Blue = 2,
}
impl TryFrom<u8> for Piece {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Green),
            1 => Ok(Self::Orange),
            2 => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}
impl Piece {
    pub fn iter(num: u8) -> impl Iterator<Item = Piece> {
        [Self::Green, Self::Orange, Self::Blue]
            .into_iter()
            .take(num as usize)
    }

    pub fn iter_all() -> impl Iterator<Item = Piece> {
        Self::iter(variant_count::<Piece>().try_into().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct PieceMap<T>(ArrayVec<T, { variant_count::<Piece>() }>);
impl<T> PieceMap<T> {
    pub fn pieces(&self) -> impl Iterator<Item = Piece> {
        Piece::iter(self.0.len().try_into().unwrap())
    }

    pub fn get(&self, piece: Piece) -> &T {
        &self.0[piece as usize]
    }

    pub fn get_mut(&mut self, piece: Piece) -> &mut T {
        &mut self.0[piece as usize]
    }
}
impl<T: Copy> From<&[T]> for PieceMap<T> {
    fn from(value: &[T]) -> Self {
        let mut array_vec = ArrayVec::new();
        array_vec.try_extend_from_slice(value).unwrap();

        Self(array_vec)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    #[default]
    Void,
    Wall,
    Free,
    Goal(Piece),
}

pub struct Level {
    pub size: Vector<u8>,
    spaces: &'static [Space],
    pub starting_positions: &'static [Vector<u8>],
    pub optimal_moves: u8,
}
impl Level {
    pub fn num_pieces(&self) -> u8 {
        self.starting_positions.len().try_into().unwrap()
    }

    pub fn all_pieces(&self) -> impl Iterator<Item = Piece> {
        Piece::iter(self.num_pieces())
    }

    pub fn user_size(&self) -> Vector<u8> {
        Vector::new(self.size.x - 2, self.size.y - 2)
    }

    pub fn get_space(&self, position: Vector<u8>) -> Space {
        self.spaces[self.size.x as usize * position.y as usize + position.x as usize]
    }

    pub fn all_positions(&self) -> impl Iterator<Item = Vector<u8>> {
        iproduct!(0..self.size.y, 0..self.size.x).map(|(y, x)| Vector::new(x, y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LevelRating(u8);
impl LevelRating {
    pub fn new(goal: u8, num_moves: u8) -> Self {
        let max = Self::maximum_possible().num_stars();

        Self(if num_moves <= goal {
            max
        } else if num_moves <= goal + 3 {
            max - (num_moves - goal)
        } else {
            1
        })
    }

    #[inline]
    pub fn is_complete(&self) -> bool {
        self.0 > 0
    }

    #[inline]
    pub fn is_optimal(&self) -> bool {
        *self == Self::maximum_possible()
    }

    #[inline]
    pub fn num_stars(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn maximum_possible() -> Self {
        Self(5)
    }
}
impl Default for LevelRating {
    fn default() -> Self {
        Self(0)
    }
}

#[cfg(test)]
mod tests {
    use core::u8;

    use super::*;

    #[test]
    fn level_rating() {
        let goal = 6;
        assert_eq!(LevelRating::new(goal, goal).num_stars(), 5);
        assert_eq!(LevelRating::new(goal, goal + 1).num_stars(), 4);
        assert_eq!(LevelRating::new(goal, goal + 2).num_stars(), 3);
        assert_eq!(LevelRating::new(goal, goal + 3).num_stars(), 2);
        assert_eq!(LevelRating::new(goal, goal + 4).num_stars(), 1);
        assert_eq!(LevelRating::new(goal, goal + 50).num_stars(), 1);
        assert_eq!(LevelRating::new(goal, u8::MAX).num_stars(), 1);
    }
}
