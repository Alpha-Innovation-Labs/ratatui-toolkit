//! Kind of styled line for markdown rendering.

use super::TextSegment;

/// Represents the kind of styled line.
#[derive(Debug, Clone)]
pub enum StyledLineKind {
    /// Heading with level (1-6).
    Heading {
        level: u8,
        text: Vec<TextSegment>,
        /// Unique section ID for tracking collapse state (index in styled_lines).
        section_id: usize,
        /// Whether this section is collapsed.
        collapsed: bool,
    },
    /// Border below heading.
    #[allow(dead_code)]
    HeadingBorder { level: u8 },
    /// Code block header with language.
    CodeBlockHeader { language: String },
    /// Code block content line (plain text or syntax highlighted).
    CodeBlockContent {
        /// Plain text content
        content: String,
        /// Syntax highlighted text (if available)
        highlighted: Option<ratatui::text::Text<'static>>,
    },
    /// Code block border (top, middle, bottom).
    CodeBlockBorder(super::CodeBlockBorderKind),
    /// Paragraph text with formatting.
    Paragraph(Vec<TextSegment>),
    /// List item with nesting level.
    ListItem {
        depth: usize,
        ordered: bool,
        number: Option<usize>,
        content: Vec<TextSegment>,
    },
    /// Blockquote.
    Blockquote(Vec<TextSegment>),
    /// Table row.
    TableRow { cells: Vec<String>, is_header: bool },
    /// Table border.
    TableBorder(super::TableBorderKind),
    /// Horizontal rule.
    HorizontalRule,
    /// Empty line.
    Empty,
    /// YAML frontmatter (collapsible).
    /// Contains the parsed fields as key-value pairs.
    Frontmatter {
        /// The frontmatter fields (key, value).
        fields: Vec<(String, String)>,
        /// Whether the frontmatter is collapsed (shows only context_id).
        collapsed: bool,
    },
    /// Expandable content block (e.g., "Show more" / "Show less").
    Expandable {
        /// Unique ID for tracking state
        content_id: String,
        /// The content to display (already styled lines)
        lines: Vec<super::StyledLine>,
        /// Maximum number of lines to show when collapsed
        max_lines: usize,
        /// Whether currently collapsed
        collapsed: bool,
        /// Total number of lines in the content
        total_lines: usize,
    },
    /// Show more / Show less toggle button.
    ExpandToggle {
        /// The content_id this toggle belongs to
        content_id: String,
        /// Whether in expanded state (shows "Show less") or collapsed (shows "Show more")
        expanded: bool,
        /// Number of hidden lines
        hidden_count: usize,
    },
}
