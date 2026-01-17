//! Settings methods for MarkdownScrollManager.

use crate::markdown_renderer::styled_line::CodeBlockTheme;

use super::super::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Enable or disable line numbers in code blocks.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show line numbers.
    pub fn set_show_line_numbers(&mut self, show: bool) {
        if self.show_line_numbers != show {
            self.show_line_numbers = show;
            self.invalidate_cache();
        }
    }

    /// Enable or disable line numbers for the entire document.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show document line numbers.
    pub fn set_show_document_line_numbers(&mut self, show: bool) {
        if self.show_document_line_numbers != show {
            self.show_document_line_numbers = show;
            self.invalidate_cache();
        }
    }

    /// Set the code block color theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - The theme to use for code blocks.
    pub fn set_code_block_theme(&mut self, theme: CodeBlockTheme) {
        if self.code_block_theme != theme {
            self.code_block_theme = theme;
            self.invalidate_cache();
        }
    }
}
