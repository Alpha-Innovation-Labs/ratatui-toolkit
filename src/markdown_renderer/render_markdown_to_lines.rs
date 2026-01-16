//! Render markdown content to styled lines for display.

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

use super::styled_line::methods::render::render as styled_line_render;
use super::styled_line::{
    CodeBlockBorderKind, StyledLine, StyledLineKind, TableBorderKind, TextSegment,
};
use super::SyntaxHighlighter;

/// Parse YAML frontmatter from the beginning of content.
/// Returns (frontmatter_fields, remaining_content).
fn parse_frontmatter(content: &str) -> (Option<Vec<(String, String)>>, &str) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content);
    }

    // Find the closing ---
    let after_opening = &trimmed[3..];
    if let Some(end_pos) = after_opening.find("\n---") {
        let frontmatter_text = &after_opening[..end_pos];
        let remaining = &after_opening[end_pos + 4..]; // Skip past "\n---"

        // Parse the frontmatter fields
        let mut fields = Vec::new();
        for line in frontmatter_text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                // Remove surrounding quotes from value if present
                let value = if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value[1..value.len() - 1].to_string()
                } else {
                    value
                };
                fields.push((key, value));
            }
        }

        if !fields.is_empty() {
            return (Some(fields), remaining);
        }
    }

    (None, content)
}

/// Render markdown content to a vector of styled lines.
///
/// # Arguments
///
/// * `content` - The markdown content to render
///
/// # Returns
///
/// A vector of StyledLine, ready for rendering.
pub fn render_markdown_to_styled_lines(content: &str) -> Vec<StyledLine> {
    render_markdown_to_styled_lines_with_frontmatter_state(content, true)
}

