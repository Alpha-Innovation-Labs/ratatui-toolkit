//! Cache management methods for MarkdownScrollManager.

use super::super::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Invalidate both parsed and render caches.
    ///
    /// Call this when content changes.
    pub fn invalidate_cache(&mut self) {
        self.parsed_cache = None;
        self.render_cache = None;
        self.section_hierarchy.clear();
    }

    /// Invalidate only the render cache.
    ///
    /// Call this when width changes but content is the same.
    pub fn invalidate_render_cache(&mut self) {
        self.render_cache = None;
    }
}
