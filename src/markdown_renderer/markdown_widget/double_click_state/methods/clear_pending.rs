//! Clear pending method for DoubleClickState.

use super::super::DoubleClickState;

impl DoubleClickState {
    /// Clear any pending click (e.g., when double-click is detected).
    pub fn clear_pending(&mut self) {
        self.pending_single_click = None;
    }
}
