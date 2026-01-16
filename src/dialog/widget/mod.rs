//! Mutable widget variant

use crate::dialog::Dialog;

/// A mutable wrapper around Dialog for widget rendering
///
/// This struct provides a mutable reference to Dialog for rendering purposes.
pub struct DialogWidget<'a> {
    /// Mutable reference to the dialog
    dialog: &'a mut Dialog<'a>,
}
