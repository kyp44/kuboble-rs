use crate::{
    level_select::{LevelInfo, LevelStatus},
    levels::MAX_STRIP_SIZE,
    Level, LevelRating, Piece, PieceMap, Space, Vector,
};
use arrayvec::{ArrayString, ArrayVec};
use core::{fmt::Write, mem::variant_count, ops::Neg};
use itertools::iproduct;
use lazy_static::lazy_static;
use serde::{de, Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

pub mod render;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
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
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'U' => Some(Self::Up),
            'D' => Some(Self::Down),
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None,
        }
    }

    pub fn as_vector(&self) -> Vector<i8> {
        match self {
            Self::Up => Vector::new(0, -1),
            Self::Down => Vector::new(0, 1),
            Self::Left => Vector::new(-1, 0),
            Self::Right => Vector::new(1, 0),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            Self::Left | Self::Right => true,
            Self::Up | Self::Down => false,
        }
    }

    pub fn is_forward(&self) -> bool {
        match self {
            Self::Down | Self::Right => true,
            Self::Up | Self::Left => false,
        }
    }
}
impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}
impl core::fmt::Display for Direction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", char::from(*self))
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
impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s: ArrayString<2> = ArrayString::new();
        write!(s, "{}{}", self.piece, self.direction).unwrap();
        serializer.serialize_str(&s)
    }
}
impl<'de> Deserialize<'de> for Move {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const NUM_VARIANTS: usize = variant_count::<Direction>() * variant_count::<Piece>();
        lazy_static! {
            static ref MOVE_VARIANTS_OWNED: ArrayVec<ArrayString<2>, NUM_VARIANTS> =
                ArrayVec::from_iter(iproduct!(Piece::iter(), Direction::iter()).map(
                    |(piece, dir)| {
                        let mut s = ArrayString::new();
                        write!(s, "{piece}{dir}").unwrap();
                        s
                    },
                ));
            static ref MOVE_VARIANTS: ArrayVec<&'static str, NUM_VARIANTS> =
                MOVE_VARIANTS_OWNED.iter().map(|s| s.as_str()).collect();
        }

        let s: ArrayString<2> = ArrayString::deserialize(deserializer)?;
        let cs: ArrayVec<char, 2> = s.chars().collect();

        Ok(Move {
            piece: Piece::from_char(cs[0]).ok_or(de::Error::unknown_variant(&s, &MOVE_VARIANTS))?,
            direction: Direction::from_char(cs[1])
                .ok_or(de::Error::unknown_variant(&s, &MOVE_VARIANTS))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceSlid {
    pub muv: Move,
    pub strip_top_left: Vector<u8>,
    pub strip_spaces: ArrayVec<Space, MAX_STRIP_SIZE>,
}
impl PieceSlid {
    pub fn starting_position(&self) -> Vector<u8> {
        match self.muv.direction {
            Direction::Right | Direction::Down => self.strip_top_left,
            _ => {
                self.strip_top_left
                    + (-self.muv.direction).as_vector() * self.slide_distance() as i8
            }
        }
    }

    pub fn slide_distance(&self) -> u8 {
        self.strip_spaces.len() as u8 - 1
    }
}
impl Neg for PieceSlid {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            muv: -self.muv,
            strip_top_left: self.strip_top_left,
            strip_spaces: self.strip_spaces,
        }
    }
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
struct LevelRunState<'a> {
    pub level: &'a Level,
    pub positions: PieceMap<Vector<u8>>,
}
impl<'a> From<&'a Level> for LevelRunState<'a> {
    fn from(value: &'a Level) -> Self {
        Self {
            level: value,
            positions: value.starting_positions.into(),
        }
    }
}
impl LevelRunState<'_> {
    pub fn is_winning(&self) -> bool {
        self.level
            .all_pieces()
            .all(|piece| self.level.get_space(self.piece_position(piece)) == Space::Goal(piece))
    }

    #[inline]
    pub fn piece_position(&self, piece: Piece) -> Vector<u8> {
        *self.positions.get(piece)
    }

    #[inline]
    pub fn teleport_piece(&mut self, piece: Piece, new_position: Vector<u8>) {
        *self.positions.get_mut(piece) = new_position;
    }

    pub fn attempt_move(&mut self, muv: Move) -> Option<PieceSlid> {
        let starting_position = self.piece_position(muv.piece);
        let mut position = starting_position.clone();
        let vector = muv.direction.as_vector();

        // Move one space at a time until we hit a wall or another piece.
        let mut distance = 0;
        loop {
            let new_position = position + vector;

            if self.level.get_space(new_position) == Space::Wall
                || self
                    .level
                    .all_pieces()
                    .any(|piece| new_position == self.piece_position(piece))
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
                strip_top_left: starting_position.min(position),
                strip_spaces: if muv.direction.is_horizontal() {
                    (starting_position.x.min(position.x)..=starting_position.x.max(position.x))
                        .map(|x| self.level.get_space(Vector::new(x, position.y)))
                        .collect()
                } else {
                    (starting_position.y.min(position.y)..=starting_position.y.max(position.y))
                        .map(|y| self.level.get_space(Vector::new(position.x, y)))
                        .collect()
                },
            })
        } else {
            None
        }
    }
}

#[cfg(not(feature = "std"))]
pub const MAX_MOVES: usize = 100;

