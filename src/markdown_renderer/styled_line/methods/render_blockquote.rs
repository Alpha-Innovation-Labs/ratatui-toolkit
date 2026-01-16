//! Render blockquote.

use super::super::StyledLine;
use super::helpers::{segments_to_plain_text, wrap_text};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _styled_line: &StyledLine,
    segments: &[super::super::TextSegment],
    width: usize,
) -> Vec<Line<'static>> {
    let prefix = "▋ ";
    let prefix_len = 2;
    let content_width = width.saturating_sub(prefix_len);

    let text = segments_to_plain_text(segments);
    let wrapped = wrap_text(&text, content_width);

    let quote_style = Style::default().fg(Color::Rgb(150, 150, 180));
    let prefix_style = Style::default().fg(Color::Rgb(100, 100, 150));

    wrapped
        .into_iter()
        .map(|line_text| {
            Line::from(vec![
                Span::styled(prefix, prefix_style),
                Span::styled(line_text, quote_style),
            ])
        })
        .collect()
}
