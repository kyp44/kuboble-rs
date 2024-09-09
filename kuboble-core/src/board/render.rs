use super::{Board, BoardChanged, PieceSlid, PiecesChanged};
use crate::{level_select::LevelStatus, Piece, Space, Vector};

#[cfg(feature = "std")]
use super::VecExt;

pub trait BoardRenderer {
    fn draw_space(&mut self, board_position: Vector<u8>, space: Space);
    fn draw_piece(&mut self, board_position: Vector<u8>, piece: Piece, is_active: bool);
    fn slide_piece(&mut self, piece_slid: &PieceSlid, is_active: bool);
    fn update_num_moves(&mut self, num_moves: u8, at_maximum: bool);
    fn update_constants(&mut self, level_num: u16, goal: u8);
    fn notify_win(&mut self, level_status: LevelStatus);
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
        for piece in self.level().all_pieces() {
            renderer.draw_piece(
                self.board_state.piece_position(piece),
                piece,
                self.active_piece == piece,
            );
        }

        // Update metrics
        renderer.update_num_moves(self.num_moves(), self.move_stack.is_full());
        renderer.update_constants(self.level_num, level.optimal_moves);

        // Display alert if applicable
        if let Some(status) = self.winning_status() {
            renderer.notify_win(status);
        }
    }
}

impl PiecesChanged<'_> {
    pub fn render<R: BoardRenderer>(&self, renderer: &mut R) {
        match self {
            PiecesChanged::Slid {
                piece_slid,
                is_active,
                old_active_piece,
            } => {
                renderer.slide_piece(piece_slid, *is_active);
                if let Some(oap) = old_active_piece {
                    renderer.draw_piece(oap.position, oap.piece, false);
                }
            }
            PiecesChanged::Moved(moved) => {
                // First, erase the pieces at the old locations
                for moved in moved.iter() {
                    renderer.draw_space(moved.from, moved.from_space);
                }
                // Now draw pieces at the new locations
                for moved in moved.iter() {
                    renderer.draw_piece(moved.to, moved.piece, moved.is_active);
                }
            }
            PiecesChanged::ActivePiece {
                active_piece,
                positions,
            } => {
                for piece in positions.pieces() {
                    renderer.draw_piece(*positions.get(piece), piece, piece == *active_piece);
                }
            }
        }
    }
}

impl BoardChanged<'_> {
    pub fn render<R: BoardRenderer>(&self, renderer: &mut R) {
        if let Some(pc) = self.pieces_changed.clone() {
            pc.render(renderer);
        }

        if let Some(n) = self.num_moves_changed {
            renderer.update_num_moves(n, self.at_max_moves);
        }

        if let Some(status) = self.winning_status.clone() {
            renderer.notify_win(status);
        }
    }
}
