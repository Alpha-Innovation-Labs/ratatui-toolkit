//! Styled line types for markdown rendering.
//!
//! Represents parsed markdown elements with styling information
//! for render-markdown.nvim style rendering.

pub mod code_block_border_kind;
pub mod column_alignment;
pub mod methods;
#[allow(clippy::module_inception)]
pub mod styled_line;
pub mod styled_line_kind;
pub mod table_border_kind;
pub mod text_segment;

pub use code_block_border_kind::CodeBlockBorderKind;
pub use column_alignment::ColumnAlignment;
pub use styled_line::StyledLine;
pub use styled_line_kind::StyledLineKind;
pub use table_border_kind::TableBorderKind;
pub use text_segment::{CheckboxState, TextSegment};

pub mod constants;
pub use constants::{
    get_language_icon, get_link_icon, CodeBlockColors, CodeBlockTheme, BLOCKQUOTE_MARKER,
    BULLET_MARKERS, CHECKBOX_CHECKED, CHECKBOX_TODO, CHECKBOX_UNCHECKED, HEADING_ICONS,
    HORIZONTAL_RULE_CHAR,
};
