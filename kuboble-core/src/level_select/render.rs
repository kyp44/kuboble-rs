use super::{Filter, LevelInfo, LevelSelector, LevelSelectorChanged, LevelSlotInfo};
use crate::{board::render, levels::NUM_LEVELS};
use strum::IntoEnumIterator;

pub trait LevelSelectRenderer {
    fn draw_level_slot(&mut self, level_slot_info: &LevelSlotInfo);
    fn update_filter(&mut self, filter: Filter, is_active: bool);
}

impl LevelSelector<'_> {
    pub fn render<R: LevelSelectRenderer>(&self, renderer: &mut R) {
        // Draw filters
        for filter in Filter::iter() {
            renderer.update_filter(filter, filter == self.active_filter);
        }

        for idx in 0..NUM_LEVELS {
            renderer.draw_level_slot(&LevelSlotInfo {
                level_info: self.level_progress.level_info(idx),
                position: idx.try_into().unwrap(),
                is_active: idx == self.active_level_idx,
            });
        }
    }
}

impl LevelSelectorChanged {
    pub fn render<R: LevelSelectRenderer>(&self, renderer: &mut R) {
        match self {
            LevelSelectorChanged::SlotsSwap(changed_slots) => {
                for change in changed_slots {
                    renderer.draw_level_slot(change)
                }
            }
            LevelSelectorChanged::Filter {
                inactive: deselect,
                active: select,
            } => {
                renderer.update_filter(*deselect, false);
                renderer.update_filter(*select, true)
            }
        }
    }
}
