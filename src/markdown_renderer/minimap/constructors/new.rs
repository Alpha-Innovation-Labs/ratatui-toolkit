//! Constructor for Minimap.

use ratatui::style::{Color, Style};

use super::super::{Minimap, MinimapConfig};

impl Default for MinimapConfig {
    fn default() -> Self {
        Self {
            width: 10,
            text_style: Style::default().fg(Color::Rgb(88, 88, 88)),
            viewport_style: Style::default().fg(Color::Rgb(97, 175, 239)).bg(Color::Rgb(40, 44, 52)),
            background_style: Style::default().bg(Color::Rgb(30, 30, 30)),
            show_density: true,
        }
    }
}

impl<'a> Minimap<'a> {
    /// Create a new Minimap with the given content.
    ///
    /// # Arguments
    ///
    /// * `content` - The text content to render as a minimap
    ///
    /// # Returns
    ///
    /// A new `Minimap` instance with default configuration.
    pub fn new(content: &'a str) -> Self {
        let total_lines = content.lines().count();
        Self {
            content,
            width: 10,
            viewport_start: 0,
            viewport_end: 0,
            total_lines,
            config: MinimapConfig::default(),
        }
    }

    /// Set the width of the minimap in characters.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in terminal columns
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self.config.width = width;
        self
    }

    /// Set the current viewport position.
    ///
    /// # Arguments
    ///
    /// * `start` - First visible line (0-indexed)
    /// * `end` - Last visible line (0-indexed)
    /// * `total` - Total number of lines in the document
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn viewport(mut self, start: usize, end: usize, total: usize) -> Self {
        self.viewport_start = start;
        self.viewport_end = end;
        self.total_lines = total;
        self
    }

    /// Set the style for minimap text (Braille characters).
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to minimap text
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn text_style(mut self, style: Style) -> Self {
        self.config.text_style = style;
        self
    }

    /// Set the style for the viewport indicator.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the viewport region
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn viewport_style(mut self, style: Style) -> Self {
        self.config.viewport_style = style;
        self
    }

    /// Set the configuration for the minimap.
    ///
    /// # Arguments
    ///
    /// * `config` - The minimap configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn config(mut self, config: MinimapConfig) -> Self {
        self.config = config;
        self
    }
}
