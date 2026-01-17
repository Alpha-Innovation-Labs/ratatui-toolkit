use ratatui::style::Color;

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
fn test_h1_has_correct_colors() {
    let markdown = "# Test";
    let text = render_markdown(markdown, Some(200));

    let heading_line = find_line_with_icon(&text, "󰲡").expect("Should find heading line");

    // Span[2] is the icon span with heading style
    let style = heading_line.spans[2].style;
    assert_eq!(
        style.fg,
        Some(Color::Rgb(255, 180, 255)),
        "H1 foreground should be bright magenta"
    );
    assert_eq!(
        style.bg,
        Some(Color::Rgb(80, 40, 80)),
        "H1 background should be purple"
    );
}

#[test]
fn test_h2_has_correct_colors() {
    let markdown = "## Test";
    let text = render_markdown(markdown, Some(200));

    let heading_line = find_line_with_icon(&text, "󰲣").expect("Should find heading line");

    // Span[2] is the icon span with heading style
    let style = heading_line.spans[2].style;
    assert_eq!(
        style.fg,
        Some(Color::Rgb(130, 180, 255)),
        "H2 foreground should be bright blue"
    );
    assert_eq!(
        style.bg,
        Some(Color::Rgb(40, 60, 80)),
        "H2 background should be blue-ish"
    );
}

#[test]
fn test_h3_has_correct_colors() {
    let markdown = "### Test";
    let text = render_markdown(markdown, Some(200));

    let heading_line = find_line_with_icon(&text, "󰲥").expect("Should find heading line");

    // Span[2] is the icon span with heading style
    let style = heading_line.spans[2].style;
    assert_eq!(
        style.fg,
        Some(Color::Rgb(130, 255, 180)),
        "H3 foreground should be bright cyan"
    );
    assert_eq!(
        style.bg,
        Some(Color::Rgb(40, 80, 60)),
        "H3 background should be green-ish"
    );
}
