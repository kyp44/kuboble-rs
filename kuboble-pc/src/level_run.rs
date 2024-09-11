use crate::{colors, ControlAction, CursesExt, PieceExt};
use easycurses::{constants::acs, EasyCurses};
use kuboble_core::{
    level_run::{render::LevelRunRenderer, Action, LevelRun, PieceSlid},
    level_select::{LevelInfo, LevelStatus},
    Level, Piece, Space, Vector,
};

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

    // Converts level position to absolute position.
    fn absolute_position(&self, level_position: Vector<u8>) -> Vector<i32> {
        Vector::new(level_position.x as i32, level_position.y as i32 + 2)
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
                if alert { colors::ALERT } else { colors::MAIN },
                format!("Moves: {}", num_moves),
            )
            .unwrap();
    }

    pub fn wait_for_key(&mut self) -> ControlAction {
        self.curses.wait_for_key()
    }
}
impl LevelRunRenderer for CursesRenderer<'_> {
    fn draw_space(&mut self, position: Vector<u8>, space: Space) {
        let (color, c) = match space {
            Space::Wall => (colors::MAIN, '#'),
            Space::Goal(piece) => (piece.to_color(), '#'),
            _ => (colors::MAIN, ' '),
        };

        self.curses
            .put_char(self.absolute_position(position), color, c.into())
            .unwrap()
    }

    fn draw_piece(&mut self, position: Vector<u8>, piece: Piece, is_active: bool) {
        self.curses
            .put_char(
                self.absolute_position(position),
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
            .print_on_row(0, colors::MAIN, format!("Level {level_num}"))
            .unwrap();

        self.curses
            .print_on_row(self.row_num(2), colors::MAIN, format!("Goal: {}", goal))
            .unwrap();
    }

    fn notify_win(&mut self, level_status: LevelStatus) {
        self.curses
            .print_on_row(
                self.row_num(3),
                colors::WIN_NOTIFICATION,
                "You win with a rating of: ",
            )
            .unwrap();

        self.curses
            .draw_rating(level_status.rating(), colors::BACKGROUND)
            .unwrap();
    }
}

pub fn play_level(curses: &mut EasyCurses, level_info: &LevelInfo) -> Option<LevelStatus> {
    let mut level_run = LevelRun::new(&level_info);
    let mut renderer = CursesRenderer::new(curses, level_info.level);

    level_run.render(&mut renderer);

    loop {
        let action = match renderer.wait_for_key() {
            ControlAction::Escape => break None,
            ControlAction::Arrow(dir) => Action::Move(dir),
            ControlAction::Tab | ControlAction::Proceed => Action::ChangeActivePiece,
            ControlAction::Backspace => Action::UndoMove,
            ControlAction::Restart => Action::Restart,
        };

        let change = level_run.execute_action(action);
        change.render(&mut renderer);

        if change.winning_status.is_some() {
            loop {
                if renderer.wait_for_key() == ControlAction::Proceed {
                    break;
                }
            }

            break change.winning_status;
        }
    }
}
