//! Handle click event at the given position.

use crate::markdown_renderer::render_styled_line;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::markdown_renderer::styled_line::StyledLineKind;

use super::should_render_line::should_render_line;

/// Handle click event at the given position.
///
/// # Arguments
///
/// * `_x` - X coordinate (unused)
/// * `y` - Y coordinate relative to the widget
/// * `width` - Width of the widget
/// * `content` - The markdown content
/// * `scroll` - The scroll manager
///
/// # Returns
///
/// `true` if the click was handled.
pub(crate) fn handle_click(
    _x: usize,
    y: usize,
    width: usize,
    content: &str,
    scroll: &mut MarkdownScrollManager,
) -> bool {
    let styled_lines = crate::markdown_renderer::render_markdown_to_styled_lines(content);

    // Account for scroll offset - y is relative to visible area
    let document_y = y + scroll.scroll_offset;
    let mut line_idx = 0;

    for (idx, styled_line) in styled_lines.iter().enumerate() {
        // Skip lines that shouldn't be rendered (collapsed sections)
        if !should_render_line(styled_line, idx, scroll) {
            continue;
        }

        let rendered = render_styled_line(styled_line, width);
        let line_count = rendered.len();

        if document_y >= line_idx && document_y < line_idx + line_count {
            match &styled_line.kind {
                StyledLineKind::Heading {
                    section_id,
                    collapsed: _,
                    ..
                } => {
                    scroll.toggle_section_collapse(*section_id);
                    scroll.invalidate_cache(); // Invalidate cache after toggle
                    return true;
                }
                StyledLineKind::Frontmatter { .. } => {
                    scroll.toggle_section_collapse(0);
                    scroll.invalidate_cache();
                    return true;
                }
                StyledLineKind::FrontmatterStart { .. } => {
                    scroll.toggle_section_collapse(0);
                    scroll.invalidate_cache();
                    return true;
                }
                StyledLineKind::ExpandToggle { content_id, .. } => {
                    scroll.toggle_expandable(content_id);
                    scroll.invalidate_cache();
                    return true;
                }
                _ => {}
            }
        }

        line_idx += line_count;
    }

    false
}
