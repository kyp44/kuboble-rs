use std::fs::File;

use derive_new::new;
use easycurses::{
    preserve_panic_message, Color, ColorPair, CursorVisibility, EasyCurses, Input, InputMode,
};
use kuboble_core::{
    level_run::Direction,
    level_select::{Action, LevelProgress, LevelSelector},
    LevelRating, Piece, Vector,
};
use level_run::play_level;
use level_select::select_level;

mod level_run;
mod level_select;

mod colors {
    use easycurses::{Color, ColorPair};

    pub const BACKGROUND: Color = Color::Black;
    pub const MAIN: Color = Color::White;
    pub const ALERT: Color = Color::Red;

    pub const STAR_INACTIVE: Color = Color::Blue;
    pub const STAR_ACTIVE: Color = Color::Yellow;

    pub const SELECTED_BACKGROUND: Color = Color::White;
    pub const SELECTED_MAIN: Color = Color::Black;

    pub const PIECE_GREEN: Color = Color::Green;
    pub const PIECE_ORANGE: Color = Color::Red;
    pub const PIECE_BLUE: Color = Color::Blue;

    pub const WIN_NOTIFICATION: Color = Color::Yellow;

    #[inline]
    pub fn basic(color: Color) -> ColorPair {
        ColorPair::new(color, BACKGROUND)
    }

    #[inline]
    pub fn selected(color: Color) -> ColorPair {
        ColorPair::new(color, SELECTED_BACKGROUND)
    }
}

trait PieceExt {
    fn to_color(&self) -> Color;
}
impl PieceExt for Piece {
    fn to_color(&self) -> Color {
        match self {
            Piece::Green => colors::PIECE_GREEN,
            Piece::Orange => colors::PIECE_ORANGE,
            Piece::Blue => colors::PIECE_BLUE,
        }
    }
}

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
    // Draws at the current cursor location does not set the color again after
    fn draw_stars(&mut self, num: u8, den: u8, background: Color) -> Option<()>;
    fn draw_rating(&mut self, level_rating: LevelRating, background: Color) -> Option<()>;
    fn wait_for_key(&mut self) -> ControlAction;
}
impl CursesExt for EasyCurses {
    fn clear_row(&mut self, row: i32) -> Option<()> {
        let size = self.get_row_col_count();

        // Paint the background color over the row
        self.set_color_pair(colors::basic(colors::BACKGROUND));
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
        self.set_color_pair(colors::basic(color));
        self.print_char(character)
    }

    fn print_on_row<S: AsRef<str>>(&mut self, row: i32, color: Color, msg: S) -> Option<()> {
        self.clear_row(row)?;
        self.move_rc(row, 0)?;
        self.set_color_pair(colors::basic(color));
        self.print(msg)
    }

    fn draw_stars(&mut self, num: u8, den: u8, background: Color) -> Option<()> {
        // Draw active stars
        self.set_color_pair(ColorPair::new(colors::STAR_ACTIVE, background));
        for _ in 0..num {
            self.print("*")?;
        }

        // Draw inactive stars
        self.set_color_pair(ColorPair::new(colors::STAR_INACTIVE, background));
        for _ in 0..(den - num) {
            self.print("*")?;
        }

        Some(())
    }

    fn draw_rating(&mut self, level_rating: LevelRating, background: Color) -> Option<()> {
        self.draw_stars(
            level_rating.num_stars(),
            LevelRating::maximum_possible().num_stars(),
            background,
        )
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

const PROGRESS_FILE_NAME: &str = "level-progress.json";

#[derive(new)]
struct Game {
    level_progress: LevelProgress,
}
impl Game {
    pub fn run(mut self) -> LevelProgress {
        preserve_panic_message(move |curses| {
            // Setup curses
            curses
                .set_cursor_visibility(CursorVisibility::Invisible)
                .unwrap();
            curses.set_input_mode(InputMode::Character).unwrap();
            curses.set_echo(false).unwrap();
            curses.set_keypad_enabled(true).unwrap();

            let mut level_selector = LevelSelector::new(&mut self.level_progress);

            loop {
                match select_level(curses, &mut level_selector) {
                    Some(level_info) => {
                        if let Some(status) = play_level(curses, &level_info) {
                            let _ =
                                level_selector.execute_action(Action::ActiveLevelCompleted(status));
                        }
                    }
                    None => break,
                }
            }
            drop(level_selector);

            self.level_progress
        })
        .unwrap()
    }
}

fn load_progress() -> Result<LevelProgress, anyhow::Error> {
    Ok(serde_json::from_reader(File::open(PROGRESS_FILE_NAME)?)?)
}

fn main() -> anyhow::Result<()> {
    //let level_progress = Game::new(load_progress()?).run();
    let level_progress =
        Game::new(load_progress().unwrap_or_else(|_| LevelProgress::default())).run();

    // Save out the level progress
    serde_json::to_writer(File::create(PROGRESS_FILE_NAME)?, &level_progress)?;

    Ok(())
}
