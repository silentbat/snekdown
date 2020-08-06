use crate::tokens::constants::block_constants::*;
use crate::tokens::constants::inline_constants::*;
use crate::tokens::constants::line_constants::*;
use crate::tokens::constants::TokenConstant;
use crate::tokens::Token::{Block, Inline, Line, Text};
use crate::tokens::{BlockToken, InlineToken, LineToken, TextToken, Token};

fn get_mappings() -> Vec<(TokenConstant, Token)> {
    vec![
        (T_HEADER_6, Block(BlockToken::H6)),
        (T_HEADER_5, Block(BlockToken::H5)),
        (T_HEADER_4, Block(BlockToken::H4)),
        (T_HEADER_3, Block(BlockToken::H3)),
        (T_HEADER_2, Block(BlockToken::H2)),
        (T_HEADER_1, Block(BlockToken::H1)),
        (T_CODE_BLOCK, Block(BlockToken::CodeBlock)),
        (T_MATH_BLOCK, Block(BlockToken::MathBlock)),
        (T_QUOTE, Line(LineToken::Quote)),
        (T_RULER, Line(LineToken::Ruler)),
        (T_LINE_CENTERED, Line(LineToken::Centered)),
        (T_LIST_ITEM_UL, Line(LineToken::ListItem(false))),
        (T_LIST_ITEM_OL, Line(LineToken::ListItem(true))),
        (T_IMPORT_OPEN, Line(LineToken::Import)),
        (T_CHECKBOX_CHECKED, Line(LineToken::Checkbox(true))),
        (T_CHECKBOX_UNCHECKED, Line(LineToken::Checkbox(false))),
        (T_BOLD, Inline(InlineToken::Bold)),
        (T_ITALIC, Inline(InlineToken::Italic)),
        (T_MONOSPACE, Inline(InlineToken::Mono)),
        (T_SUPERSCRIPT, Inline(InlineToken::Super)),
        (T_STRIKETHROUGH, Inline(InlineToken::Striked)),
        (T_UNDERLINE, Inline(InlineToken::Underline)),
        (T_MATH, Inline(InlineToken::Math)),
        (T_EMOJI, Inline(InlineToken::Emoji)),
        (T_COLOR_OPEN, Inline(InlineToken::Color)),
        (T_COLOR_RESET, Inline(InlineToken::ColorReset)),
        (T_IMG, Inline(InlineToken::Image)),
        (T_PLACEHOLDER_OPEN, Inline(InlineToken::PlaceholderOpen)),
        (T_PLACEHOLDER_CLOSE, Inline(InlineToken::PlaceholderClose)),
        (T_TABLE_SPACER_LEFT, Inline(InlineToken::TableSpacerLeft)),
        (T_BIBREF_OPEN, Inline(InlineToken::BibRef)),
        (T_TABLE_SPACER_RIGHT, Inline(InlineToken::TableSpacerRight)),
        (T_INLINE_WHITESPACE, Text(TextToken::InlineWhitespace)),
        (T_LINEBREAK, Text(TextToken::LineBreak)),
        (T_SINGLE_STRING, Text(TextToken::SingleString)),
        (T_DOUBLE_STRING, Text(TextToken::DoubleString)),
        (T_PARENT_OPEN, Text(TextToken::ParenthesesOpen)),
        (T_PARENT_CLOSE, Text(TextToken::ParenthesesClose)),
        (T_BRACE_OPEN, Text(TextToken::BracesOpen)),
        (T_BRACE_CLOSE, Text(TextToken::BracesClose)),
        (T_BRACKET_OPEN, Text(TextToken::BracketsOpen)),
        (T_BRACKET_CLOSE, Text(TextToken::BracketsClose)),
        (T_COLON, Text(TextToken::Colon)),
        (T_EQ, Text(TextToken::Eq)),
        (T_PIPE, Text(TextToken::Pipe)),
    ]
}
