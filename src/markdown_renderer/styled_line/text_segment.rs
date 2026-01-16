//! Text segment types for markdown styling.
//!
//! Represents different types of text segments within markdown content.

#[derive(Debug, Clone)]
pub enum TextSegment {
    /// Plain text.
    Plain(String),
    /// Bold text.
    Bold(String),
    /// Italic text.
    Italic(String),
    /// Bold and italic text.
    BoldItalic(String),
    /// Inline code with background.
    InlineCode(String),
    /// Link text.
    #[allow(dead_code)]
    Link { text: String, url: String },
    /// HTML tag or autolink.
    Html(String),
}
