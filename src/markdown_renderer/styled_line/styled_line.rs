//! Styled line for markdown rendering.
//!
//! Represents a single styled line that can be rendered to ratatui.

#[derive(Debug, Clone)]
pub struct StyledLine {
    /// The kind of line content.
    pub kind: super::StyledLineKind,
}
