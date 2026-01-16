use ratatui::style::Color;

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
fn test_h1_has_correct_colors() {
    let markdown = "# Test";
    let text = render_markdown(markdown, Some(200));

    let heading_line = find_line_with_icon(&text, "󰲡 ").expect("Should find heading line");

    let style = heading_line.spans[0].style;
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

    let heading_line = find_line_with_icon(&text, "󰲣 ").expect("Should find heading line");

    let style = heading_line.spans[0].style;
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

    let heading_line = find_line_with_icon(&text, "󰲥 ").expect("Should find heading line");

    let style = heading_line.spans[0].style;
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
