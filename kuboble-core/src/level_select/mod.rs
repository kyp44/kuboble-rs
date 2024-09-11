use core::{cmp::Ordering, iter::repeat, mem::discriminant};

use crate::{
    level_run::Move,
    levels::{LEVELS, MAX_OPTIMAL_MOVES, NUM_LEVELS},
    Level, LevelRating,
};
use arrayvec::ArrayVec;
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

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, EnumIter)]
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
        u16::try_from(self.index).unwrap() + 1
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
            .take(NUM_LEVELS)
            .enumerate()
            .filter_map(move |(idx, ls)| filter.passes(ls).then_some(idx))
    }
}

// Evidently a crate contain something like this just does not exist
struct WindowVec<T, const C: usize, const W: usize> {
    vec: ArrayVec<T, C>,
    window_top_idx: usize,
}
impl<T, const C: usize, const W: usize> FromIterator<T> for WindowVec<T, C, W> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            vec: ArrayVec::from_iter(iter),
            window_top_idx: 0,
        }
    }
}
impl<T, const C: usize, const W: usize> WindowVec<T, C, W> {
    // Clear and refill from iterator, resetting the window
    pub fn refill<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.vec.clear();
        self.vec.extend(iter);
        self.window_top_idx = 0;
    }

    #[inline]
    pub fn window_len(&self) -> usize {
        self.vec.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(self.window_top_idx + idx)
    }

    // Shifts the window back one
    pub fn shift_back(&mut self) -> bool {
        if self.window_top_idx > 0 {
            self.window_top_idx -= 1;
            true
        } else {
            false
        }
    }

    // Shifts the window forward one
    pub fn shift_forward(&mut self) -> bool {
        if self.window_top_idx + W < self.vec.len() {
            self.window_top_idx += 1;
            true
        } else {
            false
        }
    }

    // Iterate over full window length
    pub fn iter(&self) -> impl Iterator<Item = Option<&T>> {
        self.vec[self.window_top_idx..self.vec.len().min(self.window_top_idx + W)]
            .into_iter()
            .map(|x| Some(x))
            .chain(repeat(None))
            .take(W)
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Previous,
    Next,
}
impl Direction {
    pub fn delta(&self) -> i8 {
        match self {
            Direction::Previous => -1,
            Direction::Next => 1,
        }
    }
}

pub enum Action {
    ChangeActiveLevel(Direction),
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
}

pub struct LevelSelector<'a, const W: usize> {
    level_progress: &'a mut LevelProgress,
    active_filter: Filter,
    level_indices_window: WindowVec<u16, NUM_LEVELS, W>,
    active_window_idx: Option<u8>,
}
impl<'a, const W: usize> LevelSelector<'a, W> {
    pub fn new(level_progress: &'a mut LevelProgress) -> Self {
        let level_indices_window = WindowVec::from_iter(
            level_progress
                .filtered_indices(Filter::default())
                .map(|i| i.try_into().unwrap()),
        );

        let active_window_idx = (!level_indices_window.is_empty()).then_some(0);

        Self {
            level_progress,
            active_filter: Filter::default(),
            level_indices_window,
            active_window_idx,
        }
    }

    fn active_level_idx(&self) -> Option<usize> {
        self.active_window_idx
            .and_then(|idx| self.level_indices_window.get(idx as usize))
            .map(|idx| *idx as usize)
    }

    pub fn active_level_info(&self) -> Option<LevelInfo> {
        self.active_level_idx()
            .map(|level_idx| self.level_progress.level_info(level_idx))
    }

    fn window_slots(&self) -> ArrayVec<LevelSlotInfo, W> {
        self.level_indices_window
            .iter()
            .enumerate()
            .map(|(posi, idx)| {
                let pos = posi.try_into().unwrap();

                match idx {
                    Some(i) => LevelSlotInfo::Level {
                        level_info: self.level_progress.level_info(*i as usize),
                        position: pos,
                        is_active: pos == self.active_window_idx.unwrap(),
                    },
                    None => LevelSlotInfo::Empty(pos),
                }
            })
            .collect()
    }

    fn change_active_level(&mut self, direction: Direction) -> Option<LevelSelectorChange<W>> {
        let mut change = None;

        if let Some(window_idx) = self.active_window_idx {
            if match direction {
                Direction::Previous => window_idx > 0,
                Direction::Next => {
                    (window_idx as usize) < W.min(self.level_indices_window.window_len()) - 1
                }
            } {
                // Can Just move the active selection up without changing the window
                let mut slots_change = ArrayVec::new();

                slots_change.push(LevelSlotInfo::Level {
                    level_info: self.active_level_info().unwrap(),
                    position: window_idx.try_into().unwrap(),
                    is_active: false,
                });

                self.active_window_idx =
                    Some((window_idx as i8 + direction.delta()).try_into().unwrap());

                slots_change.push(LevelSlotInfo::Level {
                    level_info: self.active_level_info().unwrap(),
                    position: self.active_window_idx.unwrap().try_into().unwrap(),
                    is_active: true,
                });

                change = Some(LevelSelectorChange {
                    slots_change,
                    filter_change: None,
                });
            } else {
                // Need to shift the window
                if match direction {
                    Direction::Previous => self.level_indices_window.shift_back(),
                    Direction::Next => self.level_indices_window.shift_forward(),
                } {
                    // The window was actually shifted, so the entire window likely changed
                    change = Some(LevelSelectorChange {
                        slots_change: self.window_slots(),
                        filter_change: None,
                    });
                }
            }
        }

        change
    }

    pub fn execute_action(&mut self, action: Action) -> Option<LevelSelectorChange<W>> {
        match action {
            Action::ChangeActiveLevel(dir) => self.change_active_level(dir),
            Action::ChangeActiveFilter(dir) => {
                let old_filter = self.active_filter;
                self.active_filter = match dir {
                    Direction::Previous => self.active_filter.previous(),
                    Direction::Next => self.active_filter.next(),
                };

                // Rebuild the window
                self.level_indices_window.refill(
                    self.level_progress
                        .filtered_indices(self.active_filter)
                        .map(|i| i.try_into().unwrap()),
                );
                self.active_window_idx = (!self.level_indices_window.is_empty()).then_some(0);

                Some(LevelSelectorChange {
                    slots_change: self.window_slots(),
                    filter_change: Some(FilterChange {
                        inactive: old_filter,
                        active: self.active_filter,
                    }),
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
                                position: self.active_window_idx.unwrap(),
                                is_active: true,
                            });

                            LevelSelectorChange {
                                slots_change,
                                filter_change: None,
                            }

                            // TODO Will need to unlock another level probably.
                        })
                })
            }
        }
    }
}

// TODO Write some tests for these items, especially the order and equality of LevelStatus

// TODO: Definitely write tests to test expansion of LevelSelector, though maybe we should just do black box behavior
