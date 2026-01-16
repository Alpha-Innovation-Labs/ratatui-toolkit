use ratatui::style::Style;

use crate::file_system_tree::FileSystemTreeConfig;

impl FileSystemTreeConfig {
    pub fn with_show_hidden(mut self, show_hidden: bool) -> Self {
        self.show_hidden = show_hidden;
        self
    }
}
