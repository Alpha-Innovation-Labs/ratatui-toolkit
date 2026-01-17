//! Handle keyboard events for the markdown widget.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::super::markdown_event::MarkdownEvent;
use super::super::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Handle a keyboard event for navigation.
    ///
    /// Returns a `MarkdownEvent` indicating what action was taken.
    ///
    /// # Handled Keys
    ///
    /// - `j` / `Down`: Move highlighted line down (no scroll)
    /// - `k` / `Up`: Move highlighted line up (no scroll)
    /// - `PageDown`: Scroll down by viewport height
    /// - `PageUp`: Scroll up by viewport height
    /// - `Home` / `g`: Go to top
    /// - `End` / `G`: Go to bottom
    ///
    /// Note: Selection-related keys (Esc, y, Ctrl+Shift+C) should be handled
    /// by the parent application that manages the SelectionState.
    pub fn handle_key_event(&mut self, key: KeyEvent) -> MarkdownEvent {
        // Handle navigation keys
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                // Move current line down (no scroll)
                if self.scroll.current_line < self.scroll.total_lines {
                    self.scroll.current_line += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                // Move current line up (no scroll)
                if self.scroll.current_line > 1 {
                    self.scroll.current_line -= 1;
                }
            }
            KeyCode::PageDown => {
                self.scroll.scroll_down(self.scroll.viewport_height);
            }
            KeyCode::PageUp => {
                self.scroll.scroll_up(self.scroll.viewport_height);
            }
            KeyCode::Home | KeyCode::Char('g') => {
                self.scroll.scroll_to_top();
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.scroll.scroll_to_bottom();
            }
            _ => {}
        }

        MarkdownEvent::None
    }
}
