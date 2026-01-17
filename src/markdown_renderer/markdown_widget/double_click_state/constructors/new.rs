//! Constructor for DoubleClickState.

use super::super::DoubleClickState;

impl DoubleClickState {
    /// Create a new double-click state tracker.
    ///
    /// # Returns
    ///
    /// A new `DoubleClickState` with no pending clicks.
    pub fn new() -> Self {
        Self::default()
    }
}
