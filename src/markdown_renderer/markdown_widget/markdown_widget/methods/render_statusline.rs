//! Statusline rendering for MarkdownWidget.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
};
use unicode_width::UnicodeWidthStr;

use super::super::{MarkdownWidget, MarkdownWidgetMode};
use crate::statusline_stacked::{StatusLineStacked, SLANT_BL_TR, SLANT_TL_BR};

impl<'a> MarkdownWidget<'a> {
    /// Render the statusline using StatusLineStacked (powerline style).
    ///
    /// The statusline displays:
    /// - Mode indicator (NORMAL/DRAG) on the left with colored background
    /// - Filename with git stats (no background on git icons)
    /// - Scroll position (percentage/total lines) on the right
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the statusline in
    /// * `buf` - The buffer to render to
    pub(crate) fn render_statusline(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Mode colors and text
        let (mode_text, mode_color) = match self.mode {
            MarkdownWidgetMode::Normal => (" NORMAL ", Color::Rgb(97, 175, 239)), // blue
            MarkdownWidgetMode::Drag => (" DRAG ", Color::Rgb(229, 192, 123)),    // yellow/orange
        };

        let file_bg = Color::Rgb(58, 58, 58); // slightly darker than #686868

        // Get filename from source path
        let filename = self.scroll.source_path()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        // Position info - use source line count for accurate display
        let display_total = if self.scroll.source_line_count > 0 {
            self.scroll.source_line_count
        } else {
            self.scroll.total_lines
        };
        let current_line = self.scroll.current_line;
        let percentage = if display_total == 0 {
            0
        } else {
            (current_line * 100) / display_total.max(1)
        };
        let position_text = format!(" {}%/{} ", percentage, display_total);
        let position_bg = Color::Rgb(171, 178, 191);

        // Build the statusline
        let mut statusline = StatusLineStacked::new()
            // Mode segment (left)
            .start(
                Span::from(mode_text)
                    .style(Style::new().fg(Color::Black).bg(mode_color).add_modifier(Modifier::BOLD)),
                Span::from(SLANT_TL_BR).style(Style::new().fg(mode_color).bg(file_bg)),
            );

        // Filename segment
        if let Some(name) = filename {
            let file_segment = format!(" {} ", name);
            statusline = statusline.start(
                Span::from(file_segment).style(Style::new().fg(Color::White).bg(file_bg)),
                Span::from(SLANT_TL_BR).style(Style::new().fg(file_bg)),
            );
        }

        // Calculate git stats start position
        let git_stats_start_x = {
            let mode_len = mode_text.len() as u16 + 1; // +1 for slant
            let file_len = filename.map(|n| n.len() + 2).unwrap_or(0) as u16 + 1; // +2 for spaces, +1 for slant
            area.x + mode_len + file_len
        };

        // Position segment (right)
        statusline = statusline.end(
            Span::from(position_text).style(Style::new().fg(Color::Black).bg(position_bg)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(position_bg)),
        );

        // Render the statusline base
        ratatui::widgets::Widget::render(statusline, area, buf);

        // Now render git stats with colored icons (no background)
        // Icons from lvim: LineAdded (U+EADC), LineModified (U+EADE), LineRemoved (U+EADF)
        if let Some(stats) = &self.git_stats {
            let green = Style::new().fg(Color::Rgb(152, 195, 121));  // green for adds
            let yellow = Style::new().fg(Color::Rgb(229, 192, 123)); // yellow for modified
            let red = Style::new().fg(Color::Rgb(224, 108, 117));    // red for deletions
            let dim = Style::new().fg(Color::Rgb(92, 99, 112));      // dim for separators

            let mut x = git_stats_start_x;

            // Add margin after filename
            buf.set_string(x, area.y, "  ", dim);
            x += 2;

            // Added: icon space number space
            let add_icon = "\u{EADC}";
            let add_num = format!("{}", stats.additions);
            buf.set_string(x, area.y, add_icon, green);
            x += add_icon.width() as u16;
            buf.set_string(x, area.y, " ", green);
            x += 1;
            buf.set_string(x, area.y, &add_num, green);
            x += add_num.width() as u16;
            buf.set_string(x, area.y, " ", dim);
            x += 1;

            // Modified: icon space number space
            let mod_icon = "\u{EADE}";
            let mod_num = format!("{}", stats.modified);
            buf.set_string(x, area.y, mod_icon, yellow);
            x += mod_icon.width() as u16;
            buf.set_string(x, area.y, " ", yellow);
            x += 1;
            buf.set_string(x, area.y, &mod_num, yellow);
            x += mod_num.width() as u16;
            buf.set_string(x, area.y, " ", dim);
            x += 1;

            // Removed: icon space number
            let del_icon = "\u{EADF}";
            let del_num = format!("{}", stats.deletions);
            buf.set_string(x, area.y, del_icon, red);
            x += del_icon.width() as u16;
            buf.set_string(x, area.y, " ", red);
            x += 1;
            buf.set_string(x, area.y, &del_num, red);
        }
    }
}
