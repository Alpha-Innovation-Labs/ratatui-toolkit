//! Render markdown to styled lines with scroll state applied.

use ratatui::text::Line;

use crate::markdown_renderer::render_styled_line;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::helpers::should_render_line;

/// Render markdown to styled lines with scroll state applied.
///
/// # Arguments
///
/// * `content` - The markdown content
/// * `scroll` - The scroll manager
/// * `width` - The width to render to
///
/// # Returns
///
/// A vector of rendered lines.
#[allow(dead_code)]
pub fn render_markdown_scrollable(
    content: &str,
    scroll: &MarkdownScrollManager,
    width: usize,
) -> Vec<Line<'static>> {
    let styled_lines = crate::markdown_renderer::render_markdown_to_styled_lines(content);
    let mut result = Vec::new();

    for (idx, styled_line) in styled_lines.iter().enumerate() {
        if should_render_line(styled_line, idx, scroll) {
            let rendered = render_styled_line(styled_line, width);
            result.extend(rendered);
        }
    }

    result
}
