//! Set mode method for MarkdownWidget.

use super::super::{MarkdownWidget, MarkdownWidgetMode};

impl<'a> MarkdownWidget<'a> {
    /// Set the current mode for the statusline.
    ///
    /// # Arguments
    ///
    /// * `mode` - The mode to display (Normal or Drag)
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn mode(mut self, mode: MarkdownWidgetMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set whether to show the statusline.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the statusline
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_statusline(mut self, show: bool) -> Self {
        self.show_statusline = show;
        self
    }
}
