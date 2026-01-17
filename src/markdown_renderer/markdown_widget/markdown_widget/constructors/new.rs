//! Constructor for MarkdownWidget.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::super::{MarkdownWidget, MarkdownWidgetMode};

impl<'a> MarkdownWidget<'a> {
    /// Create a new MarkdownWidget with the given content and scroll manager.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to render
    /// * `scroll` - The scroll manager for handling scroll state
    ///
    /// # Returns
    ///
    /// A new `MarkdownWidget` instance.
    pub fn new(content: &'a str, scroll: &'a mut MarkdownScrollManager) -> Self {
        Self {
            content,
            scroll,
            is_resizing: false,
            mode: MarkdownWidgetMode::Normal,
            show_statusline: true,
            git_stats: None,
        }
    }
}
