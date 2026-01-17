//! Convert StyledLineKind to a human-readable string.

use crate::markdown_renderer::styled_line::StyledLineKind;

/// Convert StyledLineKind to a human-readable string.
///
/// # Arguments
///
/// * `kind` - The line kind to convert
///
/// # Returns
///
/// A human-readable string describing the line kind.
pub(crate) fn styled_line_kind_to_string(kind: &StyledLineKind) -> String {
    match kind {
        StyledLineKind::Heading { level, .. } => format!("Heading (H{})", level),
        StyledLineKind::HeadingBorder { .. } => "Heading Border".to_string(),
        StyledLineKind::CodeBlockHeader { language, .. } => {
            format!(
                "Code Block Header ({})",
                if language.is_empty() { "text" } else { language }
            )
        }
        StyledLineKind::CodeBlockContent { .. } => "Code Block Content".to_string(),
        StyledLineKind::CodeBlockBorder { .. } => "Code Block Border".to_string(),
        StyledLineKind::Paragraph(_) => "Paragraph".to_string(),
        StyledLineKind::ListItem { ordered, depth, .. } => {
            if *ordered {
                format!("Ordered List Item (depth {})", depth)
            } else {
                format!("Unordered List Item (depth {})", depth)
            }
        }
        StyledLineKind::Blockquote { depth, .. } => format!("Blockquote (depth {})", depth),
        StyledLineKind::TableRow { is_header, .. } => {
            if *is_header {
                "Table Header".to_string()
            } else {
                "Table Row".to_string()
            }
        }
        StyledLineKind::TableBorder(_) => "Table Border".to_string(),
        StyledLineKind::HorizontalRule => "Horizontal Rule".to_string(),
        StyledLineKind::Empty => "Empty".to_string(),
        StyledLineKind::Frontmatter { .. } => "Frontmatter".to_string(),
        StyledLineKind::FrontmatterStart { .. } => "Frontmatter Start".to_string(),
        StyledLineKind::FrontmatterField { key, .. } => format!("Frontmatter Field ({})", key),
        StyledLineKind::FrontmatterEnd => "Frontmatter End".to_string(),
        StyledLineKind::Expandable { .. } => "Expandable Content".to_string(),
        StyledLineKind::ExpandToggle { .. } => "Expand Toggle".to_string(),
    }
}
