use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn is_running(&self) -> bool {
        if let Some(terminal) = &self.terminal {
            if let Ok(mut child) = terminal.child.try_lock() {
                return child.try_wait().ok().flatten().is_none();
            }
        }
        false
    }
}
