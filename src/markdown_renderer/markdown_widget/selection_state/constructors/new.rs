//! Constructor for SelectionState.

use super::super::SelectionState;

impl SelectionState {
    /// Create a new inactive selection state.
    pub fn new() -> Self {
        Self::default()
    }
}
