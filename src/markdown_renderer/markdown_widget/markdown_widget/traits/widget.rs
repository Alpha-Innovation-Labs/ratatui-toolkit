//! Widget trait implementation for MarkdownWidget.

use ratatui::{layout::Rect, widgets::Widget};

use super::super::MarkdownWidget;
use crate::markdown_renderer::markdown_widget::render_markdown_interactive_with_options;

impl<'a> Widget for MarkdownWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Reserve space for statusline if enabled
        let (content_area, statusline_area) = if self.show_statusline && area.height > 1 {
            (
                Rect {
                    height: area.height.saturating_sub(1),
                    ..area
                },
                Some(Rect {
                    y: area.y + area.height.saturating_sub(1),
                    height: 1,
                    ..area
                }),
            )
        } else {
            (area, None)
        };

        self.scroll.update_viewport(content_area);

        let text = render_markdown_interactive_with_options(
            self.content,
            self.scroll,
            content_area,
            self.is_resizing,
        );

        // Render markdown content
        for (i, line) in text.lines.iter().enumerate() {
            if i < content_area.height as usize {
                let y = content_area.y + i as u16;
                let mut x = content_area.x;
                for span in line.spans.iter() {
                    let span_width = span.content.chars().count() as u16;
                    if x.saturating_sub(content_area.x) < content_area.width {
                        buf.set_string(x, y, &span.content, span.style);
                        x = x.saturating_add(span_width);
                    }
                }
            }
        }

        // Render statusline
        if let Some(sl_area) = statusline_area {
            self.render_statusline(sl_area, buf);
        }
    }
}
