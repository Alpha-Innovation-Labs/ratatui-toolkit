use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn with_size(mut self, width_pct: u16, height_pct: u16) -> Self {
        self.size_percent = (width_pct, height_pct);
        self
    }
}
