use crate::fuzzy_finder::FuzzyFinder;

impl Drop for FuzzyFinder {
    fn drop(&mut self) {
        if let Some(terminal) = &self.terminal {
            if let Ok(mut child) = terminal.child.lock() {
                let _ = child.kill();
            }
        }
    }
}
