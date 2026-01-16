use ratatui::style::Style;

use crate::file_system_tree::FileSystemTreeConfig;

impl FileSystemTreeConfig {
    pub fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }
}
