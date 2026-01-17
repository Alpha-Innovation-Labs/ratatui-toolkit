//! Handle mouse events for the markdown widget.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use super::super::super::helpers::{handle_click, is_in_area};
use super::super::super::markdown_event::MarkdownEvent;
use super::super::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Handle a mouse event for scrolling and click interactions.
    ///
    /// Returns a `MarkdownEvent` indicating what action was taken.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event
    /// * `area` - The area the widget occupies (for bounds checking)
    /// * `content` - The markdown content (for click handling)
    ///
    /// Note: For selection/copy functionality, use `handle_mouse_event_with_selection`
    /// from the standalone functions with a separate SelectionState.
    pub fn handle_mouse_event(
        &mut self,
        event: &MouseEvent,
        area: Rect,
        content: &str,
    ) -> MarkdownEvent {
        if !is_in_area(event.column, event.row, area) {
            return MarkdownEvent::None;
        }

        let relative_y = event.row.saturating_sub(area.y) as usize;
        let relative_x = event.column.saturating_sub(area.x) as usize;
        let width = area.width as usize;

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Set current line based on click position (1-indexed)
                let clicked_line = self.scroll.scroll_offset + relative_y + 1;
                if clicked_line <= self.scroll.total_lines {
                    self.scroll.current_line = clicked_line;
                }

                // Handle click for collapse/expand
                handle_click(relative_x, relative_y, width, content, self.scroll);
                MarkdownEvent::None
            }
            MouseEventKind::ScrollUp => {
                self.scroll.scroll_up(5);
                MarkdownEvent::None
            }
            MouseEventKind::ScrollDown => {
                self.scroll.scroll_down(5);
                MarkdownEvent::None
            }
            _ => MarkdownEvent::None,
        }
    }
}
