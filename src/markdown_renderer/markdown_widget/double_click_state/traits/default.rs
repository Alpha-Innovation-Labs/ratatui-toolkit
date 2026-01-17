//! Default trait implementation for DoubleClickState.

use super::super::DoubleClickState;

impl Default for DoubleClickState {
    fn default() -> Self {
        Self {
            last_click_time: None,
            last_click_pos: None,
            pending_single_click: None,
        }
    }
}
