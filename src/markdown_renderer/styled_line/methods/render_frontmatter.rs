//! Render frontmatter.

use super::super::StyledLine;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _styled_line: &StyledLine,
    fields: &[(String, String)],
    collapsed: bool,
    _width: usize,
) -> Vec<Line<'static>> {
    let border_style = Style::default().fg(Color::DarkGray);
    let key_style = Style::default().fg(Color::Rgb(130, 180, 255));
    let value_style = Style::default().fg(Color::Rgb(180, 180, 180));
    let collapse_icon_style = Style::default().fg(Color::Yellow);

    if collapsed {
        let context_id = fields
            .iter()
            .find(|(k, _)| k == "context_id")
            .map(|(_, v)| v.as_str())
            .unwrap_or("frontmatter");

        vec![Line::from(vec![
            Span::styled("▶ ", collapse_icon_style),
            Span::styled("---", border_style),
            Span::styled(" ", Style::default()),
            Span::styled(context_id.to_string(), key_style),
            Span::styled(" ", Style::default()),
            Span::styled("---", border_style),
        ])]
    } else {
        let mut lines = Vec::new();

        lines.push(Line::from(vec![
            Span::styled("▼ ", collapse_icon_style),
            Span::styled("---", border_style),
        ]));

        for (key, value) in fields {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(format!("{}: ", key), key_style),
                Span::styled(value.clone(), value_style),
            ]));
        }

        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("---", border_style),
        ]));

        lines
    }
}
