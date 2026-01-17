use ratatui::style::Modifier;

use crate::markdown_renderer::render_markdown;

fn find_line_with_icon<'a>(
    text: &'a ratatui::text::Text<'a>,
    icon: &str,
) -> Option<&'a ratatui::text::Line<'a>> {
    text.lines.iter().find(|line| {
        // Icon is now in span[2] (span[0] is indent, span[1] is collapse indicator)
        line.spans.len() > 2 && line.spans[2].content.contains(icon)
    })
}

#[test]
fn test_heading_background_extends_full_width() {
    let markdown = "# Short";
    let text = render_markdown(markdown, Some(200));

    let heading_line = find_line_with_icon(&text, "󰲡").expect("Should find heading line");

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

    let heading_line = find_line_with_icon(&text, "󰲡").expect("Should find heading with icon");

    // Span[2] is the icon span which should be bold
    assert!(heading_line.spans[2]
        .style
        .add_modifier
        .contains(Modifier::BOLD));
}
