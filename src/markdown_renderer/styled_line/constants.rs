//! Constants for markdown rendering styling.
//!
//! Contains icons, markers, and color constants used in markdown rendering.

use ratatui::style::Color;

/// Nerd Font heading icons by level.
pub const HEADING_ICONS: [&str; 6] = [
    "󰲡 ", // H1
    "󰲣 ", // H2
    "󰲥 ", // H3
    "󰲧 ", // H4
    "󰲩 ", // H5
    "󰲫 ", // H6
];

/// Bullet markers that cycle by nesting level.
pub const BULLET_MARKERS: [&str; 4] = ["● ", "○ ", "◆ ", "◇ "];

/// Language icons for code blocks.
pub fn get_language_icon(lang: &str) -> &'static str {
    match lang.to_lowercase().as_str() {
        "rust" | "rs" => " ",
        "python" | "py" => "󰌠 ",
        "javascript" | "js" => " ",
        "typescript" | "ts" => " ",
        "bash" | "sh" | "shell" | "zsh" => "󰈮 ",
        "json" => " ",
        "yaml" | "yml" => " ",
        "toml" => " ",
        "html" => " ",
        "css" => " ",
        "markdown" | "md" => " ",
        "sql" => " ",
        "go" => " ",
        "java" => " ",
        "c" => " ",
        "cpp" | "c++" => " ",
        "ruby" | "rb" => " ",
        "lua" => " ",
        "vim" => " ",
        "docker" | "dockerfile" => " ",
        "git" => " ",
        _ => "󰈙 ",
    }
}

/// Background colors for heading levels.
pub fn heading_bg_color(level: u8) -> Color {
    match level {
        1 => Color::Rgb(80, 40, 80), // Purple-ish
        2 => Color::Rgb(40, 60, 80), // Blue-ish
        3 => Color::Rgb(40, 80, 60), // Green-ish
        4 => Color::Rgb(80, 60, 40), // Orange-ish
        5 => Color::Rgb(60, 60, 60), // Gray
        6 => Color::Rgb(50, 50, 50), // Darker gray
        _ => Color::Rgb(50, 50, 50),
    }
}

/// Foreground colors for heading levels.
pub fn heading_fg_color(level: u8) -> Color {
    match level {
        1 => Color::Rgb(255, 180, 255), // Bright magenta
        2 => Color::Rgb(130, 180, 255), // Bright blue
        3 => Color::Rgb(130, 255, 180), // Bright cyan
        4 => Color::Rgb(255, 200, 130), // Bright orange
        5 => Color::Rgb(200, 200, 200), // Light gray
        6 => Color::Rgb(170, 170, 170), // Gray
        _ => Color::White,
    }
}
