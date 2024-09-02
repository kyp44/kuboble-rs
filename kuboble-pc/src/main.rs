use std::borrow::Cow;

use derive_new::new;
use easycurses::{
    constants::acs, Color, ColorPair, CursorVisibility, EasyCurses, Input, InputMode,
};
use kuboble_core::{
    Action, Alert, Board, BoardRenderer, Direction, Level, Piece, Space, Vector, LEVELS,
};

const BACKGROUND_COLOR: Color = Color::Black;

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

trait CursesExt {
    fn clear_row(&mut self, row: i32) -> Option<()>;
    fn clear_screen(&mut self) -> Option<()>;
    fn put_char(&mut self, position: Vector<i32>, color: Color, character: u32) -> Option<()>;
    fn print_on_row<S: AsRef<str>>(&mut self, row: i32, color: Color, msg: S) -> Option<()>;
}
impl CursesExt for EasyCurses {
    fn clear_row(&mut self, row: i32) -> Option<()> {
        let size = self.get_row_col_count();

        // Paint the background color over the row
        self.set_color_pair(ColorPair::new(BACKGROUND_COLOR, BACKGROUND_COLOR));
        for col in 0..size.1 {
            self.move_rc(row as i32, col)?;
            self.print_char(' ')?;
        }
        Some(())
    }

    fn clear_screen(&mut self) -> Option<()> {
        // Clear the  screen
        self.clear()?;

        // Paint the background color
        let size = self.get_row_col_count();
        for row in 0..size.0 - 1 {
            self.clear_row(row)?;
        }

        Some(())
    }

    fn put_char(&mut self, position: Vector<i32>, color: Color, character: u32) -> Option<()> {
        self.move_rc(position.y, position.x)?;
        self.set_color_pair(ColorPair::new(color, BACKGROUND_COLOR));
        self.print_char(character)
    }

    // Row is
    fn print_on_row<S: AsRef<str>>(&mut self, row: i32, color: Color, msg: S) -> Option<()> {
        self.clear_row(row)?;
        self.move_rc(row, 0)?;
        self.set_color_pair(ColorPair::new(color, BACKGROUND_COLOR));
        self.print(msg)
    }
}

#[derive(new)]
struct CursesRenderer<'a> {
    curses: &'a mut EasyCurses,
    level: &'a Level,
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

    fn slide_piece(&mut self, piece_slid: kuboble_core::PieceSlid) {
        self.draw_space(
            piece_slid.starting_position,
            self.level.get_space(piece_slid.starting_position),
        );
        self.draw_piece(
            piece_slid.starting_position
                + piece_slid.muv.direction.as_vector() * piece_slid.distance.try_into().unwrap(),
            piece_slid.muv.piece,
            piece_slid.is_active,
        );
    }

    fn update_num_moves(&mut self, num_moves: u8) {
        self.draw_num_moves(num_moves, false);
    }

    fn update_goal(&mut self, goal: u8) {
        self.curses
            .print_on_row(self.row_num(2), Color::White, format!("Goal: {}", goal))
            .unwrap();
    }

    fn display_alert(&mut self, alert: Alert) {
        let (color, msg): (Color, Cow<str>) = match alert {
            Alert::Win(rating) => (
                Color::Yellow,
                format!("You win with {}/5 stars!", rating.rating()).into(),
            ),
            Alert::MaxMoves(n) => {
                self.draw_num_moves(n, true);
                return;
            }
            Alert::Clear => (BACKGROUND_COLOR, "".into()),
        };

        self.curses
            .print_on_row(self.row_num(3), color, msg)
            .unwrap();
    }
}
impl CursesRenderer<'_> {
    // Converts board position to absolute position.
    fn board_position(&self, board_position: Vector<u8>) -> Vector<i32> {
        Vector::new(board_position.x as i32 + 1, board_position.y as i32 + 1)
    }

    // Converts HUD row to absolute row.
    fn row_num(&self, hud_row: u8) -> i32 {
        (self.level.size.y + 2 + hud_row) as i32
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

    pub fn wait_for_key(&mut self) -> Input {
        self.curses.get_input().unwrap()
    }
}

fn main() {
    // Setup curses
    let mut curses = EasyCurses::initialize_system().unwrap();
    curses
        .set_cursor_visibility(CursorVisibility::Invisible)
        .unwrap();
    curses.set_input_mode(InputMode::Character).unwrap();
    curses.set_echo(false).unwrap();
    curses.set_keypad_enabled(true).unwrap();

    curses.clear_screen().unwrap();

    let mut board = Board::from(&LEVELS[0]);
    let mut renderer = CursesRenderer::new(&mut curses, board.level());

    board.render(&mut renderer);

    loop {
        let action = match renderer.wait_for_key() {
            Input::Character('\u{1b}') | Input::Character('q') => {
                // Quit
                break;
            }
            Input::KeyUp => Action::Move(Direction::Up),
            Input::KeyDown => Action::Move(Direction::Down),
            Input::KeyLeft => Action::Move(Direction::Left),
            Input::KeyRight => Action::Move(Direction::Right),
            Input::Character('\t') | Input::Character(' ') => Action::ChangeActivePiece,
            Input::KeyBackspace => Action::UndoMove,
            Input::Character('r') => Action::Restart,
            _ => {
                continue;
            }
        };

        board.execute_action(action).render(&mut renderer);
    }
}
