use core::{cmp::Ordering, mem::discriminant};

use crate::{
    board::Move,
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

#[derive(Default, Serialize, Deserialize)]
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

    // Only updates the status if it is better and returns whether it was updated
    pub fn attempt_status_update(&mut self, level_idx: usize, new_status: LevelStatus) -> bool {
        if new_status > self.level_statuses[level_idx] {
            self.level_statuses[level_idx] = new_status;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Previous,
    Next,
}

// TODO: Need iterator?
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
}

pub enum Action {
    ChangeActiveLevel(Direction),
    ChangeActiveFilter(Direction),
    ActiveLevelCompleted(LevelStatus),
}

pub struct LevelSlotInfo {
    pub level_info: LevelInfo,
    pub position: u8,
    pub is_active: bool,
}

pub enum LevelSelectorChanged {
    UpdateSlot(LevelSlotInfo),
    SlotsSwap([LevelSlotInfo; 2]),
    Filter { inactive: Filter, active: Filter },
}

pub struct LevelSelector<'a, const W: usize> {
    level_progress: &'a mut LevelProgress,
    active_level_idx: usize,
    active_filter: Filter,
    window_size: u8,
}
impl<'a, const W: usize> LevelSelector<'a, W> {
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
            Action::ActiveLevelCompleted(new_status) => {
                self.level_progress
                    .attempt_status_update(self.active_level_idx, new_status)
                    .then(|| {
                        // Need to update the active level
                        LevelSelectorChanged::UpdateSlot(LevelSlotInfo {
                            level_info: self.level_progress.level_info(self.active_level_idx),
                            position: self.active_level_idx.try_into().unwrap(),
                            is_active: true,
                        })

                        // TODO Will need to unlock another level probably.
                    })
            }
        }
    }
}

// TODO Write some tests for these items, especially the order and equality of LevelStatus
