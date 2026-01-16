use anyhow::Result;

use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn kill(&mut self) -> Result<()> {
        if let Some(terminal) = &self.terminal {
            let mut child = terminal.child.lock().unwrap();
            child.kill()?;
        }
        Ok(())
    }
}
