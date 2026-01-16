use crate::file_system_tree::{FileSystemEntry, FileSystemTree};
use crate::tree_view::TreeViewState;

impl<'a> FileSystemTree<'a> {
    pub fn get_selected_entry(&self, state: &TreeViewState) -> Option<FileSystemEntry> {
        if let Some(path) = &state.selected_path {
            self.get_entry_at_path(path)
        } else {
            None
        }
    }
}
