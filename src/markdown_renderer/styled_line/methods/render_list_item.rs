//! Render list item.

use super::super::{StyledLine, BULLET_MARKERS};
use super::helpers::{segments_to_plain_text, wrap_text};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _styled_line: &StyledLine,
    depth: usize,
    ordered: bool,
    number: Option<usize>,
    content: &[super::super::TextSegment],
    width: usize,
) -> Vec<Line<'static>> {
    let indent = "  ".repeat(depth);
    let marker = if ordered {
        format!("{}. ", number.unwrap_or(1))
    } else {
        let marker_idx = depth % BULLET_MARKERS.len();
        BULLET_MARKERS[marker_idx].to_string()
    };

    let prefix = format!("{}{}", indent, marker);
    let prefix_len = prefix.chars().count();
    let content_width = width.saturating_sub(prefix_len);

    let text = segments_to_plain_text(content);
    let wrapped = wrap_text(&text, content_width);

    let mut lines = Vec::new();
    for (i, line_text) in wrapped.into_iter().enumerate() {
        if i == 0 {
            let spans = vec![
                Span::styled(indent.clone(), Style::default()),
                Span::styled(marker.clone(), Style::default().fg(Color::Cyan)),
                Span::styled(line_text, Style::default()),
            ];
            lines.push(Line::from(spans));
        } else {
            let continuation_indent = " ".repeat(prefix_len);
            lines.push(Line::from(vec![
                Span::styled(continuation_indent, Style::default()),
                Span::styled(line_text, Style::default()),
            ]));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(vec![
            Span::styled(indent, Style::default()),
            Span::styled(marker, Style::default().fg(Color::Cyan)),
        ]));
    }

    lines
}
