//! Check if a line should be rendered based on collapse state.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::markdown_renderer::styled_line::{StyledLine, StyledLineKind};

/// Check if a line should be rendered based on collapse state.
///
/// # Arguments
///
/// * `styled_line` - The line to check
/// * `_idx` - The index of the line (unused but kept for API compatibility)
/// * `scroll` - The scroll manager containing collapse state
///
/// # Returns
///
/// `true` if the line should be rendered.
pub(crate) fn should_render_line(
    styled_line: &StyledLine,
    _idx: usize,
    scroll: &MarkdownScrollManager,
) -> bool {
    // Headings: visible unless a parent section is collapsed (hierarchical collapse)
    if let StyledLineKind::Heading { section_id, .. } = &styled_line.kind {
        // Check if any parent section is collapsed
        if let Some(&(_level, parent_id)) = scroll.section_hierarchy.get(section_id) {
            if let Some(parent) = parent_id {
                // If parent is collapsed, this heading is hidden
                if scroll.is_section_collapsed(parent) {
                    return false;
                }
            }
        }
        return true;
    }

    // Legacy Frontmatter block is always visible
    if matches!(styled_line.kind, StyledLineKind::Frontmatter { .. }) {
        return true;
    }

    // FrontmatterStart is always visible (contains collapse toggle)
    if matches!(styled_line.kind, StyledLineKind::FrontmatterStart { .. }) {
        return true;
    }

    // FrontmatterField and FrontmatterEnd are hidden when frontmatter is collapsed
    if matches!(
        styled_line.kind,
        StyledLineKind::FrontmatterField { .. } | StyledLineKind::FrontmatterEnd
    ) {
        // Frontmatter uses section_id 0 for collapse state
        if scroll.is_section_collapsed(0) {
            return false;
        }
        return true;
    }

    // Check if this line belongs to a collapsed section
    if let Some(section_id) = styled_line.section_id {
        if scroll.is_section_collapsed(section_id) {
            return false;
        }
    }

    true
}
