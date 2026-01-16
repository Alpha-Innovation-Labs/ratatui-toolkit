use ratatui::style::Modifier;

use crate::markdown_renderer::render_markdown;

fn find_line_with_icon<'a>(
    text: &'a ratatui::text::Text<'a>,
    icon: &str,
) -> Option<&'a ratatui::text::Line<'a>> {
    text.lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.starts_with(icon))
}

#[test]
fn test_heading_background_extends_full_width() {
    let markdown = "# Short";
    let text = render_markdown(markdown, Some(200));

    let heading_line = find_line_with_icon(&text, "󰲡 ").expect("Should find heading line");

    // Calculate total content length (all spans)
    let total_len: usize = heading_line.spans.iter().map(|s| s.content.len()).sum();
    assert!(
        total_len >= 100,
        "Heading should be padded for full width, got length: {}",
        total_len
    );
}

#[test]
fn test_bold_text_in_heading() {
    let markdown = "# This is **bold**";
    let text = render_markdown(markdown, Some(200));

    let heading_line = find_line_with_icon(&text, "󰲡 ").expect("Should find heading with icon");

    // The icon span should be bold
    assert!(heading_line.spans[0]
        .style
        .add_modifier
        .contains(Modifier::BOLD));
}
