//! Extract plain text content from a StyledLineKind.

use crate::markdown_renderer::styled_line::{StyledLineKind, TextSegment};

/// Convert segments to plain text.
fn segments_to_text(segments: &[TextSegment]) -> String {
    segments
        .iter()
        .map(|seg| match seg {
            TextSegment::Plain(t) => t.clone(),
            TextSegment::Bold(t) => t.clone(),
            TextSegment::Italic(t) => t.clone(),
            TextSegment::BoldItalic(t) => t.clone(),
            TextSegment::InlineCode(t) => format!("`{}`", t),
            TextSegment::Link { text, .. } => text.clone(),
            TextSegment::Strikethrough(t) => t.clone(),
            TextSegment::Html(t) => t.clone(),
            TextSegment::Checkbox(_) => String::new(),
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Extract plain text content from a StyledLineKind.
///
/// # Arguments
///
/// * `kind` - The line kind to extract text from
///
/// # Returns
///
/// The plain text content of the line.
pub(crate) fn styled_line_to_plain_text(kind: &StyledLineKind) -> String {
    match kind {
        StyledLineKind::Heading { text, .. } => segments_to_text(text),
        StyledLineKind::Paragraph(segments) => segments_to_text(segments),
        StyledLineKind::ListItem { content, .. } => segments_to_text(content),
        StyledLineKind::Blockquote { content, .. } => segments_to_text(content),
        StyledLineKind::CodeBlockHeader { language, .. } => format!("```{}", language),
        StyledLineKind::CodeBlockContent { content, .. } => content.clone(),
        StyledLineKind::TableRow { cells, .. } => cells.join(" | "),
        StyledLineKind::Frontmatter { fields, .. } => fields
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join(", "),
        StyledLineKind::FrontmatterStart { context_id, .. } => {
            context_id.clone().unwrap_or_else(|| "---".to_string())
        }
        StyledLineKind::FrontmatterField { key, value } => format!("{}: {}", key, value),
        StyledLineKind::FrontmatterEnd => "---".to_string(),
        _ => String::new(),
    }
}
