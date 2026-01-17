//! Scroll navigation methods for MarkdownScrollManager.

use super::super::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Scroll up by given number of lines.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of lines to scroll up.
    pub fn scroll_up(&mut self, amount: usize) {
        let max_offset = self.max_scroll_offset();
        self.scroll_offset = self.scroll_offset.saturating_sub(amount).min(max_offset);
    }

    /// Scroll down by given number of lines.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of lines to scroll down.
    pub fn scroll_down(&mut self, amount: usize) {
        let max_offset = self.max_scroll_offset();
        self.scroll_offset = (self.scroll_offset + amount).min(max_offset);
    }

    /// Move current line up (for keyboard navigation).
    pub fn line_up(&mut self) {
        if self.current_line > 1 {
            self.current_line -= 1;
        }
        self.adjust_scroll_for_current_line();
    }

    /// Move current line down (for keyboard navigation).
    pub fn line_down(&mut self) {
        if self.current_line < self.total_lines {
            self.current_line += 1;
        }
        self.adjust_scroll_for_current_line();
    }

    /// Move to top of document.
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
        self.current_line = 1;
    }

    /// Move to bottom of document.
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.max_scroll_offset();
        self.current_line = self.total_lines;
    }

    /// Set current line and adjust scroll to keep it visible.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number to set as current (1-indexed).
    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = line.clamp(1, self.total_lines);
        self.adjust_scroll_for_current_line();
    }

    /// Adjust scroll offset to ensure current_line is visible.
    pub fn adjust_scroll_for_current_line(&mut self) {
        if self.current_line < self.scroll_offset + 1 {
            self.scroll_offset = self.current_line.saturating_sub(1);
        }
        if self.viewport_height > 0 && self.current_line > self.scroll_offset + self.viewport_height
        {
            self.scroll_offset = self.current_line.saturating_sub(self.viewport_height);
        }
    }

    /// Check if current line is visible in the viewport.
    ///
    /// # Returns
    ///
    /// `true` if the current line is within the visible viewport.
    pub fn is_current_line_visible(&self) -> bool {
        let first_visible = self.scroll_offset + 1;
        let last_visible = self.scroll_offset + self.viewport_height;
        self.current_line >= first_visible && self.current_line <= last_visible
    }

    /// Get the maximum valid scroll offset.
    ///
    /// # Returns
    ///
    /// The maximum scroll offset that keeps content visible.
    pub fn max_scroll_offset(&self) -> usize {
        self.total_lines.saturating_sub(self.viewport_height)
    }

    /// Get range of currently visible lines (1-indexed, inclusive).
    ///
    /// # Returns
    ///
    /// A tuple of (start_line, end_line) for visible content.
    pub fn visible_range(&self) -> (usize, usize) {
        let start = self.scroll_offset + 1;
        let end = (self.scroll_offset + self.viewport_height).min(self.total_lines);
        (start, end)
    }

    /// Calculate percentage scrolled (0.0 to 1.0).
    ///
    /// # Returns
    ///
    /// The scroll position as a percentage of total scrollable content.
    pub fn scroll_percentage(&self) -> f64 {
        let max_offset = self.max_scroll_offset();
        if max_offset == 0 {
            0.0
        } else {
            self.scroll_offset as f64 / max_offset as f64
        }
    }
}
