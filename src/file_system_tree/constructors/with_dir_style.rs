use ratatui::style::Style;

use crate::file_system_tree::FileSystemTreeConfig;

impl FileSystemTreeConfig {
    pub fn with_dir_style(mut self, style: Style) -> Self {
        self.dir_style = style;
        self
    }
}
