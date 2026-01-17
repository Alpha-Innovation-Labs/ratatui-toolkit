//! A scrollable, interactive markdown widget.

mod constructors;
mod methods;
mod traits;

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

/// Mode for the markdown widget statusline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkdownWidgetMode {
    /// Normal viewing mode.
    #[default]
    Normal,
    /// Drag/selection mode.
    Drag,
}

/// A scrollable, interactive markdown widget.
///
/// This widget renders markdown content with:
/// - Scroll support (keyboard and mouse)
/// - Clickable headings to collapse/expand sections
/// - Clickable frontmatter to collapse/expand
/// - Expandable content blocks ("Show more"/"Show less")
/// - Statusline showing mode and scroll position
///
/// Note: This widget requires external scroll management. Use the
/// `render_markdown_scrollable` function along with `MarkdownScrollManager`
/// for full interactive support.
/// Git statistics for display in statusline.
#[derive(Debug, Clone, Copy, Default)]
pub struct GitStats {
    /// Lines added.
    pub additions: usize,
    /// Files modified (or lines modified depending on context).
    pub modified: usize,
    /// Lines deleted.
    pub deletions: usize,
}

#[derive(Debug)]
pub struct MarkdownWidget<'a> {
    /// The markdown content to render.
    pub(crate) content: &'a str,
    /// The scroll manager for handling scroll state.
    pub(crate) scroll: &'a mut MarkdownScrollManager,
    /// When true, use stale cache for smoother resize during drag operations.
    pub(crate) is_resizing: bool,
    /// Current mode for the statusline.
    pub(crate) mode: MarkdownWidgetMode,
    /// Whether to show the statusline.
    pub(crate) show_statusline: bool,
    /// Git statistics for the file (optional).
    pub(crate) git_stats: Option<GitStats>,
}
