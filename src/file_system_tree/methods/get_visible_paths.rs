use crate::file_system_tree::{FileSystemEntry, FileSystemTree};
use crate::tree_view::{TreeNode, TreeViewState};

impl<'a> FileSystemTree<'a> {
    fn get_visible_paths(&self, state: &TreeViewState) -> Vec<Vec<usize>> {
        let mut paths = Vec::new();

        fn traverse(
            nodes: &[TreeNode<FileSystemEntry>],
            current_path: Vec<usize>,
            state: &TreeViewState,
            paths: &mut Vec<Vec<usize>>,
        ) {
            for (idx, node) in nodes.iter().enumerate() {
                let mut path = current_path.clone();
                path.push(idx);
                paths.push(path.clone());

                if state.is_expanded(&path) && !node.children.is_empty() {
                    traverse(&node.children, path, state, paths);
                }
            }
        }

        traverse(&self.nodes, Vec::new(), state, &mut paths);
        paths
    }
}
