use core::borrow;

use easycurses::{
    constants::acs, Color, ColorPair, CursorVisibility, EasyCurses, Input, InputMode,
};
use kuboble_core::{
    Board, BoardState, BoardStatus, Direction, Level, Move, MoveStatus, Piece, Space, Vector,
    LEVELS,
};
use strum::IntoEnumIterator;

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
    fn put_board_char(
        &mut self,
        board_position: Vector<u8>,
        color: Color,
        character: u32,
    ) -> Option<()>;
    fn render_space(&mut self, board_position: Vector<u8>, space: Space) -> Option<()>;
    fn render_piece(&mut self, board_position: Vector<u8>, piece: Piece) -> Option<()>;
    fn render_board(&mut self, board_state: &Board) -> Option<()>;
    fn update_move(&mut self, level: &Level, muv: &Move) -> Option<()>;
    fn print_on_row<S: AsRef<str>>(&mut self, row: i32, color: Color, msg: S) -> Option<()>;
    fn render_hud(&mut self, board: &Board) -> Option<()>;
}
impl CursesExt for EasyCurses {
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

    fn put_board_char(
        &mut self,
        board_position: Vector<u8>,
        color: Color,
        character: u32,
    ) -> Option<()> {
        self.move_rc(board_position.y as i32 + 1, board_position.x as i32 + 1)?;
        self.set_color_pair(ColorPair::new(color, BACKGROUND_COLOR));
        self.print_char(character)
    }

    fn clear_row(&mut self, row: i32) -> Option<()> {
        // Paint the background color
        let size = self.get_row_col_count();

        self.set_color_pair(ColorPair::new(BACKGROUND_COLOR, BACKGROUND_COLOR));
        for col in 0..size.1 {
            self.move_rc(row, col)?;
            self.print_char(' ')?;
        }
        Some(())
    }

    fn render_space(&mut self, board_position: Vector<u8>, space: Space) -> Option<()> {
        let (color, c) = match space {
            Space::Wall => (Color::White, '#'),
            Space::Goal(piece) => (piece.to_color(), '#'),
            _ => (Color::White, ' '),
        };

        self.put_board_char(board_position, color, c.into())
    }

    fn render_piece(&mut self, board_position: Vector<u8>, piece: Piece) -> Option<()> {
        self.put_board_char(board_position, piece.to_color(), acs::diamond())
    }

    fn render_board(&mut self, board: &Board) -> Option<()> {
        // Render level spaces
        for position in board.level.positions() {
            self.render_space(position, board.level.get_space(position))?;
        }

        // Render pieces
        for piece in Piece::iter() {
            self.render_piece(*board.current_board_state().positions.get(piece), piece)?;
        }

        // Render HUD
        self.render_hud(board)?;

        Some(())
    }

    fn update_move(&mut self, level: &Level, muv: &Move) -> Option<()> {
        self.render_space(muv.from, level.get_space(muv.from))?;
        self.render_piece(muv.to, muv.piece)
    }

    fn print_on_row<S: AsRef<str>>(&mut self, row: i32, color: Color, msg: S) -> Option<()> {
        self.clear_row(row)?;
        self.move_rc(row, 0)?;
        self.set_color_pair(ColorPair::new(color, BACKGROUND_COLOR));
        self.print(msg)
    }

    fn render_hud(&mut self, board: &Board) -> Option<()> {
        let board_status = board.board_status();
        let first_row = board.level.size.y as i32 + 2;

        self.print_on_row(
            first_row,
            Color::White,
            format!("Moves: {}", board_status.num_moves),
        )?;
        self.print_on_row(
            first_row + 1,
            Color::White,
            format!("Goal: {}", board.level.optimal_moves),
        )?;

        let mut row = first_row + 2;
        if board_status.winning_position {
            self.print_on_row(row, Color::Yellow, "You win!")?;
            row += 1;
        } else {
            self.clear_row(row)?;
        }
        if board_status.at_max_moves {
            self.print_on_row(row, Color::Red, "At maximum number of moves!")?;
        } else {
            self.clear_row(row)?;
        }

        Some(())
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
    curses.render_board(&board).unwrap();

    loop {
        let direction = match curses.get_input().unwrap() {
            Input::Character('\u{1b}') | Input::Character('q') => {
                break;
            }
            Input::KeyUp => Direction::Up,
            Input::KeyDown => Direction::Down,
            Input::KeyLeft => Direction::Left,
            Input::KeyRight => Direction::Right,
            Input::Character('\t') | Input::Character(' ') => {
                board.change_piece();
                continue;
            }
            Input::KeyBackspace => {
                if let Some(muv) = board.undo() {
                    curses.update_move(board.level, &muv).unwrap();
                    curses.render_hud(&board).unwrap();
                }
                continue;
            }
            Input::Character('r') => {
                for muv in board.restart() {
                    curses.update_move(board.level, &muv).unwrap()
                }
                curses.render_hud(&board).unwrap();
                continue;
            }
            _ => {
                continue;
            }
        };

        let update_hud = match board.make_move(direction) {
            MoveStatus::NoEffect => false,
            MoveStatus::MaxMoves => true,
            MoveStatus::MoveMade(muv) => {
                curses.update_move(board.level, &muv).unwrap();
                true
            }
        };

        if update_hud {
            curses.render_hud(&board).unwrap();
        }
    }
}
