use ratatui::style::Style;

use crate::file_system_tree::FileSystemTreeConfig;

impl FileSystemTreeConfig {
    pub fn with_file_style(mut self, style: Style) -> Self {
        self.file_style = style;
        self
    }
}
