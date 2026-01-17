//! Section collapse/expand methods for MarkdownScrollManager.

use super::super::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Toggle the collapse state of a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to toggle.
    pub fn toggle_section_collapse(&mut self, section_id: usize) {
        let is_collapsed = self.collapsed_sections.entry(section_id).or_insert(false);
        *is_collapsed = !*is_collapsed;
    }

    /// Set the collapse state of a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section.
    /// * `collapsed` - Whether the section should be collapsed.
    pub fn set_section_collapsed(&mut self, section_id: usize, collapsed: bool) {
        self.collapsed_sections.insert(section_id, collapsed);
    }

    /// Check if a section is collapsed (directly or via parent hierarchy).
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to check.
    ///
    /// # Returns
    ///
    /// `true` if the section or any of its parent sections is collapsed.
    pub fn is_section_collapsed(&self, section_id: usize) -> bool {
        // First check if this section is directly collapsed
        if self.collapsed_sections.get(&section_id).copied().unwrap_or(false) {
            return true;
        }

        // Check if any parent section is collapsed (hierarchical collapse)
        let mut current_id = section_id;
        while let Some(&(_level, parent_id)) = self.section_hierarchy.get(&current_id) {
            if let Some(parent) = parent_id {
                if self.collapsed_sections.get(&parent).copied().unwrap_or(false) {
                    return true;
                }
                current_id = parent;
            } else {
                break;
            }
        }

        false
    }

    /// Register section hierarchy (called during parsing).
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section.
    /// * `level` - The heading level (1-6).
    /// * `parent_section_id` - The parent section's ID, if any.
    pub fn register_section(
        &mut self,
        section_id: usize,
        level: u8,
        parent_section_id: Option<usize>,
    ) {
        self.section_hierarchy
            .insert(section_id, (level, parent_section_id));
    }

    /// Clear section hierarchy (called when content changes).
    pub fn clear_section_hierarchy(&mut self) {
        self.section_hierarchy.clear();
    }

    /// Expand a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to expand.
    pub fn expand_section(&mut self, section_id: usize) {
        self.collapsed_sections.insert(section_id, false);
    }

    /// Collapse a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to collapse.
    pub fn collapse_section(&mut self, section_id: usize) {
        self.collapsed_sections.insert(section_id, true);
    }

    /// Expand all sections.
    pub fn expand_all_sections(&mut self) {
        let section_ids: Vec<usize> = self.collapsed_sections.keys().copied().collect();
        for section_id in section_ids {
            self.collapsed_sections.insert(section_id, false);
        }
    }

    /// Collapse all sections.
    pub fn collapse_all_sections(&mut self) {
        let section_ids: Vec<usize> = self.collapsed_sections.keys().copied().collect();
        for section_id in section_ids {
            self.collapsed_sections.insert(section_id, true);
        }
    }
}
