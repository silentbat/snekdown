pub mod constants;
pub mod mappings;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Token {
    Block(BlockToken),
    Line(LineToken),
    Inline(InlineToken),
    Text(TextToken),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum BlockToken {
    CodeBlock,
    MathBlock,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum LineToken {
    Ruler,
    Import,
    ListItem(bool),
    Centered,
    Quote,
    Checkbox(bool),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum InlineToken {
    Italic,
    Bold,
    Underline,
    Striked,
    Super,
    Mono,
    Math,
    Emoji,
    Color,
    ColorReset,
    PlaceholderOpen,
    PlaceholderClose,
    BibRef,
    TableSpacerLeft,
    TableSpacerRight,
    ImageOpen,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum TextToken {
    Plain(String),
    Indent(u32),
    InlineWhitespace,
    LineBreak,
    Eq,
    Pipe,
    Colon,
    ParenthesesOpen,
    ParenthesesClose,
    BracketsOpen,
    BracketsClose,
    BracesOpen,
    BracesClose,
    SingleString,
    DoubleString,
}