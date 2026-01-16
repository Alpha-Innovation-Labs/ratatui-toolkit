use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn with_loading_message(mut self, msg: impl Into<String>) -> Self {
        self.loading_message = msg.into();
        self
    }
}
