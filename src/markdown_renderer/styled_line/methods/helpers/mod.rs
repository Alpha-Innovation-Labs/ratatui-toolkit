//! Text processing helper functions for markdown rendering.

use super::super::TextSegment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

pub fn render_text_segment(segment: &TextSegment, base_style: Style) -> Span<'static> {
    match segment {
        TextSegment::Plain(text) => Span::styled(text.clone(), base_style),
        TextSegment::Bold(text) => {
            Span::styled(text.clone(), base_style.add_modifier(Modifier::BOLD))
        }
        TextSegment::Italic(text) => {
            Span::styled(text.clone(), base_style.add_modifier(Modifier::ITALIC))
        }
        TextSegment::BoldItalic(text) => Span::styled(
            text.clone(),
            base_style
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        ),
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

pub fn segments_to_plain_text(segments: &[TextSegment]) -> String {
    segments
        .iter()
        .map(|seg| match seg {
            TextSegment::Plain(text) => text.clone(),
            TextSegment::Bold(text) => text.clone(),
            TextSegment::Italic(text) => text.clone(),
            TextSegment::BoldItalic(text) => text.clone(),
            TextSegment::InlineCode(text) => format!("`{}`", text),
            TextSegment::Link { text, .. } => text.clone(),
            TextSegment::Html(text) => text.clone(),
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 || text.is_empty() {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in text.split_whitespace() {
        let word_width = word.chars().count();

        if current_width == 0 {
            current_line = word.to_string();
            current_width = word_width;
        } else if current_width + 1 + word_width <= width {
            current_line.push(' ');
            current_line.push_str(word);
            current_width += 1 + word_width;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            current_width = word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
