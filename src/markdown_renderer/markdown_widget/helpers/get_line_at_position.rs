//! Get line information at a given screen position.

use crate::markdown_renderer::render_styled_line;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::should_render_line::should_render_line;
use super::styled_line_kind_to_string::styled_line_kind_to_string;
use super::styled_line_to_plain_text::styled_line_to_plain_text;
use crate::markdown_renderer::markdown_widget::MarkdownDoubleClickEvent;

/// Get line information at the given screen position.
///
/// # Arguments
///
/// * `y` - Y coordinate relative to the widget
/// * `width` - Width of the widget
/// * `content` - The markdown content
/// * `scroll` - The scroll manager
///
/// # Returns
///
/// A `MarkdownDoubleClickEvent` if a line was found at the position.
pub(crate) fn get_line_at_position(
    y: usize,
    width: usize,
    content: &str,
    scroll: &MarkdownScrollManager,
) -> Option<MarkdownDoubleClickEvent> {
    let styled_lines = crate::markdown_renderer::render_markdown_to_styled_lines(content);
    let document_y = y + scroll.scroll_offset;
    let mut visual_line_idx = 0;
    let mut logical_line_num = 0; // Track the visible logical line number (1-indexed for display)

    for (idx, styled_line) in styled_lines.iter().enumerate() {
        if !should_render_line(styled_line, idx, scroll) {
            continue;
        }

        logical_line_num += 1; // Increment for each visible logical line

        let rendered = render_styled_line(styled_line, width);
        let line_count = rendered.len();

        if document_y >= visual_line_idx && document_y < visual_line_idx + line_count {
            let line_kind = styled_line_kind_to_string(&styled_line.kind);
            let text_content = styled_line_to_plain_text(&styled_line.kind);

            return Some(MarkdownDoubleClickEvent {
                line_number: logical_line_num,
                line_kind,
                content: text_content,
            });
        }

        visual_line_idx += line_count;
    }

    None
}
