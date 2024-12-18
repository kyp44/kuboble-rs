#![cfg_attr(not(feature = "std"), no_std)]
#![feature(variant_count)]
#![feature(let_chains)]
#![feature(generic_const_exprs)]

use core::{
    cmp,
    ops::{Index, IndexMut},
};
use enum_map::{Enum, EnumMap};
use itertools::iproduct;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

pub mod level_run;
pub mod level_select;
pub mod levels;

// NOTE: We cannot use a library like `nalgebra` because we need a const constructor.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
        Self::new((self.x as i8 + rhs.x) as u8, (self.y as i8 + rhs.y) as u8)
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
impl<T: Ord> PartialOrd for Vector<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord> Ord for Vector<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let o = self.y.cmp(&other.y);

        match o {
            cmp::Ordering::Equal => self.x.cmp(&other.x),
            _ => o,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter, Enum)]
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
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'G' => Some(Self::Green),
            'O' => Some(Self::Orange),
            'B' => Some(Self::Blue),
            _ => None,
        }
    }
}
impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Green => 'G',
            Piece::Orange => 'O',
            Piece::Blue => 'B',
        }
    }
}
impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Clone)]
pub struct PieceMap<T> {
    num_pieces: usize,
    map: EnumMap<Piece, T>,
}
impl<T> PieceMap<T> {
    pub fn pieces(&self) -> impl Iterator<Item = Piece> {
        Piece::iter().take(self.num_pieces)
    }
}
impl<T> Index<Piece> for PieceMap<T> {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        self.map.index(index)
    }
}
impl<T> IndexMut<Piece> for PieceMap<T> {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        self.map.index_mut(index)
    }
}
impl<T: Clone + Default> From<&[T]> for PieceMap<T> {
    fn from(value: &[T]) -> Self {
        Self {
            num_pieces: value.len(),
            map: Piece::iter()
                .zip(value.iter())
                .map(|(k, v)| (k, v.clone()))
                .collect(),
        }
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
impl Space {
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '_' => Some(Self::Void),
            '#' => Some(Self::Wall),
            ' ' => Some(Self::Free),
            _ => match Piece::from_char(c) {
                Some(p) => Some(Self::Goal(p)),
                None => None,
            },
        }
    }
}
impl From<Space> for char {
    fn from(value: Space) -> Self {
        match value {
            Space::Void => '_',
            Space::Wall => '#',
            Space::Free => ' ',
            Space::Goal(piece) => piece.into(),
        }
    }
}
impl core::fmt::Display for Space {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug)]
pub struct Level {
    pub size: Vector<u8>,
    spaces: &'static [Space],
    pub starting_positions: &'static [Vector<u8>],
    pub optimal_moves: u8,
}
impl Level {
    pub fn num_pieces(&self) -> u8 {
        self.starting_positions.len() as u8
    }

    pub fn all_pieces(&self) -> impl Iterator<Item = Piece> {
        Piece::iter().take(self.num_pieces() as usize)
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

pub trait BufferedRenderer {
    fn flush(&mut self);
}

#[cfg(test)]
mod tests {
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
