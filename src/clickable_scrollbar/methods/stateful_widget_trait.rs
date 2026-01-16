//! Extension trait for rendering.
//!
//! This module defines the trait that extends [`ClickableScrollbar`] with
/// rendering capabilities via the StatefulWidget pattern.
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

/// Extension trait providing [`StatefulWidget`] implementation for [`ClickableScrollbar`].
pub trait ClickableScrollbarStatefulWidgetExt {
    /// The state type required by the widget
    type State;

    /// Renders the widget to the buffer with the given state.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the widget in
    /// * `buf` - The buffer to render to
    /// * `state` - The mutable state for the widget
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
