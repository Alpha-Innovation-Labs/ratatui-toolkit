//! Markdown content source abstraction.
//!
//! Provides a unified interface for loading markdown content from either
//! a string or a file path, with support for auto-reload when the file changes.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Represents the source of markdown content.
///
/// This enum provides a unified interface for working with markdown content
/// that can come from either a static string or a file on disk.
#[derive(Debug, Clone)]
pub enum MarkdownSource {
    /// Static string content (no auto-reload support).
    String(String),

    /// File-based content with auto-reload support.
    File {
        /// Path to the markdown file.
        path: PathBuf,
        /// Cached content from the file.
        content: String,
    },
}

impl MarkdownSource {
    /// Create a new `MarkdownSource` from a string.
    ///
    /// # Arguments
    /// * `s` - The markdown string content.
    ///
    /// # Example
    /// ```
    /// use ratatui_toolkit::markdown_renderer::MarkdownSource;
    ///
    /// let source = MarkdownSource::from_string("# Hello World");
    /// assert_eq!(source.content(), "# Hello World");
    /// ```
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }

    /// Create a new `MarkdownSource` from a file path.
    ///
    /// Reads the file content immediately and caches it.
    ///
    /// # Arguments
    /// * `path` - Path to the markdown file.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file cannot be read.
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_toolkit::markdown_renderer::MarkdownSource;
    ///
    /// let source = MarkdownSource::from_file("README.md").unwrap();
    /// println!("Content: {}", source.content());
    /// ```
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path)?;
        Ok(Self::File { path, content })
    }

    /// Get the current content of the markdown source.
    ///
    /// For string sources, this returns the original string.
    /// For file sources, this returns the cached content.
    pub fn content(&self) -> &str {
        match self {
            Self::String(s) => s,
            Self::File { content, .. } => content,
        }
    }

    /// Get the file path if this is a file-based source.
    ///
    /// Returns `None` for string sources.
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::String(_) => None,
            Self::File { path, .. } => Some(path),
        }
    }

    /// Check if this source is file-based.
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    /// Check if this source is string-based.
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Reload the content from the file.
    ///
    /// For string sources, this is a no-op and returns `Ok(false)`.
    /// For file sources, this re-reads the file and returns `Ok(true)` if
    /// the content has changed.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file cannot be read.
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_toolkit::markdown_renderer::MarkdownSource;
    ///
    /// let mut source = MarkdownSource::from_file("README.md").unwrap();
    /// // ... file is modified externally ...
    /// if source.reload().unwrap() {
    ///     println!("Content changed!");
    /// }
    /// ```
    pub fn reload(&mut self) -> io::Result<bool> {
        match self {
            Self::String(_) => Ok(false),
            Self::File { path, content } => {
                let new_content = fs::read_to_string(&*path)?;
                if new_content != *content {
                    *content = new_content;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    /// Set the content directly (for string sources).
    ///
    /// This is useful for updating string-based sources programmatically.
    /// For file sources, this updates the cached content but does not write to disk.
    ///
    /// Returns `true` if the content was changed.
    pub fn set_content(&mut self, new_content: impl Into<String>) -> bool {
        let new_content = new_content.into();
        match self {
            Self::String(content) => {
                if *content != new_content {
                    *content = new_content;
                    true
                } else {
                    false
                }
            }
            Self::File { content, .. } => {
                if *content != new_content {
                    *content = new_content;
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Default for MarkdownSource {
    fn default() -> Self {
        Self::String(String::new())
    }
}

impl From<String> for MarkdownSource {
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}

impl From<&str> for MarkdownSource {
    fn from(s: &str) -> Self {
        Self::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_string_source() {
        let source = MarkdownSource::from_string("# Hello");
        assert_eq!(source.content(), "# Hello");
        assert!(source.is_string());
        assert!(!source.is_file());
        assert!(source.path().is_none());
    }

    #[test]
    fn test_file_source() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "# From File").unwrap();

        let source = MarkdownSource::from_file(temp.path()).unwrap();
        assert!(source.content().contains("# From File"));
        assert!(source.is_file());
        assert!(!source.is_string());
        assert!(source.path().is_some());
    }

    #[test]
    fn test_reload() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "# Original").unwrap();

        let mut source = MarkdownSource::from_file(temp.path()).unwrap();
        assert!(source.content().contains("# Original"));

        // Overwrite file content
        temp.reopen().unwrap();
        let mut file = std::fs::File::create(temp.path()).unwrap();
        writeln!(file, "# Modified").unwrap();

        // Reload and verify
        assert!(source.reload().unwrap());
        assert!(source.content().contains("# Modified"));
    }

    #[test]
    fn test_reload_no_change() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "# Same").unwrap();

        let mut source = MarkdownSource::from_file(temp.path()).unwrap();
        assert!(!source.reload().unwrap()); // No change
    }

    #[test]
    fn test_string_reload_noop() {
        let mut source = MarkdownSource::from_string("# Static");
        assert!(!source.reload().unwrap());
    }

    #[test]
    fn test_set_content() {
        let mut source = MarkdownSource::from_string("# Old");
        assert!(source.set_content("# New"));
        assert_eq!(source.content(), "# New");
        assert!(!source.set_content("# New")); // No change
    }

    #[test]
    fn test_from_impls() {
        let source: MarkdownSource = "# Hello".into();
        assert_eq!(source.content(), "# Hello");

        let source: MarkdownSource = String::from("# World").into();
        assert_eq!(source.content(), "# World");
    }
}
