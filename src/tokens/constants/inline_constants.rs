use crate::tokens::constants::TokenConstant;

pub const T_BOLD: TokenConstant = &["**"];
pub const T_ITALIC: TokenConstant = &["*", "_"];
pub const T_MONOSPACE: TokenConstant = &["`"];
pub const T_SUPERSCRIPT: TokenConstant = &["^"];
pub const T_STRIKETHROUGH: TokenConstant = &["~~"];
pub const T_UNDERLINE: TokenConstant = &["__"];
pub const T_MATH: TokenConstant = &["$$"];
pub const T_EMOJI: TokenConstant = &[":"];

pub const T_SINGLE_STRING: TokenConstant = &["'"];
pub const T_DOUBLE_STRING: TokenConstant = &["\""];

pub const T_COLOR_OPEN: TokenConstant = &["§["];
pub const T_COLOR_RESET: TokenConstant = &["§[]"];

pub const T_PARENT_OPEN: TokenConstant = &["("];
pub const T_PARENT_CLOSE: TokenConstant = &[")"];

pub const T_BRACE_OPEN: TokenConstant = &["{"];
pub const T_BRACE_CLOSE: TokenConstant = &["}"];

pub const T_BRACKET_OPEN: TokenConstant = &["["];
pub const T_BRACKET_CLOSE: TokenConstant = &["]"];

pub const T_PLACEHOLDER_OPEN: TokenConstant = &["[["];
pub const T_PLACEHOLDER_CLOSE: TokenConstant = &["]]"];

pub const T_BIBREF_OPEN: TokenConstant = &["[^"];

pub const T_COLON: TokenConstant = &[":"];
pub const T_EQ: TokenConstant = &["="];
pub const T_PIPE: TokenConstant = &["|"];

pub const T_IMG: TokenConstant = &["!["];
pub const T_INLINE_WHITESPACE: TokenConstant = &[" ", "\t"];
pub const T_LINEBREAK: TokenConstant = &["\n", "\x15\n"];

pub const T_TABLE_SPACER_LEFT: TokenConstant = &["|--"];
pub const T_TABLE_SPACER_RIGHT: TokenConstant = &["--|"];
