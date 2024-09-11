use crate::{
    colors::{self, SELECTED_MAIN},
    ControlAction, CursesExt,
};
use easycurses::{ColorPair, EasyCurses};
use kuboble_core::{
    level_run::Direction as KeyDirection,
    level_select::{
        render::LevelSelectRenderer, Action, Direction, Filter, LevelInfo, LevelSelector,
        LevelSlotInfo,
    },
};

struct CursesRenderer<'a> {
    curses: &'a mut EasyCurses,
}
impl<'a> CursesRenderer<'a> {
    pub fn new(curses: &'a mut EasyCurses) -> Self {
        curses.clear_screen();
        curses.print_on_row(0, colors::MAIN, "Filter:");

        Self { curses }
    }

    fn level_row(slot_position: u8) -> i32 {
        slot_position as i32 + 2
    }

    pub fn wait_for_key(&mut self) -> ControlAction {
        self.curses.wait_for_key()
    }
}
impl LevelSelectRenderer for CursesRenderer<'_> {
    fn draw_level_slot(&mut self, level_slot_info: &LevelSlotInfo) {
        match level_slot_info {
            LevelSlotInfo::Empty(pos) => self.curses.clear_row(Self::level_row(*pos)).unwrap(),
            LevelSlotInfo::Level {
                level_info,
                position,
                is_active,
            } => {
                // NOTE: Evidently we cannot just get the background color from a pair, which is annoying
                let background = if *is_active {
                    colors::SELECTED_BACKGROUND
                } else {
                    colors::BACKGROUND
                };
                let main_pair = if *is_active {
                    colors::selected(SELECTED_MAIN)
                } else {
                    colors::basic(colors::MAIN)
                };

                // Print level number
                self.curses.move_rc(Self::level_row(*position), 0).unwrap();
                self.curses.set_color_pair(main_pair);
                self.curses
                    .print(format!("Level {:<8}", level_info.user_num(),))
                    .unwrap();

                // Print rating
                self.curses
                    .draw_rating(level_info.rating, background)
                    .unwrap();

                // Print user size
                let user_size = level_info.level.user_size();
                self.curses.set_color_pair(main_pair);
                self.curses
                    .print(format!("{:>8}x{}", user_size.x, user_size.y))
                    .unwrap();
            }
        }
    }

    fn update_filter(&mut self, filter: Filter, is_active: bool) {
        let background = if is_active {
            colors::MAIN
        } else {
            colors::BACKGROUND
        };

        const START_COL: i32 = 8;
        const SPACE_SIZE: i32 = 1;

        match filter {
            Filter::All => {
                self.curses.move_rc(0, START_COL).unwrap();
                self.curses
                    .set_color_pair(ColorPair::new(colors::STAR_ACTIVE, background));
                self.curses.print("All").unwrap();
            }
            Filter::Incomplete => {
                self.curses.move_rc(0, START_COL + 3 + SPACE_SIZE).unwrap();
                self.curses.draw_stars(0, 2, background).unwrap();
            }
            Filter::PartiallyComplete => {
                self.curses
                    .move_rc(0, START_COL + 5 + 2 * SPACE_SIZE)
                    .unwrap();
                self.curses.draw_stars(1, 2, background).unwrap();
            }
            Filter::Optimal => {
                self.curses
                    .move_rc(0, START_COL + 7 + 3 * SPACE_SIZE)
                    .unwrap();
                self.curses.draw_stars(2, 2, background).unwrap();
            }
        }
    }
}

const LEVEL_WINDOW_SIZE: usize = 10;

pub fn select_level(
    curses: &mut EasyCurses,
    level_selector: &mut LevelSelector<LEVEL_WINDOW_SIZE>,
) -> Option<LevelInfo> {
    let mut renderer = CursesRenderer::new(curses);
    level_selector.render(&mut renderer);

    loop {
        let action = match renderer.wait_for_key() {
            ControlAction::Escape => break None,
            ControlAction::Arrow(dir) => match dir {
                KeyDirection::Up => Action::ChangeActiveLevel(Direction::Previous),
                KeyDirection::Down => Action::ChangeActiveLevel(Direction::Next),
                KeyDirection::Left => Action::ChangeActiveFilter(Direction::Previous),
                KeyDirection::Right => Action::ChangeActiveFilter(Direction::Next),
            },
            ControlAction::Proceed => match level_selector.active_level_info() {
                Some(level_info) => return Some(level_info),
                None => continue,
            },
            _ => {
                continue;
            }
        };

        if let Some(changed) = level_selector.execute_action(action) {
            changed.render(&mut renderer);
        }
    }
}