/// Render markdown content with configurable frontmatter collapsed state.
pub fn render_markdown_to_styled_lines_with_frontmatter_state(
    content: &str,
    frontmatter_collapsed: bool,
) -> Vec<StyledLine> {
    let mut lines = Vec::new();

    // Parse frontmatter first
    let (frontmatter, remaining_content) = parse_frontmatter(content);

    // Add frontmatter if present
    if let Some(fields) = frontmatter {
        lines.push(StyledLine {
            kind: StyledLineKind::Frontmatter {
                fields,
                collapsed: frontmatter_collapsed,
            },
        });
        lines.push(StyledLine {
            kind: StyledLineKind::Empty,
        });
    }

    let mut current_segments: Vec<TextSegment> = Vec::new();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_started = false;
    let mut list_stack: Vec<(bool, usize)> = Vec::new(); // (ordered, current_number)
    let mut in_blockquote = false;

    // Table state
    let mut in_table = false;
    let mut table_header_done = false;
    let mut current_row_cells: Vec<String> = Vec::new();
    let mut table_col_widths: Vec<usize> = Vec::new();
    let mut pending_table_rows: Vec<(Vec<String>, bool)> = Vec::new(); // (cells, is_header)

    // Text formatting state
    let mut in_bold = false;
    let mut in_italic = false;

    let options = Options::all();
    let parser = Parser::new_ext(remaining_content, options);

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { .. } => {
                    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                }
                Tag::Paragraph => {
                    // Nothing special needed at start
                }
                Tag::CodeBlock(kind) => {
                    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                    in_code_block = true;
                    code_block_started = false;
                    code_block_lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                }
                Tag::List(start) => {
                    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                    let ordered = start.is_some();
                    let number = start.unwrap_or(1) as usize;
                    list_stack.push((ordered, number));
                }
                Tag::Item => {
                    // Will be handled with text content
                }
                Tag::BlockQuote(_) => {
                    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                    in_blockquote = true;
                }
                Tag::Emphasis => {
                    in_italic = true;
                }
                Tag::Strong => {
                    in_bold = true;
                }
                Tag::Link { .. } => {
                    // We'll capture the text and URL
                    // For now, treat like plain text until we see the end
                }
                Tag::Table(_alignments) => {
                    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                    in_table = true;
                    table_header_done = false;
                    table_col_widths.clear();
                    pending_table_rows.clear();
                }
                Tag::TableHead => {
                    current_row_cells.clear();
                }
                Tag::TableRow => {
                    current_row_cells.clear();
                }
                Tag::TableCell => {
                    // Start a new cell - push empty string to accumulate text into
                    current_row_cells.push(String::new());
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(level) => {
                    let level_num = match level {
                        pulldown_cmark::HeadingLevel::H1 => 1,
                        pulldown_cmark::HeadingLevel::H2 => 2,
                        pulldown_cmark::HeadingLevel::H3 => 3,
                        pulldown_cmark::HeadingLevel::H4 => 4,
                        pulldown_cmark::HeadingLevel::H5 => 5,
                        pulldown_cmark::HeadingLevel::H6 => 6,
                    };

                    let text = std::mem::take(&mut current_segments);

                    let section_id = lines.len(); // Index where this heading will be
                    lines.push(StyledLine {
                        kind: StyledLineKind::Heading {
                            level: level_num,
                            text,
                            section_id,
                            collapsed: false, // Default to expanded
                        },
                    });
                    // Add empty line after heading for visual spacing
                    // (markdown blank lines between block elements don't generate events)
                    lines.push(StyledLine {
                        kind: StyledLineKind::Empty,
                    });
                }
                TagEnd::Paragraph => {
                    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                    lines.push(StyledLine {
                        kind: StyledLineKind::Empty,
                    });
                }
                TagEnd::CodeBlock => {
                    // End code block
                    lines.push(StyledLine {
                        kind: StyledLineKind::CodeBlockBorder(CodeBlockBorderKind::Bottom),
                    });
                    lines.push(StyledLine {
                        kind: StyledLineKind::Empty,
                    });
                    in_code_block = false;
                    code_block_lang.clear();
                }
                TagEnd::List(_) => {
                    list_stack.pop();
                    if list_stack.is_empty() {
                        lines.push(StyledLine {
                            kind: StyledLineKind::Empty,
                        });
                    }
                }
                TagEnd::Item => {
                    if !current_segments.is_empty() {
                        let depth = list_stack.len().saturating_sub(1);
                        let (ordered, number) = list_stack.last().copied().unwrap_or((false, 1));
                        let content = std::mem::take(&mut current_segments);

                        lines.push(StyledLine {
                            kind: StyledLineKind::ListItem {
                                depth,
                                ordered,
                                number: if ordered { Some(number) } else { None },
                                content,
                            },
                        });

                        // Increment number for ordered lists
                        if let Some((is_ordered, num)) = list_stack.last_mut() {
                            if *is_ordered {
                                *num += 1;
                            }
                        }
                    }
                }
                TagEnd::BlockQuote(_) => {
                    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                    in_blockquote = false;
                }
                TagEnd::Emphasis => {
                    in_italic = false;
                }
                TagEnd::Strong => {
                    in_bold = false;
                }
                TagEnd::Link => {
                    // Link ended, text already captured
                }
                TagEnd::Table => {
                    // Render the complete table with proper borders
                    if !pending_table_rows.is_empty() {
                        // Top border
                        lines.push(StyledLine {
                            kind: StyledLineKind::TableBorder(TableBorderKind::Top(
                                table_col_widths.clone(),
                            )),
                        });

                        for (cells, is_header) in pending_table_rows.drain(..) {
                            // Pad cells to column widths
                            let padded_cells: Vec<String> = cells
                                .iter()
                                .enumerate()
                                .map(|(j, cell)| {
                                    let width =
                                        table_col_widths.get(j).copied().unwrap_or(cell.len());
                                    format!("{:width$}", cell, width = width)
                                })
                                .collect();

                            lines.push(StyledLine {
                                kind: StyledLineKind::TableRow {
                                    cells: padded_cells,
                                    is_header,
                                },
                            });

                            // Header separator after first row
                            if is_header {
                                lines.push(StyledLine {
                                    kind: StyledLineKind::TableBorder(
                                        TableBorderKind::HeaderSeparator(table_col_widths.clone()),
                                    ),
                                });
                            }
                        }

                        // Bottom border
                        lines.push(StyledLine {
                            kind: StyledLineKind::TableBorder(TableBorderKind::Bottom(
                                table_col_widths.clone(),
                            )),
                        });
                    }

                    in_table = false;
                    lines.push(StyledLine {
                        kind: StyledLineKind::Empty,
                    });
                }
                TagEnd::TableHead => {
                    // Finalize header row
                    for (i, cell) in current_row_cells.iter().enumerate() {
                        if i >= table_col_widths.len() {
                            table_col_widths.push(cell.len());
                        } else {
                            table_col_widths[i] = table_col_widths[i].max(cell.len());
                        }
                    }
                    pending_table_rows.push((current_row_cells.clone(), true)); // Header
                    current_row_cells.clear();
                    table_header_done = true;
                }
                TagEnd::TableRow => {
                    // Finalize body row
                    if table_header_done {
                        for (i, cell) in current_row_cells.iter().enumerate() {
                            if i >= table_col_widths.len() {
                                table_col_widths.push(cell.len());
                            } else {
                                table_col_widths[i] = table_col_widths[i].max(cell.len());
                            }
                        }

                        pending_table_rows.push((current_row_cells.clone(), false));
                    }
                    current_row_cells.clear();
                }
                TagEnd::TableCell => {
                    // Cell content already added via Text events
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_code_block {
                    // Start code block with header if not done yet
                    if !code_block_started {
                        lines.push(StyledLine {
                            kind: StyledLineKind::CodeBlockBorder(CodeBlockBorderKind::Top),
                        });
                        lines.push(StyledLine {
                            kind: StyledLineKind::CodeBlockHeader {
                                language: code_block_lang.clone(),
                            },
                        });
                        lines.push(StyledLine {
                            kind: StyledLineKind::CodeBlockBorder(
                                CodeBlockBorderKind::HeaderSeparator,
                            ),
                        });
                        code_block_started = true;
                    }

                    // Add each line of code with syntax highlighting
                    let highlighter = SyntaxHighlighter::new();
                    for line in text.lines() {
                        let highlighted = highlighter.highlight(line, &code_block_lang);
                        lines.push(StyledLine {
                            kind: StyledLineKind::CodeBlockContent {
                                content: line.to_string(),
                                highlighted,
                            },
                        });
                    }
                } else if in_table {
                    // Accumulate text for current cell
                    if let Some(last_cell) = current_row_cells.last_mut() {
                        last_cell.push_str(&text);
                    } else {
                        current_row_cells.push(text.to_string());
                    }
                } else {
                    let segment = if in_bold && in_italic {
                        TextSegment::BoldItalic(text.to_string())
                    } else if in_bold {
                        TextSegment::Bold(text.to_string())
                    } else if in_italic {
                        TextSegment::Italic(text.to_string())
                    } else {
                        TextSegment::Plain(text.to_string())
                    };
                    current_segments.push(segment);
                }
            }
            Event::Code(code) => {
                if in_table {
                    // Add inline code to cell
                    if let Some(last_cell) = current_row_cells.last_mut() {
                        last_cell.push_str(&format!("`{}`", code));
                    } else {
                        current_row_cells.push(format!("`{}`", code));
                    }
                } else {
                    current_segments.push(TextSegment::InlineCode(code.to_string()));
                }
            }
            Event::SoftBreak => {
                if !in_code_block && !in_table {
                    current_segments.push(TextSegment::Plain(" ".to_string()));
                }
            }
            Event::HardBreak => {
                if !in_code_block {
                    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                }
            }
            Event::Rule => {
                flush_paragraph(&mut lines, &mut current_segments, in_blockquote);
                lines.push(StyledLine {
                    kind: StyledLineKind::HorizontalRule,
                });
                lines.push(StyledLine {
                    kind: StyledLineKind::Empty,
                });
            }
            _ => {}
        }
    }

    // Flush any remaining content
    flush_paragraph(&mut lines, &mut current_segments, in_blockquote);

    // Remove trailing empty lines
    while matches!(lines.last(), Some(l) if matches!(l.kind, StyledLineKind::Empty)) {
        lines.pop();
    }

    if lines.is_empty() {
        lines.push(StyledLine {
            kind: StyledLineKind::Empty,
        });
    }

    lines
}

/// Flush accumulated segments as a paragraph or blockquote.
fn flush_paragraph(
    lines: &mut Vec<StyledLine>,
    segments: &mut Vec<TextSegment>,
    in_blockquote: bool,
) {
    if segments.is_empty() {
        return;
    }

    let content = std::mem::take(segments);

    if in_blockquote {
        lines.push(StyledLine {
            kind: StyledLineKind::Blockquote(content),
        });
    } else {
        lines.push(StyledLine {
            kind: StyledLineKind::Paragraph(content),
        });
    }
}

/// Legacy function for backward compatibility - renders to plain strings.
pub fn render_markdown_to_lines(content: &str) -> Vec<String> {
    let styled = render_markdown_to_styled_lines(content);
    styled
        .iter()
        .flat_map(|line| {
            let rendered = styled_line_render(line, 80);
            rendered
                .into_iter()
                .map(|l| {
                    l.spans
                        .into_iter()
                        .map(|s| s.content.to_string())
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
