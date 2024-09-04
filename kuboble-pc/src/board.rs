use crate::{ControlAction, CursesExt};
use derive_new::new;
use easycurses::{
    constants::acs::{self, pi},
    Color, EasyCurses, Input,
};
use kuboble_core::{
    board::{render::BoardRenderer, Action, Board, Direction, PieceSlid},
    level_select::LevelInfo,
    Level, LevelRating, Piece, Space, Vector,
};

trait PieceExt {
    fn to_color(&self) -> Color;
}
impl PieceExt for Piece {
    fn to_color(&self) -> Color {
        match self {
            Piece::Green => Color::Green,
            Piece::Orange => Color::Red,
        }
    }
}

struct CursesRenderer<'a> {
    curses: &'a mut EasyCurses,
    level_height: u8,
}
impl<'a> CursesRenderer<'a> {
    pub fn new(curses: &'a mut EasyCurses, level: &Level) -> Self {
        curses.clear_screen();

        Self {
            curses,
            level_height: level.size.y,
        }
    }

    // Converts board position to absolute position.
    fn board_position(&self, board_position: Vector<u8>) -> Vector<i32> {
        Vector::new(board_position.x as i32, board_position.y as i32 + 2)
    }

    // Converts HUD row to absolute row.
    fn row_num(&self, hud_row: u8) -> i32 {
        (self.level_height + 3 + hud_row) as i32
    }

    fn update_active_piece(&mut self, piece: Piece) {
        self.curses
            .print_on_row(self.row_num(0), piece.to_color(), "Active: ")
            .unwrap();
        self.curses.print_char(acs::diamond()).unwrap();
    }

    fn draw_num_moves(&mut self, num_moves: u8, alert: bool) {
        self.curses
            .print_on_row(
                self.row_num(1),
                if alert { Color::Red } else { Color::White },
                format!("Moves: {}", num_moves),
            )
            .unwrap();
    }

    pub fn wait_for_key(&mut self) -> ControlAction {
        self.curses.wait_for_key()
    }
}
impl BoardRenderer for CursesRenderer<'_> {
    fn draw_space(&mut self, board_position: Vector<u8>, space: Space) {
        let (color, c) = match space {
            Space::Wall => (Color::White, '#'),
            Space::Goal(piece) => (piece.to_color(), '#'),
            _ => (Color::White, ' '),
        };

        self.curses
            .put_char(self.board_position(board_position), color, c.into())
            .unwrap()
    }

    fn draw_piece(&mut self, board_position: Vector<u8>, piece: Piece, is_active: bool) {
        self.curses
            .put_char(
                self.board_position(board_position),
                piece.to_color(),
                acs::diamond(),
            )
            .unwrap();

        if is_active {
            self.update_active_piece(piece);
        }
    }

    fn slide_piece(&mut self, piece_slid: &PieceSlid, is_active: bool) {
        self.draw_space(piece_slid.starting_position, piece_slid.starting_space);
        self.draw_piece(
            piece_slid.starting_position
                + piece_slid.muv.direction.as_vector() * piece_slid.distance.try_into().unwrap(),
            piece_slid.muv.piece,
            is_active,
        );
    }

    fn update_num_moves(&mut self, num_moves: u8, at_maximum: bool) {
        self.draw_num_moves(num_moves, at_maximum);
    }

    fn update_constants(&mut self, level_num: u16, goal: u8) {
        self.curses
            .print_on_row(0, Color::White, format!("Level {level_num}"))
            .unwrap();

        self.curses
            .print_on_row(self.row_num(2), Color::White, format!("Goal: {}", goal))
            .unwrap();
    }

    fn notify_win(&mut self, rating: kuboble_core::LevelRating) {
        self.curses
            .print_on_row(
                self.row_num(3),
                Color::Yellow,
                format!("You win with {}/5 stars!", rating.num_stars()),
            )
            .unwrap();
    }
}

pub fn play_board(curses: &mut EasyCurses, level_info: &LevelInfo) -> Option<LevelRating> {
    let mut board = Board::new(&level_info);
    let mut renderer = CursesRenderer::new(curses, level_info.level);

    board.render(&mut renderer);

    loop {
        let action = match renderer.wait_for_key() {
            ControlAction::Escape => break None,
            ControlAction::Arrow(dir) => Action::Move(dir),
            ControlAction::Tab | ControlAction::Proceed => Action::ChangeActivePiece,
            ControlAction::Backspace => Action::UndoMove,
            ControlAction::Restart => Action::Restart,
            _ => {
                continue;
            }
        };

        // TODO: Need to check if we win!
        let board_changed = board.execute_action(action);

        board_changed.render(&mut renderer);

        if board_changed.winning_rating.is_some() {
            // Just wait for the enter key then return
            loop {
                if renderer.wait_for_key() == ControlAction::Proceed {
                    break;
                }
            }

            break board_changed.winning_rating;
        }
    }
}
