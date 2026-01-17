//! Find the styled line at a given screen position.

use crate::markdown_renderer::render_styled_line;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::markdown_renderer::styled_line::StyledLine;

use super::helpers::should_render_line;

/// Find the styled line at the given screen position.
///
/// # Arguments
///
/// * `content` - The markdown content
/// * `screen_y` - The Y coordinate on screen
/// * `scroll` - The scroll manager
///
/// # Returns
///
/// The index and styled line at the position, if found.
#[allow(dead_code)]
pub fn find_line_at_position(
    content: &str,
    screen_y: usize,
    scroll: &MarkdownScrollManager,
) -> Option<(usize, StyledLine)> {
    let styled_lines = crate::markdown_renderer::render_markdown_to_styled_lines(content);
    let mut current_y = 0;

    for (idx, styled_line) in styled_lines.iter().enumerate() {
        if should_render_line(styled_line, idx, scroll) {
            let rendered = render_styled_line(styled_line, 80);
            let line_count = rendered.len();

            if screen_y >= current_y && screen_y < current_y + line_count {
                return Some((idx, styled_line.clone()));
            }

            current_y += line_count;
        }
    }

    None
}
