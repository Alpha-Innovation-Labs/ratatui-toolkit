//! Render horizontal rule.

use super::super::StyledLine;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(_styled_line: &StyledLine, width: usize) -> Line<'static> {
    let rule = "─".repeat(width);
    Line::from(Span::styled(rule, Style::default().fg(Color::DarkGray)))
}
