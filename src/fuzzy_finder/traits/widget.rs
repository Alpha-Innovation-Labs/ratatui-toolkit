use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::fuzzy_finder::{centered_rect, FuzzyFinder};

impl Widget for &FuzzyFinder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(self.size_percent.0, self.size_percent.1, area);

        Clear.render(popup_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str())
            .style(Style::default().fg(Color::White));

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        if let Some(terminal) = &self.terminal {
            let parser = terminal.parser.lock().unwrap();
            let screen = parser.screen();

            for (row_idx, row) in screen.rows(0, inner.height).enumerate() {
                if row_idx >= inner.height as usize {
                    break;
                }
                let line = Line::from(row.as_str());
                let y = inner.y + row_idx as u16;
                buf.set_line(inner.x, y, &line, inner.width);
            }
        } else {
            let loading = Paragraph::new(self.loading_message.as_str())
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));
            loading.render(inner, buf);
        }
    }
}
