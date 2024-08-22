use std::borrow::Cow;

use derive_new::new;
use easycurses::{
    constants::acs, Color, ColorPair, CursorVisibility, EasyCurses, Input, InputMode,
};
use kuboble_core::{
    Action, Alert, Board, Direction, Level, Piece, PieceMoved, RenderAction, Space, Vector, LEVELS,
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
struct BoardRenderer<'a> {
    curses: &'a mut EasyCurses,
    level: &'a Level,
}
impl BoardRenderer<'_> {
    // Converts board position to absolute position.
    fn board_position(&self, board_position: Vector<u8>) -> Vector<i32> {
        Vector::new(board_position.x as i32 + 1, board_position.y as i32 + 1)
    }

    // Converts HUD row to absolute row.
    fn row_num(&self, hud_row: u8) -> i32 {
        (self.level.size.y + 2 + hud_row) as i32
    }

    fn render_space(&mut self, board_position: Vector<u8>, space: Space) {
        let (color, c) = match space {
            Space::Wall => (Color::White, '#'),
            Space::Goal(piece) => (piece.to_color(), '#'),
            _ => (Color::White, ' '),
        };

        self.curses
            .put_char(self.board_position(board_position), color, c.into())
            .unwrap()
    }

    fn render_piece(&mut self, board_position: Vector<u8>, piece: Piece, is_active: bool) {
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

    fn update_active_piece(&mut self, piece: Piece) {
        self.curses
            .print_on_row(self.row_num(0), piece.to_color(), "Active: ")
            .unwrap();
        self.curses.print_char(acs::diamond()).unwrap();
    }

    fn update_num_moves(&mut self, num_moves: u8) {
        self.curses
            .print_on_row(
                self.row_num(1),
                Color::White,
                format!("Moves: {}", num_moves),
            )
            .unwrap();
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
                format!("You win with {} stars!", rating.rating()).into(),
            ),
            Alert::MaxMoves => (Color::Red, "At the maximum number of moves!".into()),
        };

        self.curses
            .print_on_row(self.row_num(3), color, msg)
            .unwrap();
    }

    pub fn execute_actions(&mut self, actions: impl Iterator<Item = RenderAction>) {
        for action in actions {
            match action {
                RenderAction::DrawSpace { position, space } => self.render_space(position, space),
                RenderAction::DrawPiece {
                    position,
                    piece,
                    is_active,
                } => self.render_piece(position, piece, is_active),
                RenderAction::SlidePiece(slid) => {
                    self.render_space(
                        slid.starting_position,
                        self.level.get_space(slid.starting_position),
                    );
                    self.render_piece(
                        slid.starting_position
                            + slid.muv.direction.as_vector() * slid.distance.try_into().unwrap(),
                        slid.muv.piece,
                        slid.is_active,
                    );
                }
                RenderAction::UpdateNumMoves(n) => self.update_num_moves(n),
                RenderAction::UpdateGoal(n) => self.update_goal(n),
                RenderAction::Alert(a) => self.display_alert(a),
            }
        }
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
    let mut renderer = BoardRenderer::new(&mut curses, board.level());

    renderer.execute_actions(board.render_actions());

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

        renderer.execute_actions(board.execute_action(action).render_actions().into_iter());
    }
}
