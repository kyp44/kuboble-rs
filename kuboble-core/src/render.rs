use strum::IntoEnumIterator;

use crate::{
    Board, BoardChanged, LevelRating, Piece, PieceMoved, PieceSlid, PiecesChanged, Space, Vector,
};

#[cfg(feature = "std")]
use crate::VecExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alert {
    // You win with the rating
    Win(LevelRating),
    MaxMoves(u8),
    Clear,
}

pub trait BoardRenderer {
    fn draw_space(&mut self, board_position: Vector<u8>, space: Space);
    fn draw_piece(&mut self, board_position: Vector<u8>, piece: Piece, is_active: bool);
    fn slide_piece(&mut self, piece_slid: PieceSlid);
    fn update_num_moves(&mut self, num_moves: u8);
    fn update_goal(&mut self, goal: u8);
    fn display_alert(&mut self, alert: Alert);
}

impl Board<'_> {
    // For drawing the entire current board
    pub fn render<R: BoardRenderer>(&self, renderer: &mut R) {
        let level = self.board_state.level;

        // Render level spaces
        for position in self.board_state.level.all_positions() {
            renderer.draw_space(position, level.get_space(position));
        }

        // Render pieces
        for piece in Piece::iter() {
            renderer.draw_piece(
                *self.board_state.positions.get(piece),
                piece,
                self.active_piece == piece,
            );
        }

        // Update metrics
        renderer.update_num_moves(self.num_moves());
        renderer.update_goal(level.optimal_moves);

        // Display alert if applicable
        renderer.display_alert(if let Some(rating) = self.winning_rating() {
            Alert::Win(rating)
        } else {
            if self.move_stack.is_full() {
                Alert::MaxMoves(self.num_moves())
            } else {
                Alert::Clear
            }
        });
    }
}

impl PieceMoved {
    pub fn render<R: BoardRenderer>(&self, renderer: &mut R) {
        renderer.draw_space(self.from, self.from_space);
        renderer.draw_piece(self.to, self.piece, self.is_active);
    }
}

impl PiecesChanged<'_> {
    pub fn render<R: BoardRenderer>(self, renderer: &mut R) {
        match self {
            PiecesChanged::Slid {
                piece_slid,
                old_active_piece,
            } => {
                renderer.slide_piece(piece_slid);
                if let Some(oap) = old_active_piece {
                    renderer.draw_piece(oap.position, oap.piece, false);
                }
            }
            PiecesChanged::Moved(moved) => {
                for piece_moved in moved {
                    piece_moved.render(renderer);
                }
            }
            PiecesChanged::ActivePiece {
                active_piece,
                positions,
            } => {
                for piece in Piece::iter() {
                    renderer.draw_piece(*positions.get(piece), piece, piece == active_piece);
                }
            }
        }
    }
}

impl BoardChanged<'_> {
    pub fn render<R: BoardRenderer>(self, renderer: &mut R) {
        if let Some(pc) = self.pieces_changed {
            pc.render(renderer);
        }

        if let Some(n) = self.num_moves_changed {
            renderer.update_num_moves(n);
        }

        let mut alert = None;
        if let Some(rating) = self.winning_rating {
            alert = Some(Alert::Win(rating));
        } else {
            if let Some(n) = self.at_max_moves {
                alert = Some(Alert::MaxMoves(n));
            }
        }

        renderer.display_alert(if let Some(a) = alert { a } else { Alert::Clear });
    }
}
