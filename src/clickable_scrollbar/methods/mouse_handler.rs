//! Mouse event handling implementation for [`ClickableScrollbarState`].
//!
//! This module contains the implementation of mouse event handling methods
//! for the scrollbar, including click, drag, and scroll wheel support.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::widgets::ScrollbarOrientation;

use crate::clickable_scrollbar::methods::mouse_handler_trait::ClickableScrollbarStateMouseExt;
use crate::clickable_scrollbar::{ClickableScrollbarState, ScrollbarEvent};

impl ClickableScrollbarStateMouseExt for ClickableScrollbarState {
    fn handle_mouse_event(&mut self, event: &MouseEvent) -> ScrollbarEvent {
        let (col, row) = (event.column, event.row);

        if !self.area.contains((col, row).into()) {
            if self.drag_active {
                self.drag_active = false;
            }
            return ScrollbarEvent::None;
        }

        match event.kind {
            MouseEventKind::ScrollDown => {
                if self.is_vertical() {
                    ScrollbarEvent::Down(self.calculate_scroll_increment())
                } else {
                    ScrollbarEvent::None
                }
            }
            MouseEventKind::ScrollUp => {
                if self.is_vertical() {
                    ScrollbarEvent::Up(self.calculate_scroll_increment())
                } else {
                    ScrollbarEvent::None
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                self.drag_active = true;
                let pos = self.map_position_to_offset(col, row);
                ScrollbarEvent::Position(pos)
            }
            MouseEventKind::Drag(MouseButton::Left) if self.drag_active => {
                let pos = self.map_position_to_offset(col, row);
                ScrollbarEvent::Position(pos)
            }
            MouseEventKind::Up(MouseButton::Left) => {
                self.drag_active = false;
                ScrollbarEvent::None
            }
            _ => ScrollbarEvent::None,
        }
    }
}

impl ClickableScrollbarState {
    pub(crate) fn calculate_scroll_increment(&self) -> usize {
        self.scroll_by
            .unwrap_or_else(|| (self.page_len / 10).max(1))
    }

    fn map_position_to_offset(&self, col: u16, row: u16) -> usize {
        if self.is_vertical() {
            let pos = row.saturating_sub(self.area.y).saturating_sub(1) as usize;
            let span = self.area.height.saturating_sub(2) as usize;

            if span > 0 {
                (self.max_offset * pos) / span
            } else {
                0
            }
        } else {
            let pos = col.saturating_sub(self.area.x).saturating_sub(1) as usize;
            let span = self.area.width.saturating_sub(2) as usize;

            if span > 0 {
                (self.max_offset * pos) / span
            } else {
                0
            }
        }
    }

    fn is_vertical(&self) -> bool {
        matches!(
            self.orientation,
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::MouseEventKind;
    use ratatui::layout::Rect;

    fn make_mouse_event(kind: MouseEventKind, col: u16, row: u16) -> MouseEvent {
        MouseEvent {
            kind,
            column: col,
            row,
            modifiers: crossterm::event::KeyModifiers::empty(),
        }
    }

    fn create_test_state() -> ClickableScrollbarState {
        ClickableScrollbarState {
            area: Rect::new(10, 5, 1, 20),
            orientation: ScrollbarOrientation::VerticalRight,
            offset: 0,
            page_len: 10,
            max_offset: 90,
            scroll_by: None,
            drag_active: false,
        }
    }

    #[test]
    fn test_handle_mouse_event_outside_area() {
        let mut state = create_test_state();
        state.drag_active = true;
        let event = make_mouse_event(MouseEventKind::Up(MouseButton::Left), 0, 0);
        let result = state.handle_mouse_event(&event);
        assert_eq!(result, ScrollbarEvent::None);
        assert!(!state.drag_active);
    }

    #[test]
    fn test_handle_mouse_event_scroll_down_vertical() {
        let mut state = create_test_state();
        let event = make_mouse_event(MouseEventKind::ScrollDown, 10, 10);
        let result = state.handle_mouse_event(&event);
        assert!(matches!(result, ScrollbarEvent::Down(_)));
    }

    #[test]
    fn test_handle_mouse_event_scroll_up_vertical() {
        let mut state = create_test_state();
        let event = make_mouse_event(MouseEventKind::ScrollUp, 10, 10);
        let result = state.handle_mouse_event(&event);
        assert!(matches!(result, ScrollbarEvent::Up(_)));
    }

    #[test]
    fn test_handle_mouse_event_click() {
        let mut state = create_test_state();
        let event = make_mouse_event(MouseEventKind::Down(MouseButton::Left), 10, 10);
        let result = state.handle_mouse_event(&event);
        assert!(matches!(result, ScrollbarEvent::Position(_)));
        assert!(state.drag_active);
    }

    #[test]
    fn test_handle_mouse_event_drag() {
        let mut state = create_test_state();
        state.drag_active = true;
        let event = make_mouse_event(MouseEventKind::Drag(MouseButton::Left), 10, 15);
        let result = state.handle_mouse_event(&event);
        assert!(matches!(result, ScrollbarEvent::Position(_)));
    }

    #[test]
    fn test_handle_mouse_event_drag_not_active() {
        let mut state = create_test_state();
        state.drag_active = false;
        let event = make_mouse_event(MouseEventKind::Drag(MouseButton::Left), 10, 15);
        let result = state.handle_mouse_event(&event);
        assert_eq!(result, ScrollbarEvent::None);
    }

    #[test]
    fn test_handle_mouse_event_release() {
        let mut state = create_test_state();
        state.drag_active = true;
        let event = make_mouse_event(MouseEventKind::Up(MouseButton::Left), 10, 10);
        let result = state.handle_mouse_event(&event);
        assert_eq!(result, ScrollbarEvent::None);
        assert!(!state.drag_active);
    }
}
