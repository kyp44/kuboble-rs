use crate::{ControlAction, CursesExt};
use easycurses::{Color, EasyCurses, Input};
use kuboble_core::{
    board::Direction as KeyDirection,
    level_select::{
        render::{self, LevelSelectRenderer},
        Action, Direction, LevelInfo, LevelSelector, LevelSlotInfo,
    },
    Level, LevelRating,
};

struct CursesRenderer<'a> {
    curses: &'a mut EasyCurses,
}
impl<'a> CursesRenderer<'a> {
    pub fn new(curses: &'a mut EasyCurses) -> Self {
        curses.clear_screen();

        Self { curses }
    }

    pub fn wait_for_key(&mut self) -> ControlAction {
        self.curses.wait_for_key()
    }
}
impl LevelSelectRenderer for CursesRenderer<'_> {
    fn draw_level_slot(&mut self, level_slot_info: &LevelSlotInfo) {
        let level_info = &level_slot_info.level_info;
        let user_size = level_info.level.user_size();

        self.curses
            .print_on_row(
                level_slot_info.position as i32 + 2,
                if level_slot_info.is_active {
                    Color::Green
                } else {
                    Color::White
                },
                format!(
                    "Level {:<8}{}/{:<8}{}x{}",
                    level_info.user_num(),
                    level_info.rating.num_stars(),
                    LevelRating::maximum_possible().num_stars(),
                    user_size.x,
                    user_size.y,
                ),
            )
            .unwrap()
    }

    fn update_filter(&mut self, filter: kuboble_core::level_select::Filter, is_active: bool) {
        // TODO
    }
}

pub fn select_level(
    curses: &mut EasyCurses,
    level_selector: &mut LevelSelector,
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
            ControlAction::Proceed => return Some(level_selector.selected_level()),
            _ => {
                continue;
            }
        };

        if let Some(changed) = level_selector.execute_action(action) {
            changed.render(&mut renderer);
        }
    }
}