#[cfg(feature = "std")]
trait VecExt {
    fn is_full(&self) -> bool;
}
#[cfg(feature = "std")]
impl<T> VecExt for Vec<T> {
    fn is_full(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Move(Direction),
    ChangeActivePiece,
    UndoMove,
    Restart,
}

#[derive(Debug, Clone)]
pub enum PiecesChanged<'a> {
    Slid {
        piece_slid: PieceSlid,
        is_active: bool,
        old_active_piece: Option<OldActivePiece>,
    },
    Moved(ArrayVec<PieceMoved, { variant_count::<Piece>() }>),
    ActivePiece {
        active_piece: Piece,
        positions: &'a PieceMap<Vector<u8>>,
    },
}

#[derive(Debug, Default, Clone)]
pub struct LevelRunChange<'a> {
    pub pieces_changed: Option<PiecesChanged<'a>>,
    pub num_moves_changed: Option<u8>,
    pub winning_status: Option<LevelStatus>,
    pub at_max_moves: bool,
}

pub struct LevelRun<'a> {
    level_num: u16,
    state: LevelRunState<'a>,
    #[cfg(feature = "std")]
    move_stack: Vec<PieceSlid>,
    #[cfg(not(feature = "std"))]
    move_stack: ArrayVec<PieceSlid, MAX_MOVES>,
    active_piece: Piece,
}
impl<'a> LevelRun<'a> {
    pub fn new(level_info: &LevelInfo) -> Self {
        Self {
            level_num: level_info.user_num(),
            state: LevelRunState::from(level_info.level),
            move_stack: Default::default(),
            active_piece: Default::default(),
        }
    }
}
impl<'a> LevelRun<'a> {
    pub fn level(&self) -> &'a Level {
        self.state.level
    }

    pub fn piece_positions(&self) -> &PieceMap<Vector<u8>> {
        &self.state.positions
    }

    fn num_moves(&self) -> u8 {
        self.move_stack.len() as u8
    }

    fn winning_status(&self) -> Option<LevelStatus> {
        self.state.is_winning().then(|| {
            let rating = LevelRating::new(self.level().optimal_moves, self.num_moves());

            if rating.is_optimal() {
                LevelStatus::Optimal(self.move_stack.iter().map(|s| s.muv).collect())
            } else {
                LevelStatus::Complete(rating)
            }
        })
    }

    pub fn execute_action(&mut self, action: Action) -> LevelRunChange {
        match action {
            Action::Move(d) => self.attempt_move(d),
            Action::ChangeActivePiece => LevelRunChange {
                pieces_changed: Some(self.change_active_piece()),
                ..Default::default()
            },
            Action::UndoMove => self.undo_move(),
            Action::Restart => self.restart(),
        }
    }

    fn attempt_move(&mut self, direction: Direction) -> LevelRunChange {
        let mut muv = Move::new(self.active_piece, direction);
        let mut new_state = self.state.clone();
        let mut old_active_piece = None;

        // Try to move the active piece
        let mut moved = new_state.attempt_move(muv);

        let mut change = LevelRunChange::default();

        // If the active piece cannot move, can the other piece?
        if moved.is_none() {
            for piece in self
                .level()
                .all_pieces()
                .filter(|p| *p != self.active_piece)
            {
                muv = Move::new(piece, direction);
                moved = new_state.attempt_move(muv);
                if let Some(ref slid) = moved {
                    // Make the new piece active
                    old_active_piece = Some(OldActivePiece {
                        piece: self.active_piece,
                        position: new_state.piece_position(self.active_piece),
                    });
                    self.active_piece = slid.muv.piece;
                    break;
                }
            }
        }

        if let Some(piece_slid) = moved
            && !self.move_stack.is_full()
        {
            self.move_stack.push(piece_slid.clone());
            self.state = new_state;

            change.pieces_changed = Some(PiecesChanged::Slid {
                piece_slid,
                is_active: true,
                old_active_piece,
            });
            change.num_moves_changed = Some(self.num_moves());
            change.winning_status = self.winning_status();
        }
        change.at_max_moves = self.move_stack.is_full();

        change
    }

    fn change_active_piece(&mut self) -> PiecesChanged {
        let new_piece =
            Piece::try_from((self.active_piece as u8 + 1) % self.level().num_pieces()).unwrap();

        self.active_piece = new_piece;

        PiecesChanged::ActivePiece {
            active_piece: new_piece,
            positions: &self.state.positions,
        }
    }

    pub fn undo_move(&mut self) -> LevelRunChange {
        if !self.move_stack.is_empty() {
            // Determine the inverse move
            let undo_slide = self.move_stack.pop().unwrap();

            // Apply the inverse move.
            self.state
                .teleport_piece(undo_slide.muv.piece, undo_slide.starting_position());
            let is_active = undo_slide.muv.piece == self.active_piece;

            LevelRunChange {
                pieces_changed: Some(PiecesChanged::Slid {
                    piece_slid: -undo_slide,
                    is_active,
                    old_active_piece: None,
                }),
                num_moves_changed: Some(self.num_moves()),
                ..Default::default()
            }
        } else {
            LevelRunChange::default()
        }
    }

    pub fn restart(&mut self) -> LevelRunChange {
        if !self.move_stack.is_empty() {
            // We need to clear the stack
            let old_state = self.state.clone();

            // Reset the state
            self.state = LevelRunState::from(self.level());
            self.move_stack.clear();

            // Reset the active piece
            self.active_piece = Piece::default();

            LevelRunChange {
                pieces_changed: Some(PiecesChanged::Moved(
                    self.level()
                        .all_pieces()
                        .map(|piece| {
                            let from = old_state.piece_position(piece);
                            PieceMoved {
                                piece,
                                is_active: piece == self.active_piece,
                                from,
                                from_space: self.level().get_space(from),
                                to: self.state.piece_position(piece),
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

// TODO: Write some tests!
