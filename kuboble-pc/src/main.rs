use board::play_board;
use easycurses::{
    preserve_panic_message, Color, ColorPair, CursorVisibility, EasyCurses, Input, InputMode,
};
use kuboble_core::{
    board::Direction,
    level_select::{LevelProgress, LevelSelector, LevelStatus},
    Vector,
};
use level_select::select_level;

mod board;
mod level_select;

const BACKGROUND_COLOR: Color = Color::Black;

#[derive(PartialEq, Eq)]
enum ControlAction {
    Escape,
    Arrow(Direction),
    Proceed,
    Tab,
    Backspace,
    Restart,
}

trait CursesExt {
    fn clear_row(&mut self, row: i32) -> Option<()>;
    fn clear_screen(&mut self) -> Option<()>;
    fn put_char(&mut self, position: Vector<i32>, color: Color, character: u32) -> Option<()>;
    fn print_on_row<S: AsRef<str>>(&mut self, row: i32, color: Color, msg: S) -> Option<()>;
    fn wait_for_key(&mut self) -> ControlAction;
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

    fn wait_for_key(&mut self) -> ControlAction {
        loop {
            break match self.get_input().unwrap() {
                Input::Character('\u{1b}') | Input::Character('q') | Input::Character('Q') => {
                    ControlAction::Escape
                }
                Input::KeyUp => ControlAction::Arrow(Direction::Up),
                Input::KeyDown => ControlAction::Arrow(Direction::Down),
                Input::KeyLeft => ControlAction::Arrow(Direction::Left),
                Input::KeyRight => ControlAction::Arrow(Direction::Right),
                Input::Character('\n') | Input::Character(' ') => ControlAction::Proceed,
                Input::Character('\t') => ControlAction::Tab,
                Input::KeyBackspace => ControlAction::Backspace,
                Input::Character('r') | Input::Character('R') => ControlAction::Restart,
                _ => {
                    continue;
                }
            };
        }
    }
}

fn run_game(curses: &mut EasyCurses) {
    // Setup curses
    curses
        .set_cursor_visibility(CursorVisibility::Invisible)
        .unwrap();
    curses.set_input_mode(InputMode::Character).unwrap();
    curses.set_echo(false).unwrap();
    curses.set_keypad_enabled(true).unwrap();

    curses.clear_screen().unwrap();

    let mut level_progress = LevelProgress::default();
    let mut level_selector = LevelSelector::new(&mut level_progress, 10);

    loop {
        match select_level(curses, &mut level_selector) {
            Some(level_info) => {
                if let Some(rating) = play_board(curses, &level_info) {
                    // TODO: Need real status, especially in case of optimal solution
                    //level_progress.update_status(level_info.index, LevelStatus::Complete(rating))
                }
            }
            None => break,
        }
    }
}

fn main() {
    preserve_panic_message(run_game).unwrap();
}
