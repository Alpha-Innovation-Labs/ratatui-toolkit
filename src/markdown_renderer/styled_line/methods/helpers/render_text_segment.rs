//! Render a text segment with base style.

use super::super::TextSegment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

/// Render a text segment with base style.
pub fn render_text_segment(segment: &TextSegment, base_style: Style) -> Span<'static> {
    match segment {
        TextSegment::Plain(text) => Span::styled(text.clone(), base_style),
        TextSegment::Bold(text) => {
            Span::styled(text.clone(), base_style.add_modifier(Modifier::BOLD))
        }
        TextSegment::Italic(text) => {
            Span::styled(text.clone(), base_style.add_modifier(Modifier::ITALIC))
        }
        TextSegment::InlineCode(text) => Span::styled(
            format!(" {} ", text),
            base_style
                .bg(Color::Rgb(60, 60, 60))
                .fg(Color::Rgb(230, 180, 100)),
        ),
        TextSegment::Link { text, .. } => Span::styled(
            text.clone(),
            base_style
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED),
        ),
        TextSegment::Html(text) => Span::styled(
            text.clone(),
            base_style
                .fg(Color::Rgb(100, 180, 100))
                .add_modifier(Modifier::ITALIC),
        ),
    }
}
