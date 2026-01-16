use std::sync::{Arc, Mutex};
use vt100::Parser;

use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn get_parser(&self) -> Option<Arc<Mutex<Parser>>> {
        self.terminal.as_ref().map(|t| Arc::clone(&t.parser))
    }
}
