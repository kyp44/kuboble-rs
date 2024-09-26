use crate::{
    level_run::Move,
    levels::{LEVELS, MAX_OPTIMAL_MOVES, NUM_LEVELS},
    Level, LevelRating,
};
use arrayvec::ArrayVec;
use core::{cmp::Ordering, iter::repeat, mem::discriminant};
use derive_new::new;
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub mod render;

#[derive(Clone, Debug, Default, Eq, Serialize, Deserialize)]
pub enum LevelStatus {
    #[default]
    Incomplete,
    Complete(LevelRating),
    Optimal(ArrayVec<Move, MAX_OPTIMAL_MOVES>),
}
impl LevelStatus {
    pub fn rating(&self) -> LevelRating {
        match self {
            LevelStatus::Incomplete => LevelRating::default(),
            LevelStatus::Complete(r) => *r,
            LevelStatus::Optimal(_) => LevelRating::maximum_possible(),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            LevelStatus::Incomplete => false,
            LevelStatus::Complete(_) => true,
            LevelStatus::Optimal(_) => true,
        }
    }
}
impl PartialEq for LevelStatus {
    fn eq(&self, other: &Self) -> bool {
        if let (Self::Complete(rl), Self::Complete(rr)) = (self, other) {
            rl == rr
        } else {
            discriminant(self) == discriminant(other)
        }
    }
}
impl PartialOrd for LevelStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for LevelStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            LevelStatus::Incomplete => match other {
                LevelStatus::Incomplete => Ordering::Equal,
                _ => Ordering::Less,
            },
            LevelStatus::Complete(rl) => match other {
                LevelStatus::Incomplete => Ordering::Greater,
                LevelStatus::Complete(rr) => rl.cmp(rr),
                LevelStatus::Optimal(_) => Ordering::Less,
            },
            LevelStatus::Optimal(_) => match other {
                LevelStatus::Optimal(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
        }
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, EnumIter, Enum)]
pub enum Filter {
    #[default]
    All,
    Incomplete,
    PartiallyComplete,
    Optimal,
}
impl Filter {
    pub fn next(&self) -> Self {
        match self {
            Filter::All => Self::Incomplete,
            Filter::Incomplete => Self::PartiallyComplete,
            Filter::PartiallyComplete => Self::Optimal,
            Filter::Optimal => Self::All,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Filter::All => Self::Optimal,
            Filter::Incomplete => Self::All,
            Filter::PartiallyComplete => Self::Incomplete,
            Filter::Optimal => Self::PartiallyComplete,
        }
    }

