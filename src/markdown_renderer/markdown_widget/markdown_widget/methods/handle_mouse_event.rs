//! Handle mouse events for the markdown widget.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::markdown_renderer::render_styled_line;

use super::super::super::helpers::{is_in_area, should_render_line};
use super::super::super::markdown_event::MarkdownEvent;
use super::super::super::selection_state::SelectionPos;
use super::super::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Handle a mouse event for all interactions.
    ///
    /// This method handles:
    /// - Click-to-focus: Sets the current line on click
    /// - Double-click: Returns event with line info
    /// - Text selection: Drag to select, auto-copy on release
    /// - Heading collapse: Click on heading to toggle
    /// - Scrolling: Mouse wheel to scroll
    ///
    /// Returns a `MarkdownEvent` indicating what action was taken.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event
    /// * `area` - The area the widget occupies (for bounds checking)
    pub fn handle_mouse_event(&mut self, event: &MouseEvent, area: Rect) -> MarkdownEvent {
        if !is_in_area(event.column, event.row, area) {
            // Click outside area exits selection mode
            if self.selection.is_active() {
                self.selection.exit();
                return MarkdownEvent::SelectionEnded;
            }
            return MarkdownEvent::None;
        }

        let relative_y = event.row.saturating_sub(area.y) as usize;
        let relative_x = event.column.saturating_sub(area.x) as usize;
        let width = area.width as usize;

        // Document coordinates (accounting for scroll)
        let document_y = (relative_y + self.scroll.scroll_offset) as i32;
        let document_x = relative_x as i32;

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Exit active selection on new click
                if self.selection.is_active() {
                    self.selection.exit();
                }

                // Process click for double-click detection
                let (is_double, should_process_pending) =
                    self.double_click.process_click(event.column, event.row);

                if is_double {
                    // Double-click: return line info
                    if let Some(evt) = self.get_line_info_at_position(relative_y, width) {
                        return MarkdownEvent::DoubleClick {
                            line_number: evt.0,
                            line_kind: evt.1,
                            content: evt.2,
                        };
                    }
                }

                // If there was a pending click from a different position, process it now
                if should_process_pending {
                    // The old pending click was NOT part of a double-click
                    // But we don't process it here - we let it be handled by check_pending_timeout
                }

                // Don't process single-click actions (heading collapse, focus) immediately
                // to avoid content shifting between clicks of a double-click.
                // These are handled in check_pending_timeout.

                MarkdownEvent::None
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                let event_result = if !self.selection.is_active() {
                    // Start selection on drag
                    self.selection
                        .enter(document_x, document_y, self.rendered_lines.clone(), width);
                    self.selection.anchor = Some(SelectionPos::new(document_x, document_y));
                    self.mode = super::super::MarkdownWidgetMode::Drag;
                    MarkdownEvent::SelectionStarted
                } else {
                    MarkdownEvent::None
                };

                // Update cursor position during drag
                self.selection.update_cursor(document_x, document_y);

                event_result
            }
            MouseEventKind::Up(MouseButton::Left) => {
                // Selection complete - auto-copy to clipboard
                if self.selection.is_active() && self.selection.has_selection() {
                    // Update frozen lines with current rendered lines
                    self.selection.frozen_lines = Some(self.rendered_lines.clone());
                    self.selection.frozen_width = width;

                    // Auto-copy to clipboard
                    if let Some(text) = self.selection.get_selected_text() {
                        if !text.is_empty() {
                            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                if clipboard.set_text(&text).is_ok() {
                                    return MarkdownEvent::Copied {
                                        text: text.clone(),
                                    };
                                }
                            }
                        }
                    }
                }
                MarkdownEvent::None
            }
            MouseEventKind::ScrollUp => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_up(5);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: -(old_offset.saturating_sub(self.scroll.scroll_offset) as i32),
                }
            }
            MouseEventKind::ScrollDown => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_down(5);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: (self.scroll.scroll_offset.saturating_sub(old_offset) as i32),
                }
            }
            _ => MarkdownEvent::None,
        }
    }

    /// Check for pending single-click timeout and process if needed.
    ///
    /// Call this method periodically (e.g., each frame) to handle deferred
    /// single-click actions like heading collapse and focus line changes.
    ///
    /// Returns a `MarkdownEvent` if a pending click was processed.
    ///
    /// # Arguments
    ///
    /// * `area` - The area the widget occupies (for position calculations)
    pub fn check_pending_click(&mut self, area: Rect) -> MarkdownEvent {
        if let Some((x, y)) = self.double_click.check_pending_timeout() {
            // Calculate relative position
            let relative_y = y.saturating_sub(area.y) as usize;
            let relative_x = x.saturating_sub(area.x) as usize;
            let width = area.width as usize;

            // Set focused line based on click position (1-indexed)
            let clicked_line = self.scroll.scroll_offset + relative_y + 1;
            if clicked_line <= self.scroll.total_lines {
                self.scroll.set_current_line(clicked_line);
            }

            // Try to handle heading collapse
            if self.handle_click_collapse(relative_x, relative_y, width) {
                // Heading was toggled - get info for the event
                if let Some((_, line_kind, text)) =
                    self.get_line_info_at_position(relative_y, width)
                {
                    if line_kind == "Heading" {
                        return MarkdownEvent::HeadingToggled {
                            level: 1, // We don't have easy access to level here
                            text,
                            collapsed: true, // We toggled, but don't know new state
                        };
                    }
                }
            }

            return MarkdownEvent::FocusedLine {
                line: clicked_line,
            };
        }

        MarkdownEvent::None
    }

    /// Handle click for collapse/expand functionality.
    ///
    /// Returns `true` if a collapsible element was toggled.
    fn handle_click_collapse(&mut self, _x: usize, y: usize, width: usize) -> bool {
        use crate::markdown_renderer::styled_line::StyledLineKind;

        let styled_lines = crate::markdown_renderer::render_markdown_to_styled_lines(self.content);

        // Account for scroll offset - y is relative to visible area
        let document_y = y + self.scroll.scroll_offset;
        let mut line_idx = 0;

        for (idx, styled_line) in styled_lines.iter().enumerate() {
            // Skip lines that shouldn't be rendered (collapsed sections)
            if !should_render_line(styled_line, idx, self.scroll) {
                continue;
            }

            let rendered = render_styled_line(styled_line, width);
            let line_count = rendered.len();

            if document_y >= line_idx && document_y < line_idx + line_count {
                match &styled_line.kind {
                    StyledLineKind::Heading { section_id, .. } => {
                        self.scroll.toggle_section_collapse(*section_id);
                        self.scroll.invalidate_cache();
                        return true;
                    }
                    StyledLineKind::Frontmatter { .. } => {
                        self.scroll.toggle_section_collapse(0);
                        self.scroll.invalidate_cache();
                        return true;
                    }
                    StyledLineKind::FrontmatterStart { .. } => {
                        self.scroll.toggle_section_collapse(0);
                        self.scroll.invalidate_cache();
                        return true;
                    }
                    StyledLineKind::ExpandToggle { content_id, .. } => {
                        self.scroll.toggle_expandable(content_id);
                        self.scroll.invalidate_cache();
                        return true;
                    }
                    _ => {}
                }
            }

            line_idx += line_count;
        }

        false
    }

    /// Get line information at a given screen position.
    ///
    /// Returns (line_number, line_kind, content) if found.
    fn get_line_info_at_position(
        &self,
        y: usize,
        width: usize,
    ) -> Option<(usize, String, String)> {
        use crate::markdown_renderer::styled_line::StyledLineKind;

        let styled_lines = crate::markdown_renderer::render_markdown_to_styled_lines(self.content);
        let document_y = y + self.scroll.scroll_offset;
        let mut visual_line_idx = 0;
        let mut logical_line_num = 0;

        for (idx, styled_line) in styled_lines.iter().enumerate() {
            if !should_render_line(styled_line, idx, self.scroll) {
                continue;
            }

            logical_line_num += 1;

            let rendered = render_styled_line(styled_line, width);
            let line_count = rendered.len();

            if document_y >= visual_line_idx && document_y < visual_line_idx + line_count {
                let line_kind = match &styled_line.kind {
                    StyledLineKind::Heading { .. } => "Heading",
                    StyledLineKind::Paragraph(_) => "Paragraph",
                    StyledLineKind::CodeBlockHeader { .. } => "CodeBlockHeader",
                    StyledLineKind::CodeBlockContent { .. } => "CodeBlockContent",
                    StyledLineKind::CodeBlockBorder { .. } => "CodeBlockBorder",
                    StyledLineKind::ListItem { .. } => "ListItem",
                    StyledLineKind::Blockquote { .. } => "Blockquote",
                    StyledLineKind::Empty => "Empty",
                    StyledLineKind::HorizontalRule => "HorizontalRule",
                    StyledLineKind::Frontmatter { .. } => "Frontmatter",
                    StyledLineKind::FrontmatterStart { .. } => "FrontmatterStart",
                    StyledLineKind::FrontmatterField { .. } => "FrontmatterField",
                    StyledLineKind::FrontmatterEnd => "FrontmatterEnd",
                    StyledLineKind::Expandable { .. } => "Expandable",
                    StyledLineKind::ExpandToggle { .. } => "ExpandToggle",
                    StyledLineKind::TableRow { .. } => "TableRow",
                    StyledLineKind::TableBorder(_) => "TableBorder",
                    StyledLineKind::HeadingBorder { .. } => "HeadingBorder",
                };

                let text_content = self.get_styled_line_text(&styled_line.kind);

                return Some((logical_line_num, line_kind.to_string(), text_content));
            }

            visual_line_idx += line_count;
        }

        None
    }

    /// Extract plain text from a StyledLineKind.
    fn get_styled_line_text(&self, kind: &crate::markdown_renderer::styled_line::StyledLineKind) -> String {
        use crate::markdown_renderer::styled_line::{StyledLineKind, TextSegment};

        fn segment_to_text(seg: &TextSegment) -> &str {
            match seg {
                TextSegment::Plain(s) => s,
                TextSegment::Bold(s) => s,
                TextSegment::Italic(s) => s,
                TextSegment::BoldItalic(s) => s,
                TextSegment::InlineCode(s) => s,
                TextSegment::Link { text, .. } => text,
                TextSegment::Strikethrough(s) => s,
                TextSegment::Html(s) => s,
                TextSegment::Checkbox(_) => "",
            }
        }

        match kind {
            StyledLineKind::Heading { text, .. } => {
                text.iter().map(segment_to_text).collect()
            }
            StyledLineKind::Paragraph(segments) => {
                segments.iter().map(segment_to_text).collect()
            }
            StyledLineKind::CodeBlockContent { content, .. } => content.clone(),
            StyledLineKind::CodeBlockHeader { language, .. } => language.clone(),
            StyledLineKind::ListItem { content, .. } => {
                content.iter().map(segment_to_text).collect()
            }
            StyledLineKind::Blockquote { content, .. } => {
                content.iter().map(segment_to_text).collect()
            }
            StyledLineKind::Frontmatter { fields, .. } => {
                fields.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", ")
            }
            StyledLineKind::FrontmatterField { key, value } => format!("{}: {}", key, value),
            StyledLineKind::TableRow { cells, .. } => cells.join(" | "),
            _ => String::new(),
        }
    }

    /// Set the rendered lines for selection text extraction.
    ///
    /// Call this after rendering to update the cached lines.
    pub fn set_rendered_lines(&mut self, lines: Vec<ratatui::text::Line<'static>>) {
        self.rendered_lines = lines;
    }

    /// Check if selection mode is active.
    pub fn is_selection_active(&self) -> bool {
        self.selection.is_active()
    }

    /// Get the current selection state (for rendering).
    pub fn selection(&self) -> &super::super::super::selection_state::SelectionState {
        self.selection
    }
}
