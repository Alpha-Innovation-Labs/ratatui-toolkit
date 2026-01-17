//! Git stats setter for MarkdownWidget.

use super::super::{GitStats, MarkdownWidget};

impl<'a> MarkdownWidget<'a> {
    /// Set the git statistics to display in the statusline.
    ///
    /// # Arguments
    ///
    /// * `stats` - The git statistics (additions, modified, deletions)
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn git_stats(mut self, stats: GitStats) -> Self {
        self.git_stats = Some(stats);
        self
    }

    /// Set the git statistics from a tuple (additions, modified, deletions).
    ///
    /// # Arguments
    ///
    /// * `additions` - Lines added
    /// * `modified` - Files/lines modified
    /// * `deletions` - Lines deleted
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn git_stats_tuple(mut self, additions: usize, modified: usize, deletions: usize) -> Self {
        self.git_stats = Some(GitStats {
            additions,
            modified,
            deletions,
        });
        self
    }
}