    pub fn passes(&self, level_status: &LevelStatus) -> bool {
        match self {
            Filter::All => true,
            Filter::Incomplete => !level_status.is_complete(),
            Filter::PartiallyComplete => {
                level_status.is_complete() && !level_status.rating().is_optimal()
            }
            Filter::Optimal => level_status.rating().is_optimal(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LevelInfo {
    pub index: usize,
    pub rating: LevelRating,
    pub level: &'static Level,
}
impl LevelInfo {
    pub fn user_num(&self) -> u16 {
        self.index as u16 + 1
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct LevelProgress {
    level_statuses: ArrayVec<LevelStatus, NUM_LEVELS>,
}
impl LevelProgress {
    pub fn level_info(&self, level_idx: usize) -> LevelInfo {
        let level = &LEVELS[level_idx];

        LevelInfo {
            index: level_idx,
            rating: self
                .level_statuses
                .get(level_idx)
                .map(|s| s.rating())
                .unwrap_or_default(),
            level,
        }
    }

    #[cfg(not(feature = "unlocked"))]
    pub fn num_unlocked_levels(&self) -> usize {
        NUM_LEVELS.min(
            self.level_statuses
                .iter()
                .filter(|l| l.is_complete())
                .count()
                + 10,
        )
    }

    #[cfg(feature = "unlocked")]
    pub fn num_unlocked_levels(&self) -> usize {
        NUM_LEVELS
    }

    pub fn num_locked_levels(&self) -> usize {
        NUM_LEVELS - self.num_unlocked_levels()
    }

    // Only updates the status if it is better and returns whether it was updated
    pub fn attempt_status_update(&mut self, level_idx: usize, new_status: LevelStatus) -> bool {
        if level_idx >= self.level_statuses.len() {
            // Need to add elements so we can change this one
            self.level_statuses.extend(
                repeat(LevelStatus::default()).take(level_idx - self.level_statuses.len() + 1),
            )
        }

        // Now this element should exist
        if new_status > self.level_statuses[level_idx] {
            self.level_statuses[level_idx] = new_status;
            true
        } else {
            false
        }
    }

    pub fn filtered_indices(&self, filter: Filter) -> impl Iterator<Item = usize> + '_ {
        static DEFAULT_STATUS: LevelStatus = LevelStatus::Incomplete;

        // We need to fill in any at the end with the default status
        self.level_statuses
            .iter()
            .chain(repeat(&DEFAULT_STATUS))
            .take(self.num_unlocked_levels())
            .enumerate()
            .filter_map(move |(idx, ls)| filter.passes(ls).then_some(idx))
    }
}

#[derive(new, Debug, Default, Clone, PartialEq, Eq)]
struct WindowPosition {
    pub top_idx: usize,
    pub cursor_idx: usize,
}
impl WindowPosition {
    pub fn cursor_window_idx(&self) -> usize {
        self.cursor_idx - self.top_idx
    }
}

#[derive(Clone)]
struct WindowItem<T> {
    is_cursor: bool,
    item: T,
}

#[derive(new, Debug, Default, PartialEq, Eq)]
enum WindowChange {
    #[default]
    None,
    Window,
    CursorOnly,
}
impl WindowChange {
    pub fn any_change(&self) -> bool {
        match self {
            WindowChange::None => false,
            _ => true,
        }
    }

    pub fn compare(old: &WindowPosition, new: &WindowPosition) -> Self {
        if old.top_idx != new.top_idx {
            Self::Window
        } else if old.cursor_idx != new.cursor_idx {
            Self::CursorOnly
        } else {
            Self::None
        }
    }
}

// Evidently a crate contain something like this just does not exist
#[derive(Default)]
struct WindowVec<T, const C: usize, const W: usize> {
    vec: ArrayVec<T, C>,
    position: WindowPosition,
}
impl<T, const C: usize, const W: usize> FromIterator<T> for WindowVec<T, C, W> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            vec: ArrayVec::from_iter(iter),
            position: WindowPosition::default(),
        }
    }
}
impl<T, const C: usize, const W: usize> WindowVec<T, C, W> {
    // Clear and refill from iterator, resetting the window
    pub fn refill<I: IntoIterator<Item = T>>(&mut self, iter: I, position: &WindowPosition) {
        self.vec.clear();
        self.vec.extend(iter);

        self.set_position(position);
    }

    pub fn position(&self) -> Option<&WindowPosition> {
        (!self.vec.is_empty()).then_some(&self.position)
    }

    // Window takes priority over cursor if they conflict
    pub fn set_position(&mut self, position: &WindowPosition) -> WindowChange {
        if self.vec.is_empty() {
            WindowChange::default()
        } else {
            let new_position = self.clip_position(position);

            let change = WindowChange::compare(&self.position, &new_position);

            if change.any_change() {
                self.position = new_position;
            }

            change
        }
    }

    fn clip_position(&self, position: &WindowPosition) -> WindowPosition {
        let top_idx = position.top_idx.min(self.vec.len().saturating_sub(W));

        WindowPosition::new(
            top_idx,
            position
                .cursor_idx
                .max(top_idx)
                .min(top_idx + W - 1)
                .min(self.vec.len() - 1),
        )
    }

    pub fn cursor_item(&self) -> Option<&T> {
        self.position().map(|p| &self.vec[p.cursor_idx])
    }

    pub fn move_cursor(&mut self, direction: Direction) -> WindowChange {
        match self.position() {
            Some(position) => {
                // Set the ideal new cursor index
                let new_cursor_idx = match direction {
                    Direction::Previous => position.cursor_idx.saturating_sub(1),
                    Direction::Next => (position.cursor_idx + 1).min(self.vec.len() - 1),
                };

                // Do we need to shift the window?
                let new_top_idx = if new_cursor_idx < position.top_idx {
                    new_cursor_idx
                } else if new_cursor_idx > position.top_idx + W - 1 {
                    new_cursor_idx.saturating_sub(W - 1)
                } else {
                    position.top_idx
                };

                self.set_position(&WindowPosition::new(new_top_idx, new_cursor_idx))
            }
            None => WindowChange::default(),
        }
    }

    pub fn page_window(&mut self, direction: Direction) -> WindowChange {
        match self.position() {
            Some(position) => {
                let new_position = match direction {
                    Direction::Previous => WindowPosition::new(
                        position.top_idx.saturating_sub(W),
                        position.cursor_idx.saturating_sub(W),
                    ),
                    Direction::Next => {
                        WindowPosition::new(position.top_idx + W, position.cursor_idx + W)
                    }
                };
                let mut new_position = self.clip_position(&new_position);

                let shift = new_position.top_idx.abs_diff(position.top_idx);
                if shift > 0 && shift < W {
                    // Only did a partial shift so shift the cursor by less than saturated W
                    new_position.cursor_idx = match direction {
                        Direction::Previous => position.cursor_idx - shift,
                        Direction::Next => position.cursor_idx + shift,
                    }
                }

                let change = WindowChange::compare(position, &new_position);

                if change.any_change() {
                    self.position = new_position;
                }

                change
            }
            None => WindowChange::default(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<WindowItem<&T>>> {
        self.vec[self.position().map(|p| p.top_idx).unwrap_or(0)..self.vec.len()]
            .into_iter()
            .enumerate()
            .map(|(idx, item)| {
                let idx = self.position.top_idx + idx;
                Some(WindowItem {
                    is_cursor: idx == self.position.cursor_idx,
                    item,
                })
            })
            .chain(repeat(None))
            .take(W)
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Previous,
    Next,
}

pub enum Action {
    ChangeActiveLevel(Direction),
    ChangePage(Direction),
    ChangeActiveFilter(Direction),
    ActiveLevelCompleted(LevelStatus),
}

#[derive(Debug)]
pub enum LevelSlotInfo {
    Empty(u8),
    Level {
        level_info: LevelInfo,
        position: u8,
        is_active: bool,
    },
}

pub struct FilterChange {
    inactive: Filter,
    active: Filter,
}

pub struct LevelSelectorChange<const W: usize> {
    slots_change: ArrayVec<LevelSlotInfo, W>,
    filter_change: Option<FilterChange>,
    num_locked_change: Option<u16>,
    active_rating: Option<LevelRating>,
}

pub struct LevelSelector<'a, const W: usize> {
    level_progress: &'a mut LevelProgress,
    active_filter: Filter,
    level_indices_window: WindowVec<u16, NUM_LEVELS, W>,
    window_positions: EnumMap<Filter, Option<WindowPosition>>,
}
impl<'a, const W: usize> LevelSelector<'a, W> {
    pub fn new(level_progress: &'a mut LevelProgress) -> Self {
        let level_indices_window = WindowVec::from_iter(
            level_progress
                .filtered_indices(Filter::default())
                .map(|i| i as u16),
        );

        let active_filter = Filter::default();
        let mut window_positions: EnumMap<_, Option<WindowPosition>> = EnumMap::default();

        window_positions[active_filter] = level_indices_window.position().cloned();

        Self {
            level_progress,
            active_filter,
            window_positions,
            level_indices_window,
        }
    }

    fn active_level_idx(&self) -> Option<usize> {
        self.level_indices_window
            .cursor_item()
            .map(|idx| *idx as usize)
    }

    pub fn active_level_info(&self) -> Option<LevelInfo> {
        self.active_level_idx()
            .map(|level_idx| self.level_progress.level_info(level_idx))
    }

    pub fn active_rating(&self) -> Option<LevelRating> {
        self.active_level_info().map(|l| l.rating)
    }

    pub fn current_slot(&self, is_active: bool) -> Option<LevelSlotInfo> {
        self.active_level_info()
            .map(|level_info| LevelSlotInfo::Level {
                level_info,
                position: self
                    .level_indices_window
                    .position()
                    .unwrap()
                    .cursor_window_idx() as u8,
                is_active,
            })
    }

    fn window_slots(&self) -> ArrayVec<LevelSlotInfo, W> {
        self.level_indices_window
            .iter()
            .enumerate()
            .map(|(posi, item)| {
                let pos = posi as u8;

                match item {
                    Some(item) => LevelSlotInfo::Level {
                        level_info: self.level_progress.level_info(*item.item as usize),
                        position: pos,
                        is_active: item.is_cursor,
                    },
                    None => LevelSlotInfo::Empty(pos),
                }
            })
            .collect()
    }

    fn rebuild_window(&mut self) {
        let window_position = &mut self.window_positions[self.active_filter];

        self.level_indices_window.refill(
            self.level_progress
                .filtered_indices(self.active_filter)
                .map(|i| i as u16),
            window_position
                .as_ref()
                .unwrap_or(&WindowPosition::default()),
        );

        *window_position = self.level_indices_window.position().cloned();
    }

    fn check_window_change(
        &mut self,
        old_active_slot: LevelSlotInfo,
        window_change: WindowChange,
    ) -> Option<LevelSelectorChange<W>> {
        match window_change {
            WindowChange::None => None,
            WindowChange::Window => {
                self.window_positions[self.active_filter] =
                    self.level_indices_window.position().cloned();

                Some(LevelSelectorChange {
                    slots_change: self.window_slots(),
                    filter_change: None,
                    num_locked_change: None,
                    active_rating: self.active_rating(),
                })
            }
            WindowChange::CursorOnly => {
                self.window_positions[self.active_filter] =
                    self.level_indices_window.position().cloned();

                Some(LevelSelectorChange {
                    slots_change: ArrayVec::from_iter([
                        old_active_slot,
                        self.current_slot(true).unwrap(),
                    ]),
                    filter_change: None,
                    num_locked_change: None,
                    active_rating: self.active_rating(),
                })
            }
        }
    }

    pub fn execute_action(&mut self, action: Action) -> Option<LevelSelectorChange<W>> {
        match action {
            Action::ChangeActiveLevel(dir) => self.current_slot(false).and_then(|old_slot| {
                let change = self.level_indices_window.move_cursor(dir);
                self.check_window_change(old_slot, change)
            }),
            Action::ChangePage(dir) => self.current_slot(false).and_then(|old_slot| {
                let change = self.level_indices_window.page_window(dir);
                self.check_window_change(old_slot, change)
            }),
            Action::ChangeActiveFilter(dir) => {
                let old_filter = self.active_filter;
                self.active_filter = match dir {
                    Direction::Previous => self.active_filter.previous(),
                    Direction::Next => self.active_filter.next(),
                };

                // Rebuild the window
                self.rebuild_window();

                Some(LevelSelectorChange {
                    slots_change: self.window_slots(),
                    filter_change: Some(FilterChange {
                        inactive: old_filter,
                        active: self.active_filter,
                    }),
                    num_locked_change: None,
                    active_rating: self.active_rating(),
                })
            }
            Action::ActiveLevelCompleted(new_status) => {
                let level_idx = self.active_level_idx();

                level_idx.and_then(|level_idx| {
                    self.level_progress
                        .attempt_status_update(level_idx, new_status)
                        .then(|| {
                            let mut slots_change = ArrayVec::new();

                            // Only the active level has changed
                            slots_change.push(LevelSlotInfo::Level {
                                level_info: self.active_level_info().unwrap(),
                                position: self
                                    .level_indices_window
                                    .position()
                                    .unwrap()
                                    .cursor_window_idx()
                                    as u8,
                                is_active: true,
                            });

                            // Refresh the window if this was completed as this may unlock a level
                            self.rebuild_window();

                            LevelSelectorChange {
                                slots_change,
                                filter_change: None,
                                num_locked_change: Some(
                                    self.level_progress.num_locked_levels() as u16
                                ),
                                active_rating: self.active_rating(),
                            }
                        })
                })
            }
        }
    }
}

// TODO Write some tests for these items, especially the order and equality of LevelStatus

// TODO: Definitely write tests to test expansion of LevelSelector, though maybe we should just do black box behavior

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_vec() {
        let mut window: WindowVec<u8, 16, 5> = WindowVec::default();

        // Empty window
        assert_eq!(window.cursor_item(), None);
        assert_eq!(window.move_cursor(Direction::Next), WindowChange::default());
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::default()
        );
        assert_eq!(window.page_window(Direction::Next), WindowChange::default());
        assert_eq!(
            window.page_window(Direction::Previous),
            WindowChange::default()
        );
        assert_eq!(window.cursor_item(), None);

        window.refill(
            [10, 8, 9, 7, 8, 6, 7, 5, 6, 4, 5, 3, 4, 2, 3, 1],
            &WindowPosition::default(),
        );

        // Basic cursor and window movement forward
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &10);
        assert_eq!(
            window.move_cursor(Direction::Next),
            WindowChange::CursorOnly
        );
        assert_eq!(
            window.move_cursor(Direction::Next),
            WindowChange::CursorOnly
        );
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &9);
        assert_eq!(
            window.move_cursor(Direction::Next),
            WindowChange::CursorOnly
        );
        assert_eq!(
            window.move_cursor(Direction::Next),
            WindowChange::CursorOnly
        );
        assert_eq!(window.move_cursor(Direction::Next), WindowChange::Window);
        assert_eq!(window.move_cursor(Direction::Next), WindowChange::Window);
        assert_eq!(window.position().unwrap().top_idx, 2);
        assert_eq!(window.cursor_item().unwrap(), &7);
        assert_eq!(window.page_window(Direction::Next), WindowChange::Window);
        assert_eq!(window.position().unwrap().top_idx, 7);
        assert_eq!(window.cursor_item().unwrap(), &3);
        assert_eq!(window.page_window(Direction::Next), WindowChange::Window);
        assert_eq!(window.position().unwrap().top_idx, 11);
        assert_eq!(window.cursor_item().unwrap(), &1);
        assert_eq!(window.move_cursor(Direction::Next), WindowChange::None);
        assert_eq!(window.page_window(Direction::Next), WindowChange::None);
        assert_eq!(window.position().unwrap().top_idx, 11);
        assert_eq!(window.cursor_item().unwrap(), &1);

        // Basic cursor and window movement backward
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(window.position().unwrap().top_idx, 11);
        assert_eq!(window.cursor_item().unwrap(), &4);
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::Window
        );
        assert_eq!(window.position().unwrap().top_idx, 10);
        assert_eq!(window.cursor_item().unwrap(), &5);
        assert_eq!(
            window.page_window(Direction::Previous),
            WindowChange::Window
        );
        assert_eq!(window.position().unwrap().top_idx, 5);
        assert_eq!(window.cursor_item().unwrap(), &6);
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::Window
        );
        assert_eq!(window.position().unwrap().top_idx, 4);
        assert_eq!(window.cursor_item().unwrap(), &8);
        assert_eq!(
            window.page_window(Direction::Previous),
            WindowChange::Window
        );
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &10);
        assert_eq!(window.move_cursor(Direction::Previous), WindowChange::None);
        assert_eq!(window.page_window(Direction::Previous), WindowChange::None);
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &10);

        // Smaller window and manually setting the position
        window.refill([4, 5, 6, 7, 8, 9, 10, 11, 12], &WindowPosition::new(2, 4));
        assert_eq!(window.position().unwrap().top_idx, 2);
        assert_eq!(window.cursor_item().unwrap(), &8);
        assert_eq!(
            window.set_position(&WindowPosition::new(1, 18)),
            WindowChange::Window
        );
        assert_eq!(window.position().unwrap().top_idx, 1);
        assert_eq!(window.cursor_item().unwrap(), &9);
        assert_eq!(
            window.set_position(&WindowPosition::new(156, 25)),
            WindowChange::Window
        );
        assert_eq!(window.position().unwrap().top_idx, 4);
        assert_eq!(window.cursor_item().unwrap(), &12);

        // When page up and down cannot fully shift the window
        assert_eq!(
            window.set_position(&WindowPosition::new(1, 3)),
            WindowChange::Window
        );
        assert_eq!(
            window.page_window(Direction::Previous),
            WindowChange::Window
        );
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &6);
        assert_eq!(
            window.page_window(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &4);
        assert_eq!(window.page_window(Direction::Previous), WindowChange::None);
        assert_eq!(
            window.set_position(&WindowPosition::new(3, 4)),
            WindowChange::Window
        );
        assert_eq!(window.page_window(Direction::Next), WindowChange::Window);
        assert_eq!(window.position().unwrap().top_idx, 4);
        assert_eq!(window.cursor_item().unwrap(), &9);
        assert_eq!(
            window.page_window(Direction::Next),
            WindowChange::CursorOnly
        );
        assert_eq!(window.position().unwrap().top_idx, 4);
        assert_eq!(window.cursor_item().unwrap(), &12);
        assert_eq!(window.page_window(Direction::Next), WindowChange::None);

        // Test with overly small window
        window.refill([200, 201, 202], &WindowPosition::new(17, 255));
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &202);
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(window.move_cursor(Direction::Previous), WindowChange::None);
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &200);
        assert_eq!(
            window.move_cursor(Direction::Next),
            WindowChange::CursorOnly
        );
        assert_eq!(
            window.move_cursor(Direction::Next),
            WindowChange::CursorOnly
        );
        assert_eq!(window.move_cursor(Direction::Next), WindowChange::None);
        assert_eq!(
            window.move_cursor(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &201);
        assert_eq!(
            window.page_window(Direction::Next),
            WindowChange::CursorOnly
        );
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &202);
        assert_eq!(
            window.page_window(Direction::Previous),
            WindowChange::CursorOnly
        );
        assert_eq!(window.position().unwrap().top_idx, 0);
        assert_eq!(window.cursor_item().unwrap(), &200);
    }
}
