//! Main render implementation for StyledLine.

use super::super::{StyledLine, StyledLineKind};
use super::render_blockquote;
use super::render_code_block;
use super::render_expandable;
use super::render_frontmatter;
use super::render_heading;
use super::render_horizontal_rule;
use super::render_list_item;
use super::render_paragraph;
use super::render_table_border;
use super::render_table_row;
use ratatui::text::Line;

/// Render a styled line to ratatui Line with given width.
pub fn render(styled_line: &StyledLine, width: usize) -> Vec<Line<'static>> {
    match &styled_line.kind {
        StyledLineKind::Heading {
            level,
            text,
            collapsed,
            ..
        } => render_heading::render(styled_line, *level, text, *collapsed, width),
        StyledLineKind::HeadingBorder { level } => {
            vec![render_heading::render_border(styled_line, *level, width)]
        }
        StyledLineKind::CodeBlockHeader { language } => {
            vec![render_code_block::render_header(
                styled_line,
                language,
                width,
            )]
        }
        StyledLineKind::CodeBlockContent {
            content,
            highlighted,
        } => {
            vec![render_code_block::render_content(
                styled_line,
                content,
                highlighted.as_ref(),
                width,
            )]
        }
        StyledLineKind::CodeBlockBorder(kind) => {
            vec![render_code_block::render_border(styled_line, kind, width)]
        }
        StyledLineKind::Paragraph(segments) => {
            render_paragraph::render(styled_line, segments, width)
        }
        StyledLineKind::ListItem {
            depth,
            ordered,
            number,
            content,
        } => render_list_item::render(styled_line, *depth, *ordered, *number, content, width),
        StyledLineKind::Blockquote(segments) => {
            render_blockquote::render(styled_line, segments, width)
        }
        StyledLineKind::TableRow { cells, is_header } => {
            vec![render_table_row::render(styled_line, cells, *is_header)]
        }
        StyledLineKind::TableBorder(kind) => {
            vec![render_table_border::render(styled_line, kind)]
        }
        StyledLineKind::HorizontalRule => {
            vec![render_horizontal_rule::render(styled_line, width)]
        }
        StyledLineKind::Empty => {
            vec![Line::from("")]
        }
        StyledLineKind::Frontmatter { fields, collapsed } => {
            render_frontmatter::render(styled_line, fields, *collapsed, width)
        }
        StyledLineKind::Expandable {
            content_id,
            lines,
            max_lines,
            collapsed,
            total_lines,
        } => render_expandable::render_expandable(
            styled_line,
            content_id,
            lines,
            *max_lines,
            *collapsed,
            *total_lines,
            width,
        ),
        StyledLineKind::ExpandToggle {
            content_id,
            expanded,
            hidden_count,
        } => render_expandable::render_expand_toggle(
            styled_line,
            content_id,
            *expanded,
            *hidden_count,
            width,
        ),
    }
}
