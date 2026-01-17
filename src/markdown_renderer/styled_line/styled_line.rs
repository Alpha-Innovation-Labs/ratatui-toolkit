//! Styled line for markdown rendering.
//!
//! Represents a single styled line that can be rendered to ratatui.

#[derive(Debug, Clone, Default)]
pub struct StyledLine {
    /// The kind of line content.
    pub kind: super::StyledLineKind,
    /// The section this line belongs to (for collapse/expand).
    /// None means this line is not part of any collapsible section.
    pub section_id: Option<usize>,
    /// The source line number (1-indexed) in the original markdown.
    /// Used for double-click reporting. Default is 0 (unknown).
    pub source_line: usize,
}

impl StyledLine {
    /// Create a new styled line with source line tracking.
    pub fn new(kind: super::StyledLineKind, section_id: Option<usize>, source_line: usize) -> Self {
        Self {
            kind,
            section_id,
            source_line,
        }
    }
}
