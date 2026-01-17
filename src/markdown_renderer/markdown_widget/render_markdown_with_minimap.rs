//! Render markdown with optional minimap.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::Widget,
};

use crate::markdown_renderer::minimap::{Minimap, MinimapConfig};
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::render_markdown_interactive_with_selection::render_markdown_interactive_with_selection;
use super::selection_state::SelectionState;

/// Options for rendering markdown with minimap.
#[derive(Debug, Clone)]
pub struct MarkdownRenderOptions {
    /// Whether to show the minimap.
    pub show_minimap: bool,
    /// Minimap configuration.
    pub minimap_config: MinimapConfig,
}

impl Default for MarkdownRenderOptions {
    fn default() -> Self {
        Self {
            show_minimap: false,
            minimap_config: MinimapConfig::default(),
        }
    }
}

impl MarkdownRenderOptions {
    /// Create new options with minimap enabled.
    pub fn with_minimap() -> Self {
        Self {
            show_minimap: true,
            minimap_config: MinimapConfig::default(),
        }
    }

    /// Set minimap visibility.
    pub fn show_minimap(mut self, show: bool) -> Self {
        self.show_minimap = show;
        self
    }

    /// Set minimap width.
    pub fn minimap_width(mut self, width: u16) -> Self {
        self.minimap_config.width = width;
        self
    }

    /// Set minimap configuration.
    pub fn minimap_config(mut self, config: MinimapConfig) -> Self {
        self.minimap_config = config;
        self
    }
}

/// Render markdown with selection and optional minimap directly to buffer.
///
/// This function handles the complete rendering including:
/// - Markdown content with selection highlighting
/// - Optional minimap on the right side
///
/// # Arguments
///
/// * `content` - The markdown content to render
/// * `scroll` - The scroll manager
/// * `area` - The area to render into
/// * `buf` - The buffer to render to
/// * `is_resizing` - Whether the widget is being resized
/// * `selection` - The selection state
/// * `options` - Render options including minimap settings
///
/// # Returns
///
/// All rendered lines for selection text extraction.
pub fn render_markdown_with_minimap(
    content: &str,
    scroll: &mut MarkdownScrollManager,
    area: Rect,
    buf: &mut Buffer,
    is_resizing: bool,
    selection: &SelectionState,
    options: &MarkdownRenderOptions,
) -> Vec<Line<'static>> {
    // Calculate areas based on minimap option
    let minimap_width = options.minimap_config.width;
    let (content_area, minimap_area) = if options.show_minimap && area.width > minimap_width + 10 {
        (
            Rect {
                width: area.width.saturating_sub(minimap_width + 1),
                ..area
            },
            Some(Rect {
                x: area.x + area.width.saturating_sub(minimap_width),
                width: minimap_width,
                ..area
            }),
        )
    } else {
        (area, None)
    };

    // Update viewport height for scroll calculations
    scroll.update_viewport(content_area);

    // Render markdown content
    let (text, all_lines) = render_markdown_interactive_with_selection(
        content,
        scroll,
        content_area,
        is_resizing,
        selection,
    );

    // Render content to buffer
    for (i, line) in text.lines.iter().enumerate() {
        if i >= content_area.height as usize {
            break;
        }

        let y = content_area.y + i as u16;
        let mut x = content_area.x;

        for span in line.spans.iter() {
            let span_width = span.content.chars().count() as u16;
            if x.saturating_sub(content_area.x) < content_area.width {
                buf.set_string(x, y, &span.content, span.style);
                x = x.saturating_add(span_width);
            }
        }
    }

    // Render minimap if enabled
    if let Some(mm_area) = minimap_area {
        let viewport_start = scroll.scroll_offset;
        let viewport_end = viewport_start + content_area.height as usize;
        let total_lines = scroll.total_lines;

        let minimap = Minimap::new(content)
            .width(mm_area.width)
            .viewport(viewport_start, viewport_end, total_lines)
            .config(options.minimap_config.clone());

        minimap.render(mm_area, buf);
    }

    all_lines
}
