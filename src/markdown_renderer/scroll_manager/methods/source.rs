//! Source management methods for MarkdownScrollManager.

use std::path::Path;

use crate::markdown_renderer::markdown_source::MarkdownSource;

use super::super::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Set a string-based markdown source.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content string.
    pub fn set_source_string(&mut self, content: impl Into<String>) {
        self.source = Some(MarkdownSource::from_string(content));
        self.invalidate_cache();
    }

    /// Set a file-based markdown source.
    ///
    /// This loads the file content and enables auto-reload support.
    /// Use `reload_source()` to check for and apply file changes.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the markdown file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn set_source_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        self.source = Some(MarkdownSource::from_file(path)?);
        self.invalidate_cache();
        Ok(())
    }

    /// Get the current content from the source.
    ///
    /// # Returns
    ///
    /// The markdown content, or `None` if no source is set.
    pub fn content(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.content())
    }

    /// Check if the source is file-based.
    ///
    /// # Returns
    ///
    /// `true` if the source is loaded from a file, `false` otherwise.
    pub fn is_file_source(&self) -> bool {
        self.source.as_ref().map(|s| s.is_file()).unwrap_or(false)
    }

    /// Get the file path if this is a file-based source.
    ///
    /// # Returns
    ///
    /// The file path, or `None` if this is a string source or no source is set.
    pub fn source_path(&self) -> Option<&Path> {
        self.source.as_ref().and_then(|s| s.path())
    }

    /// Reload the source content from disk (for file-based sources).
    ///
    /// This re-reads the file and invalidates caches if the content changed.
    /// For string-based sources, this is a no-op.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Content changed and caches were invalidated.
    /// * `Ok(false)` - Content unchanged or source is string-based.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn reload_source(&mut self) -> std::io::Result<bool> {
        if let Some(ref mut source) = self.source {
            let changed = source.reload()?;
            if changed {
                self.invalidate_cache();
            }
            Ok(changed)
        } else {
            Ok(false)
        }
    }
}
