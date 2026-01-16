//! Markdown rendering module for NEXUS TUI.
//!
//! Provides styled rendering of markdown content using pulldown-cmark
//! for parsing and ratatui for terminal output.
//!
//! Styling is inspired by render-markdown.nvim with:
//! - Nerd Font icons for headings
//! - Full-width colored backgrounds for headings
//! - Box-drawing borders for code blocks and tables
//! - Styled bullet markers for lists
//! - Left border for blockquotes

mod markdown_style;
mod markdown_widget;
mod render_markdown_to_lines;
mod scroll_manager;
mod styled_line;
mod syntax_highlighter;
mod theme;

#[cfg(test)]
mod tests;

pub use markdown_style::MarkdownStyle;
pub use markdown_widget::{handle_mouse_event, render_markdown_interactive, MarkdownWidget};
pub use render_markdown_to_lines::{
    render_markdown_to_lines, render_markdown_to_styled_lines,
    render_markdown_to_styled_lines_with_frontmatter_state,
};
pub use scroll_manager::{ExpandableState, MarkdownScrollManager};
pub use styled_line::methods::render::render as render_styled_line;
pub use styled_line::{StyledLine, StyledLineKind, TextSegment};
pub use syntax_highlighter::{SyntaxHighlighter, SyntaxThemeVariant};
pub use theme::{
    get_effective_theme_variant, load_theme_from_json, palettes, ColorMapping, ColorPalette,
    MarkdownTheme, ThemeVariant,
};

/// Render markdown string to ratatui Text with default styling
///
/// # Arguments
/// * `markdown` - The markdown string to render
/// * `max_width` - Optional maximum width for full-width backgrounds (defaults to 120)
pub fn render_markdown(markdown: &str, max_width: Option<usize>) -> ratatui::text::Text<'static> {
    let width = max_width.unwrap_or(120);
    let styled_lines = render_markdown_to_styled_lines(markdown);

    let mut lines = Vec::new();
    for styled_line in styled_lines {
        lines.extend(render_styled_line(&styled_line, width));
    }

    ratatui::text::Text::from(lines)
}

/// Render markdown string to ratatui Text with custom style configuration
///
/// # Arguments
/// * `markdown` - The markdown string to render
/// * `style` - Custom style configuration (currently unused, kept for API compatibility)
/// * `max_width` - Optional maximum width for full-width backgrounds (defaults to 120)
pub fn render_markdown_with_style(
    markdown: &str,
    _style: MarkdownStyle,
    max_width: Option<usize>,
) -> ratatui::text::Text<'static> {
    render_markdown(markdown, max_width)
}
