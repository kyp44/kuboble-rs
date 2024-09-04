use core::default;

use crate::{
    board::Move,
    levels::{LEVELS, MAX_OPTIMAL_MOVES, NUM_LEVELS},
    Level, LevelRating, Vector,
};
use arrayvec::ArrayVec;
use strum::EnumIter;

pub mod render;

#[derive(Default)]
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

#[derive(Clone)]
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

// TODO: Serialization
#[derive(Default)]
pub struct LevelProgress {
    // TODO: For serialization purposes, does this need to be a an ArrayVec?
    level_statuses: [LevelStatus; NUM_LEVELS],
}
impl LevelProgress {
    pub fn level_info(&self, level_idx: usize) -> LevelInfo {
        let level = &LEVELS[level_idx];

        LevelInfo {
            index: level_idx,
            rating: self.level_statuses[level_idx].rating(),
            level,
        }
    }

    #[inline]
    pub fn update_status(&mut self, level_idx: usize, level_status: LevelStatus) {
        self.level_statuses[level_idx] = level_status;
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Previous,
    Next,
}

// TODO: Need iterator?
#[derive(Clone, Copy, Default, PartialEq, Eq, EnumIter)]
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
}

pub enum Action {
    ChangeActiveLevel(Direction),
    ChangeActiveFilter(Direction),
}

pub struct LevelSlotInfo {
    pub level_info: LevelInfo,
    pub position: u8,
    pub is_active: bool,
}

pub enum LevelSelectorChanged {
    SlotsSwap([LevelSlotInfo; 2]),
    Filter { inactive: Filter, active: Filter },
}

pub struct LevelSelector<'a> {
    level_progress: &'a mut LevelProgress,
    active_level_idx: usize,
    active_filter: Filter,
    window_size: u8,
}
impl<'a> LevelSelector<'a> {
    pub fn new(level_progress: &'a mut LevelProgress, window_size: u8) -> Self {
        Self {
            level_progress,
            active_level_idx: 0,
            active_filter: Filter::default(),
            window_size,
        }
    }

    pub fn selected_level(&self) -> LevelInfo {
        self.level_progress.level_info(self.active_level_idx)
    }

    pub fn execute_action(&mut self, action: Action) -> Option<LevelSelectorChanged> {
        match action {
            Action::ChangeActiveLevel(dir) => {
                let new_idx = match dir {
                    Direction::Previous => self.active_level_idx.saturating_sub(1),
                    Direction::Next => (NUM_LEVELS - 1).min(self.active_level_idx + 1),
                };

                if self.active_level_idx != new_idx {
                    let old_idx = self.active_level_idx;
                    self.active_level_idx = new_idx;

                    Some(LevelSelectorChanged::SlotsSwap([
                        LevelSlotInfo {
                            level_info: self.level_progress.level_info(old_idx),
                            position: old_idx.try_into().unwrap(),
                            is_active: false,
                        },
                        LevelSlotInfo {
                            level_info: self.level_progress.level_info(new_idx),
                            position: new_idx.try_into().unwrap(),
                            is_active: true,
                        },
                    ]))
                } else {
                    None
                }
            }
            Action::ChangeActiveFilter(dir) => {
                let old_filter = self.active_filter;
                self.active_filter = match dir {
                    Direction::Previous => self.active_filter.previous(),
                    Direction::Next => self.active_filter.next(),
                };

                Some(LevelSelectorChanged::Filter {
                    inactive: old_filter,
                    active: self.active_filter,
                })
            }
        }
    }
}
