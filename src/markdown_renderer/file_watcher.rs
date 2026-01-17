//! File watching for markdown auto-reload.
//!
//! Provides a non-blocking file watcher that can detect changes to markdown
//! files and notify the application to reload content.

use notify::{
    event::ModifyKind, Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::time::Duration;

/// A file watcher for detecting changes to markdown files.
///
/// Uses the `notify` crate to watch files for modifications and provides
/// a non-blocking interface to check for changes in an event loop.
///
/// # Example
/// ```no_run
/// use ratatui_toolkit::markdown_renderer::{MarkdownFileWatcher, MarkdownSource};
/// use std::path::Path;
///
/// let mut source = MarkdownSource::from_file("README.md").unwrap();
/// let mut watcher = MarkdownFileWatcher::new().unwrap();
/// watcher.watch(source.path().unwrap()).unwrap();
///
/// // In your event loop:
/// loop {
///     if watcher.check_for_changes() {
///         if source.reload().unwrap() {
///             // Content changed, update your UI
///         }
///     }
///     // ... rest of your event loop
///     # break;
/// }
/// ```
pub struct MarkdownFileWatcher {
    watcher: RecommendedWatcher,
    rx: Receiver<Result<Event, notify::Error>>,
}

impl MarkdownFileWatcher {
    /// Create a new file watcher.
    ///
    /// # Errors
    /// Returns a `notify::Error` if the watcher cannot be created.
    pub fn new() -> Result<Self, notify::Error> {
        let (tx, rx) = channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_millis(100)),
        )?;

        Ok(Self { watcher, rx })
    }

    /// Start watching a file for changes.
    ///
    /// # Arguments
    /// * `path` - Path to the file to watch.
    ///
    /// # Errors
    /// Returns a `notify::Error` if the path cannot be watched.
    pub fn watch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.watch(path, RecursiveMode::NonRecursive)
    }

    /// Stop watching a file.
    ///
    /// # Arguments
    /// * `path` - Path to the file to stop watching.
    ///
    /// # Errors
    /// Returns a `notify::Error` if the path cannot be unwatched.
    pub fn unwatch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.unwatch(path)
    }

    /// Check if there are any pending file change events.
    ///
    /// This is a non-blocking operation that returns `true` if any
    /// relevant file modifications have been detected since the last check.
    ///
    /// # Returns
    /// `true` if file changes were detected, `false` otherwise.
    pub fn check_for_changes(&self) -> bool {
        let mut has_changes = false;

        loop {
            match self.rx.try_recv() {
                Ok(Ok(event)) => {
                    if Self::is_relevant_event(&event) {
                        has_changes = true;
                    }
                }
                Ok(Err(_)) => {
                    // Watcher error, ignore
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        has_changes
    }

    /// Check if an event is relevant for triggering a reload.
    ///
    /// Filters for data modification events (content changes) and ignores
    /// metadata-only changes.
    fn is_relevant_event(event: &Event) -> bool {
        matches!(
            event.kind,
            EventKind::Modify(ModifyKind::Data(_))
                | EventKind::Modify(ModifyKind::Any)
                | EventKind::Create(_)
                | EventKind::Remove(_)
        )
    }

    /// Drain all pending events without processing them.
    ///
    /// Useful when you want to clear the event queue without triggering
    /// any reloads (e.g., after programmatic file updates).
    pub fn drain_events(&self) {
        while self.rx.try_recv().is_ok() {}
    }
}

impl std::fmt::Debug for MarkdownFileWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownFileWatcher")
            .field("watcher", &"RecommendedWatcher")
            .field("rx", &"Receiver<...>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::thread;
    use std::time::Duration;
    use tempfile::NamedTempFile;

    #[test]
    fn test_watcher_creation() {
        let watcher = MarkdownFileWatcher::new();
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_watch_file() {
        let temp = NamedTempFile::new().unwrap();
        let mut watcher = MarkdownFileWatcher::new().unwrap();
        assert!(watcher.watch(temp.path()).is_ok());
    }

    #[test]
    fn test_unwatch_file() {
        let temp = NamedTempFile::new().unwrap();
        let mut watcher = MarkdownFileWatcher::new().unwrap();
        watcher.watch(temp.path()).unwrap();
        assert!(watcher.unwatch(temp.path()).is_ok());
    }

    #[test]
    fn test_check_for_changes_empty() {
        let temp = NamedTempFile::new().unwrap();
        let mut watcher = MarkdownFileWatcher::new().unwrap();
        watcher.watch(temp.path()).unwrap();

        // No changes yet
        assert!(!watcher.check_for_changes());
    }

    #[test]
    fn test_detect_file_modification() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "initial content").unwrap();

        let mut watcher = MarkdownFileWatcher::new().unwrap();
        watcher.watch(temp.path()).unwrap();

        // Modify the file
        thread::sleep(Duration::from_millis(50));
        let mut file = std::fs::File::create(temp.path()).unwrap();
        writeln!(file, "modified content").unwrap();
        file.sync_all().unwrap();

        // Give the watcher time to detect the change
        thread::sleep(Duration::from_millis(200));

        // Should detect the change
        assert!(watcher.check_for_changes());
    }

    #[test]
    fn test_drain_events() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "initial").unwrap();

        let mut watcher = MarkdownFileWatcher::new().unwrap();
        watcher.watch(temp.path()).unwrap();

        // Modify the file
        thread::sleep(Duration::from_millis(50));
        let mut file = std::fs::File::create(temp.path()).unwrap();
        writeln!(file, "modified").unwrap();
        file.sync_all().unwrap();

        thread::sleep(Duration::from_millis(200));

        // Drain events
        watcher.drain_events();

        // Should be empty now
        assert!(!watcher.check_for_changes());
    }

    #[test]
    fn test_debug_impl() {
        let watcher = MarkdownFileWatcher::new().unwrap();
        let debug_str = format!("{:?}", watcher);
        assert!(debug_str.contains("MarkdownFileWatcher"));
    }
}
