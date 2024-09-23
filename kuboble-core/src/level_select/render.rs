use crate::{BufferedRenderer, LevelRating};

use super::{Filter, LevelSelector, LevelSelectorChange, LevelSlotInfo};
use strum::IntoEnumIterator;

pub trait LevelSelectRenderer: BufferedRenderer {
    fn draw_level_slot(&mut self, level_slot_info: &LevelSlotInfo);
    fn update_filter(&mut self, filter: Filter, is_active: bool);
    fn update_num_locked(&mut self, num_locked: u16);
    fn update_active_rating(&mut self, rating: Option<LevelRating>);
}

impl<const W: usize> LevelSelector<'_, W> {
    pub fn render<R: LevelSelectRenderer>(&self, renderer: &mut R) {
        // Draw filters
        for filter in Filter::iter() {
            renderer.update_filter(filter, filter == self.active_filter);
        }

        // Draw slots
        for slot in self.window_slots() {
            renderer.draw_level_slot(&slot);
        }

        // Draw locked levels
        renderer.update_num_locked(self.level_progress.num_locked_levels() as u16);

        // Draw level rating
        renderer.update_active_rating(self.active_rating());

        renderer.flush();
    }
}

impl<const W: usize> LevelSelectorChange<W> {
    pub fn render<R: LevelSelectRenderer>(&self, renderer: &mut R) {
        // Render any slots
        for slot in self.slots_change.iter() {
            renderer.draw_level_slot(slot);
        }

        // Render filter change if applicable
        if let Some(ref filter_change) = self.filter_change {
            renderer.update_filter(filter_change.inactive, false);
            renderer.update_filter(filter_change.active, true);
        }

        // Render locked levels if changed
        if let Some(n) = self.num_locked_change {
            renderer.update_num_locked(n);
        }

        // Render active rating if changed
        renderer.update_active_rating(self.active_rating);

        renderer.flush();
    }
}
