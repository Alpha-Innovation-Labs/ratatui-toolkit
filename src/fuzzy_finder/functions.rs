use ratatui::layout::Rect;

/// Calculate a centered rectangle within a given area
///
/// # Arguments
///
/// * `percent_x` - Width percentage (0-100)
/// * `percent_y` - Height percentage (0-100)
/// * `r` - The parent rectangle
///
/// # Returns
///
/// A centered rectangle with the specified dimensions
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width.saturating_mul(percent_x) / 100;
    let popup_height = r.height.saturating_mul(percent_y) / 100;

    let popup_x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let popup_y = r.y + (r.height.saturating_sub(popup_height)) / 2;

    Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    }
}
