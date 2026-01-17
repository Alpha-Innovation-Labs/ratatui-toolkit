//! Expandable content methods for MarkdownScrollManager.

use super::super::{ExpandableState, MarkdownScrollManager};

impl MarkdownScrollManager {
    /// Set the default max lines for expandable content.
    ///
    /// # Arguments
    ///
    /// * `max_lines` - Default maximum visible lines when collapsed (minimum 1).
    pub fn set_default_max_lines(&mut self, max_lines: usize) {
        self.default_max_lines = max_lines.max(1);
    }

    /// Get max lines for expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    ///
    /// # Returns
    ///
    /// The maximum visible lines for this content, or the default if not set.
    pub fn get_max_lines(&self, content_id: &str) -> usize {
        self.expandable_content
            .get(content_id)
            .map(|state| state.max_lines)
            .unwrap_or(self.default_max_lines)
    }

    /// Set max lines for expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    /// * `max_lines` - Maximum visible lines when collapsed (minimum 1).
    pub fn set_max_lines(&mut self, content_id: &str, max_lines: usize) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.max_lines = max_lines.max(1);
    }

    /// Toggle expandable content collapsed state.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    pub fn toggle_expandable(&mut self, content_id: &str) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.collapsed = !state.collapsed;
    }

    /// Check if expandable content is collapsed.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    ///
    /// # Returns
    ///
    /// `true` if the content is collapsed (default state).
    pub fn is_expandable_collapsed(&self, content_id: &str) -> bool {
        self.expandable_content
            .get(content_id)
            .map(|state| state.collapsed)
            .unwrap_or(true)
    }

    /// Expand expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    pub fn expand_expandable(&mut self, content_id: &str) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.collapsed = false;
    }

    /// Collapse expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    pub fn collapse_expandable(&mut self, content_id: &str) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.collapsed = true;
    }
}
