use core::iter;

use smallvec::{smallvec, SmallVec};
use strum::IntoEnumIterator;

use crate::{
    Board, BoardChanged, LevelRating, OldActivePiece, Piece, PieceMoved, PieceSlid, PiecesChanged,
    Space, Vector,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alert {
    // You win with the rating
    Win(LevelRating),
    MaxMoves,
}

pub enum RenderAction {
    DrawSpace {
        position: Vector<u8>,
        space: Space,
    },
    DrawPiece {
        position: Vector<u8>,
        piece: Piece,
        is_active: bool,
    },
    SlidePiece(PieceSlid),
    UpdateNumMoves(u8),
    UpdateGoal(u8),
    Alert(Alert),
}

impl Board<'_> {
    // Appends the winning or max moves alerts, or neither.
    fn append_render_alerts<'a>(
        &'a self,
        iter: impl Iterator<Item = RenderAction> + 'a,
    ) -> impl Iterator<Item = RenderAction> + 'a {
        iter.chain(
            match self.winning_rating() {
                Some(r) => iter::once(RenderAction::Alert(Alert::Win(r))),
                None => iter::once(RenderAction::Alert(Alert::MaxMoves)),
            }
            .filter(|ra| {
                if let RenderAction::Alert(a) = ra {
                    *a == Alert::MaxMoves && self.move_stack_full()
                } else {
                    true
                }
            }),
        )
    }
    // For drawing the entire current board
    pub fn render_actions(&self) -> impl Iterator<Item = RenderAction> + '_ {
        let level = self.board_state.level;

        self.append_render_alerts(
            self.board_state
                .level
                .all_positions()
                .map(|position| RenderAction::DrawSpace {
                    position,
                    space: level.get_space(position),
                })
                .chain(Piece::iter().map(|piece| RenderAction::DrawPiece {
                    position: *self.board_state.positions.get(piece),
                    piece: piece,
                    is_active: self.active_piece == piece,
                }))
                .chain([
                    RenderAction::UpdateNumMoves(self.num_moves()),
                    RenderAction::UpdateGoal(level.optimal_moves),
                ]),
        )
    }
}

impl From<PieceSlid> for RenderAction {
    fn from(value: PieceSlid) -> Self {
        RenderAction::SlidePiece(value)
    }
}
impl From<OldActivePiece> for RenderAction {
    fn from(value: OldActivePiece) -> Self {
        RenderAction::DrawPiece {
            position: value.position,
            piece: value.piece,
            is_active: false,
        }
    }
}
impl From<PieceMoved> for [RenderAction; 2] {
    fn from(value: PieceMoved) -> Self {
        [
            RenderAction::DrawSpace {
                position: value.from,
                space: value.from_space,
            },
            RenderAction::DrawPiece {
                position: value.to,
                piece: value.piece,
                is_active: value.is_active,
            },
        ]
    }
}

impl PiecesChanged<'_> {
    pub fn render_actions(self) -> SmallVec<[RenderAction; 2]> {
        match self {
            PiecesChanged::Slid {
                piece_slid,
                old_active_piece,
            } => {
                let mut vec = smallvec![piece_slid.into()];
                if let Some(oap) = old_active_piece {
                    vec.push(oap.into())
                }
                vec
            }
            PiecesChanged::Moved(moved) => todo!(),
            PiecesChanged::ActivePiece {
                active_piece,
                positions,
            } => todo!(),
        }
    }
}

impl BoardChanged<'_> {
    // TODO: How big does this vec need to be?
    pub fn render_actions(self) -> SmallVec<[RenderAction; 40]> {
        todo!()
    }
}
