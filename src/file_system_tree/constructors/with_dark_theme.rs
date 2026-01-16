use ratatui::style::Style;

use crate::file_system_tree::FileSystemTreeConfig;

impl FileSystemTreeConfig {
    pub fn with_dark_theme(mut self, use_dark: bool) -> Self {
        self.use_dark_theme = use_dark;
        self
    }
}
