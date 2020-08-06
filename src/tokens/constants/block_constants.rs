use crate::tokens::constants::TokenConstant;

pub const T_HEADER_6: TokenConstant = &["######"];
pub const T_HEADER_5: TokenConstant = &["#####"];
pub const T_HEADER_4: TokenConstant = &["####"];
pub const T_HEADER_3: TokenConstant = &["###"];
pub const T_HEADER_2: TokenConstant = &["##"];
pub const T_HEADER_1: TokenConstant = &["#"];

pub const T_CODE_BLOCK: TokenConstant = &["```"];
pub const T_MATH_BLOCK: TokenConstant = &["$$$"];

pub const T_TABLE_SPACER_LEFT: TokenConstant = &["|--"];
pub const T_TABLE_SPACER_RIGHT: TokenConstant = &["--|"];

pub const T_RULER: TokenConstant = &["---", "___", "- - -", "_ _ _ "];
