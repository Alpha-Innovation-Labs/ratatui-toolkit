//! Events emitted by the markdown widget.

/// Events that can be emitted by the markdown widget.
#[derive(Debug, Clone)]
pub enum MarkdownEvent {
    /// No event occurred.
    None,
    /// Text was copied to clipboard.
    Copied,
    /// A double-click occurred on a line.
    DoubleClick {
        /// Source line number (1-indexed).
        line_number: usize,
        /// Type of line clicked (e.g., "Heading", "Paragraph").
        line_kind: String,
        /// Text content of the line.
        content: String,
    },
    /// Selection mode was entered.
    SelectionStarted,
    /// Selection mode was exited.
    SelectionEnded,
}
