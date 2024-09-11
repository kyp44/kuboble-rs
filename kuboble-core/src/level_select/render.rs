use super::{Filter, LevelSelector, LevelSelectorChange, LevelSlotInfo};
use strum::IntoEnumIterator;

pub trait LevelSelectRenderer {
    fn draw_level_slot(&mut self, level_slot_info: &LevelSlotInfo);
    fn update_filter(&mut self, filter: Filter, is_active: bool);
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
    }
}
