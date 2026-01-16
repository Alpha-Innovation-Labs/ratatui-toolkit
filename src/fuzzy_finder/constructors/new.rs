use crate::fuzzy_finder::{FuzzyFinder, FuzzyFinderTerminal};

impl FuzzyFinder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            terminal: None,
            size_percent: (80, 80),
            title: title.into(),
            loading_message: "Loading...".to_string(),
        }
    }
}
