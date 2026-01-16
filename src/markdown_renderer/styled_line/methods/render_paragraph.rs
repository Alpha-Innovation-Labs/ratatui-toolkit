//! Render paragraph text.

use super::super::StyledLine;
use super::helpers::wrap_text;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _styled_line: &StyledLine,
    segments: &[super::super::TextSegment],
    width: usize,
) -> Vec<Line<'static>> {
    let plain_text = segments_to_plain_text(segments);
    let wrapped = wrap_text(&plain_text, width);

    wrapped
        .into_iter()
        .map(|line_text| {
            let spans = render_segments_with_style(&line_text, segments);
            Line::from(spans)
        })
        .collect()
}

fn segments_to_plain_text(segments: &[super::super::TextSegment]) -> String {
    segments
        .iter()
        .map(|seg| match seg {
            super::super::TextSegment::Plain(text) => text.clone(),
            super::super::TextSegment::Bold(text) => text.clone(),
            super::super::TextSegment::Italic(text) => text.clone(),
            super::super::TextSegment::BoldItalic(text) => text.clone(),
            super::super::TextSegment::InlineCode(text) => format!("`{}`", text),
            super::super::TextSegment::Link { text, .. } => text.clone(),
            super::super::TextSegment::Html(content) => content.clone(),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn render_segments_with_style(
    line_text: &str,
    segments: &[super::super::TextSegment],
) -> Vec<Span<'static>> {
    if segments.is_empty() {
        return vec![Span::styled(line_text.to_string(), Style::default())];
    }

    let mut spans = Vec::new();
    let mut current_plain = String::new();

    for segment in segments {
        match segment {
            super::super::TextSegment::Plain(text) => {
                current_plain.push_str(text);
            }
            super::super::TextSegment::Bold(text) => {
                if !current_plain.is_empty() {
                    spans.push(Span::styled(current_plain.clone(), Style::default()));
                    current_plain.clear();
                }
                spans.push(Span::styled(
                    text.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ));
            }
            super::super::TextSegment::Italic(text) => {
                if !current_plain.is_empty() {
                    spans.push(Span::styled(current_plain.clone(), Style::default()));
                    current_plain.clear();
                }
                spans.push(Span::styled(
                    text.clone(),
                    Style::default().add_modifier(Modifier::ITALIC),
                ));
            }
            super::super::TextSegment::BoldItalic(text) => {
                if !current_plain.is_empty() {
                    spans.push(Span::styled(current_plain.clone(), Style::default()));
                    current_plain.clear();
                }
                spans.push(Span::styled(
                    text.clone(),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ));
            }
            super::super::TextSegment::InlineCode(text) => {
                if !current_plain.is_empty() {
                    spans.push(Span::styled(current_plain.clone(), Style::default()));
                    current_plain.clear();
                }
                spans.push(Span::styled(
                    format!(" {} ", text),
                    Style::default()
                        .bg(ratatui::style::Color::Rgb(60, 60, 60))
                        .fg(ratatui::style::Color::Rgb(230, 180, 100)),
                ));
            }
            super::super::TextSegment::Link { text, .. } => {
                current_plain.push_str(text);
            }
            super::super::TextSegment::Html(content) => {
                current_plain.push_str(content);
            }
        }
    }

    if !current_plain.is_empty() {
        spans.push(Span::styled(current_plain, Style::default()));
    }

    if spans.is_empty() {
        vec![Span::styled(line_text.to_string(), Style::default())]
    } else {
        spans
    }
}
