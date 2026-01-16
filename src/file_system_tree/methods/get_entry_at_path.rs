use crate::file_system_tree::{FileSystemEntry, FileSystemTree};
use crate::tree_view::TreeNode;

impl<'a> FileSystemTree<'a> {
    fn get_entry_at_path(&self, path: &[usize]) -> Option<FileSystemEntry> {
        fn find_entry(
            nodes: &[TreeNode<FileSystemEntry>],
            path: &[usize],
        ) -> Option<FileSystemEntry> {
            if path.is_empty() {
                return None;
            }

            if path.len() == 1 {
                return nodes.get(path[0]).map(|n| n.data.clone());
            }

            if let Some(node) = nodes.get(path[0]) {
                return find_entry(&node.children, &path[1..]);
            }

            None
        }

        find_entry(&self.nodes, path)
    }
}
