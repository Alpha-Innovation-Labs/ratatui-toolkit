//! Selection state for markdown widget text selection and copy.

mod constructors;
mod methods;

pub use constructors::*;
pub use methods::*;

/// Position in the rendered text (visual coordinates).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectionPos {
    /// X coordinate (column).
    pub x: i32,
    /// Y coordinate (row, relative to document start, not screen).
    pub y: i32,
}

impl SelectionPos {
    /// Create a new position.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Selection state for markdown widget.
///
/// Tracks whether selection mode is active and the selection bounds.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// Whether selection mode is active.
    pub active: bool,
    /// Selection anchor (start point).
    pub anchor: Option<SelectionPos>,
    /// Current cursor/end position.
    pub cursor: Option<SelectionPos>,
    /// Cached rendered lines for stable selection.
    pub frozen_lines: Option<Vec<ratatui::text::Line<'static>>>,
    /// Width when lines were frozen.
    pub frozen_width: usize,
}
