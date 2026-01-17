//! Viewport management methods for MarkdownScrollManager.

use ratatui::layout::Rect;

use super::super::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Update viewport dimensions.
    ///
    /// # Arguments
    ///
    /// * `area` - The new viewport area.
    pub fn update_viewport(&mut self, area: Rect) {
        self.viewport_height = area.height as usize;
    }

    /// Update total line count.
    ///
    /// # Arguments
    ///
    /// * `total` - The total number of lines in the document.
    pub fn update_total_lines(&mut self, total: usize) {
        self.total_lines = total.max(1);
    }
}
