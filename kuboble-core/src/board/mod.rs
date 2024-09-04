use crate::{level_select::LevelInfo, Level, LevelRating, Piece, PieceMap, Space, Vector};
use arrayvec::ArrayVec;
use core::{mem::variant_count, ops::Neg};
use strum::IntoEnumIterator;

pub mod render;

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
    pub starting_space: Space,
    pub distance: u8,
}
impl PieceSlid {
    pub fn invert(self, level: &Level) -> Self {
        let starting_position =
            self.starting_position + self.muv.direction.as_vector() * self.distance as i8;

        Self {
            muv: -self.muv,
            starting_position,
            starting_space: level.get_space(starting_position),
            distance: self.distance,
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
                || Piece::iter().any(|piece| new_position == self.piece_position(piece))
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
                starting_space: self.level.get_space(starting_position),
                distance,
            })
        } else {
            None
        }
    }
}

#[cfg(not(feature = "std"))]
pub const MAX_MOVES: usize = 50;

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
pub struct BoardChanged<'a> {
    pub pieces_changed: Option<PiecesChanged<'a>>,
    pub num_moves_changed: Option<u8>,
    pub winning_rating: Option<LevelRating>,
    pub at_max_moves: bool,
}

pub struct Board<'a> {
    level_num: u16,
    board_state: BoardState<'a>,
    #[cfg(feature = "std")]
    move_stack: Vec<PieceSlid>,
    #[cfg(not(feature = "std"))]
    move_stack: ArrayVec<PieceSlid, MAX_MOVES>,
    active_piece: Piece,
}
impl<'a> Board<'a> {
    pub fn new(level_info: &LevelInfo) -> Self {
        Self {
            level_num: level_info.user_num(),
            board_state: BoardState::from(level_info.level),
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
            Action::Move(d) => self.attempt_move(d),
            Action::ChangeActivePiece => BoardChanged {
                pieces_changed: Some(self.change_active_piece()),
                ..Default::default()
            },
            Action::UndoMove => self.undo_move(),
            Action::Restart => self.restart(),
        }
    }

    fn attempt_move(&mut self, direction: Direction) -> BoardChanged {
        let mut muv = Move::new(self.active_piece, direction);
        let mut new_state = self.board_state.clone();
        let mut old_active_piece = None;

        // Try to move the active piece
        let mut moved = new_state.attempt_move(muv);

        let mut board_changed = BoardChanged::default();

        // If the active piece cannot move, can the other piece?
        if moved.is_none() {
            for piece in Piece::iter().filter(|p| *p != self.active_piece) {
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
            self.board_state = new_state;

            board_changed.pieces_changed = Some(PiecesChanged::Slid {
                piece_slid,
                is_active: true,
                old_active_piece,
            });
            board_changed.num_moves_changed = Some(self.num_moves());
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
            // Determine the inverse move
            let undo_slide = self.move_stack.pop().unwrap();

            // Apply the inverse move.
            self.board_state
                .teleport_piece(undo_slide.muv.piece, undo_slide.starting_position);
            let is_active = undo_slide.muv.piece == self.active_piece;

            BoardChanged {
                pieces_changed: Some(PiecesChanged::Slid {
                    piece_slid: undo_slide.invert(&self.level()),
                    is_active,
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
        if !self.move_stack.is_empty() {
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
                            let from = old_state.piece_position(piece);
                            PieceMoved {
                                piece,
                                is_active: piece == self.active_piece,
                                from,
                                from_space: self.level().get_space(from),
                                to: self.board_state.piece_position(piece),
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
