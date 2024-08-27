use arrayvec::ArrayVec;
use core::{iter, mem::variant_count};
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
    Clear,
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
                    *a == Alert::MaxMoves && self.move_stack.is_full()
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
impl PieceMoved {
    pub fn render_actions(&self) -> [RenderAction; 2] {
        [
            RenderAction::DrawSpace {
                position: self.from,
                space: self.from_space,
            },
            RenderAction::DrawPiece {
                position: self.to,
                piece: self.piece,
                is_active: self.is_active,
            },
        ]
    }
}

impl PiecesChanged<'_> {
    pub fn render_actions(self) -> ArrayVec<RenderAction, { 2 * variant_count::<Piece>() }> {
        let mut actions = ArrayVec::new();

        match self {
            PiecesChanged::Slid {
                piece_slid,
                old_active_piece,
            } => {
                actions.push(piece_slid.into());
                if let Some(oap) = old_active_piece {
                    actions.push(oap.into())
                }
            }
            PiecesChanged::Moved(moved) => {
                actions.extend(
                    moved
                        .into_iter()
                        .map(|pm| pm.render_actions().into_iter())
                        .flatten(),
                );
            }
            PiecesChanged::ActivePiece {
                active_piece,
                positions,
            } => {
                actions.extend(Piece::iter().map(|piece| RenderAction::DrawPiece {
                    position: *positions.get(piece),
                    piece,
                    is_active: piece == active_piece,
                }));
            }
        }

        actions
    }
}

impl BoardChanged<'_> {
    pub fn render_actions(self) -> ArrayVec<RenderAction, { 2 * variant_count::<Piece>() + 2 }> {
        let mut actions = ArrayVec::new();

        if let Some(pc) = self.pieces_changed {
            actions.extend(pc.render_actions().into_iter());
        }

        if let Some(n) = self.num_moves_changed {
            actions.push(RenderAction::UpdateNumMoves(n));
        }

        let mut alert = None;
        if let Some(rating) = self.winning_rating {
            alert = Some(Alert::Win(rating));
        } else {
            if self.at_max_moves {
                alert = Some(Alert::MaxMoves);
            }
        }

        actions.push(RenderAction::Alert(alert.unwrap_or(Alert::Clear)));

        actions
    }
}
