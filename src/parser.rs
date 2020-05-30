use crate::elements::*;
use crate::tokens::*;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

macro_rules! parse_option {
    ($option:expr, $index:expr) => {
        if let Some(_) = $option {
        } else {
            return Err(ParseError::new($index));
        }
    };
}

#[derive(Debug)]
pub struct ParseError {
    index: usize,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "parse error at index {}", self.index)
    }
}
impl Error for ParseError {}
impl ParseError {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

pub struct Parser {
    index: usize,
    text: Vec<char>,
    current_char: char,
    section_nesting: u8,
    section_return: Option<u8>,
}

impl Parser {
    pub fn new(text: String) -> Self {
        let mut text: Vec<char> = text.chars().collect();
        text.append(&mut vec!['\n', ' ', '\n']); // push space and newline of eof. it fixes stuff and I don't know why.
        let current_char = text.get(0).unwrap().clone();

        Self {
            index: 0,
            text,
            current_char,
            section_nesting: 0,
            section_return: None,
        }
    }

    /// Increments the current index and returns the
    /// char at the indexes position
    fn next_char(&mut self) -> Option<char> {
        self.index += 1;

        self.current_char = self.text.get(self.index)?.clone();

        Some(self.current_char)
    }

    /// Returns to an index position
    fn revert_to(&mut self, index: usize) -> Result<(), ParseError> {
        if let Some(char) = self.text.get(index) {
            self.index = index;
            self.current_char = char.clone();
            Ok(())
        } else {
            Err(ParseError::new(index))
        }
    }

    /// reverts and returns a parse error
    fn revert_with_error(&mut self, index: usize) -> ParseError {
        let err = ParseError::new(self.index);

        if let Err(revert_err) = self.revert_to(index) {
            revert_err
        } else {
            err
        }
    }

    /// Skips characters until it encounters a character
    /// that isn't an inline whitespace character
    fn seek_inline_whitespace(&mut self) {
        if self.current_char.is_whitespace() && !self.check_linebreak() {
            while let Some(next_char) = self.next_char() {
                if !next_char.is_whitespace() || self.check_linebreak() {
                    break;
                }
            }
        }
    }

    /// Skips characters until it encounters a character
    /// that isn't a whitespace character
    fn seek_whitespace(&mut self) {
        if self.current_char.is_whitespace() {
            while let Some(next_char) = self.next_char() {
                if !next_char.is_whitespace() {
                    break;
                }
            }
        }
    }

    /// checks if the input character is escaped
    fn check_escaped(&self) -> bool {
        if self.index == 0 {
            return false;
        }
        if let Some(previous_char) = self.text.get(self.index - 1) {
            if previous_char == &SPECIAL_ESCAPE {
                return true;
            }
        }
        return false;
    }

    /// checks if the current character is the given input character and not escaped
    fn check_special(&self, character: &char) -> bool {
        self.current_char == *character && !self.check_escaped()
    }

    /// checks if the current character is part of the given group
    fn check_special_group(&self, chars: &[char]) -> bool {
        chars.contains(&self.current_char) && !self.check_escaped()
    }

    /// returns if the current character is a linebreak character
    /// Note: No one likes CRLF
    fn check_linebreak(&self) -> bool {
        self.current_char == LB && !self.check_escaped()
    }

    /// seeks inline whitespaces and returns if there
    /// were seeked whitespaces
    fn check_seek_inline_whitespace(&mut self) -> bool {
        let start_index = self.index;
        self.seek_inline_whitespace();
        self.index > start_index
    }

    /// checks if the next characters match a special sequence
    fn check_special_sequence(&mut self, sequence: &[char]) -> Result<(), ParseError> {
        let start_index = self.index;
        self.seek_whitespace();
        for sq_character in sequence {
            if !self.check_special(sq_character) {
                return Err(self.revert_with_error(start_index));
            }
            if self.next_char() == None {
                return Err(self.revert_with_error(start_index));
            }
        }
        if self.index > 0 {
            self.revert_to(self.index - 1)?;
        }

        Ok(())
    }

    /// parses the given text into a document
    pub fn parse(&mut self) -> Document {
        let mut document = Document::new();
        while self.index < self.text.len() {
            if let Ok(token) = self.parse_block() {
                document.add_element(token);
            }
        }

        document
    }

    /// Parses a block Token
    pub fn parse_block(&mut self) -> Result<Block, ParseError> {
        if let Some(section) = self.section_return {
            if section <= self.section_nesting {
                return Err(ParseError::new(self.index));
            } else {
                self.section_return = None;
            }
        }
        let token = if let Ok(section) = self.parse_section() {
            Block::Section(section)
        } else if let Some(_) = self.section_return {
            return Err(ParseError::new(self.index));
        } else if let Ok(list) = self.parse_list() {
            Block::List(list)
        } else if let Ok(table) = self.parse_table() {
            Block::Table(table)
        } else if let Ok(code_block) = self.parse_code_block() {
            Block::CodeBlock(code_block)
        } else if let Ok(quote) = self.parse_quote() {
            Block::Quote(quote)
        } else if let Ok(paragraph) = self.parse_paragraph() {
            Block::Paragraph(paragraph)
        } else {
            return Err(ParseError::new(self.index));
        };

        Ok(token)
    }

    /// Parses a section that consists of a header and one or more blocks
    fn parse_section(&mut self) -> Result<Section, ParseError> {
        let start_index = self.index;
        self.seek_whitespace();
        if self.check_special(&HASH) {
            let mut size = 1;
            while let Some(_) = self.next_char() {
                if self.check_special(&HASH) {
                    size += 1;
                } else {
                    break;
                }
            }
            if size <= self.section_nesting || !self.current_char.is_whitespace() {
                if size <= self.section_nesting {
                    self.section_return = Some(size);
                }
                return Err(self.revert_with_error(start_index));
            }
            self.seek_inline_whitespace();
            let mut header = self.parse_header()?;
            header.size = size;
            self.section_nesting = size;
            let mut section = Section::new(header);

            while let Ok(block) = self.parse_block() {
                section.add_element(block);
            }

            self.section_nesting -= 1;
            Ok(section)
        } else {
            return Err(self.revert_with_error(start_index));
        }
    }

    /// parses the header of a section
    fn parse_header(&mut self) -> Result<Header, ParseError> {
        Ok(Header {
            size: 0,
            line: self.parse_inline()?,
        })
    }

    /// parses a code block
    fn parse_code_block(&mut self) -> Result<CodeBlock, ParseError> {
        let mut language = String::new();
        self.check_special_sequence(&SQ_CODE_BLOCK)?;
        while let Some(character) = self.next_char() {
            if self.check_linebreak() {
                break;
            }
            language.push(character);
        }
        let mut text = String::new();
        while let Some(character) = self.next_char() {
            if let Ok(_) = self.check_special_sequence(&SQ_CODE_BLOCK) {
                break;
            }
            text.push(character);
        }

        Ok(CodeBlock {
            language,
            code: text,
        })
    }

    /// parses a quote
    fn parse_quote(&mut self) -> Result<Quote, ParseError> {
        let start_index = self.index;
        self.seek_whitespace();
        let metadata = if let Ok(meta) = self.parse_inline_metadata() {
            Some(meta)
        } else {
            None
        };
        if self.check_special(&META_CLOSE) {
            if self.next_char() == None {
                return Err(self.revert_with_error(start_index));
            }
        }
        let mut quote = Quote::new(metadata);

        while self.check_special(&QUOTE_START)
            && self.next_char() != None
            && self.check_seek_inline_whitespace()
        {
            if let Ok(text) = self.parse_text() {
                quote.add_text(text);
            } else {
                break;
            }
        }
        if quote.text.len() == 0 {
            return Err(self.revert_with_error(start_index));
        }

        Ok(quote)
    }

    /// Parses metadata
    /// TODO: Metadata object instead of raw string
    fn parse_inline_metadata(&mut self) -> Result<InlineMetadata, ParseError> {
        let start_index = self.index;
        if !self.check_special(&META_OPEN) {
            // if no meta open tag is present, the parsing has failed
            return Err(ParseError::new(self.index));
        }
        let mut text = String::new();
        while let Some(character) = self.next_char() {
            if self.check_special(&META_CLOSE) || self.check_linebreak() {
                // abort the parsing of the inner content when encountering a closing tag or linebreak
                break;
            }
            text.push(character);
        }
        if self.check_linebreak() || text.len() == 0 {
            // if there was a linebreak (the metadata wasn't closed) or there is no inner data
            // return an error
            return Err(self.revert_with_error(start_index));
        }

        Ok(InlineMetadata { data: text })
    }

    /// Parses a paragraph
    fn parse_paragraph(&mut self) -> Result<Paragraph, ParseError> {
        let mut paragraph = Paragraph::new();
        while let Ok(token) = self.parse_inline() {
            paragraph.add_element(token);
            let start_index = self.index;
            if self.check_special_group(&BLOCK_SPECIAL_CHARS) {
                self.revert_to(start_index)?;
                break;
            }
            self.revert_to(start_index)?;
        }

        if paragraph.elements.len() > 0 {
            Ok(paragraph)
        } else {
            Err(ParseError::new(self.index))
        }
    }

    /// parses a list which consists of one or more list items
    /// The parsing is done iterative to resolve nested items
    fn parse_list(&mut self) -> Result<List, ParseError> {
        let mut list = List::new();
        let start_index = self.index;
        self.seek_whitespace();

        let ordered = if self.check_special_group(&LIST_SPECIAL_CHARS) {
            false
        } else {
            true
        };
        list.ordered = ordered;
        let mut list_hierarchy: Vec<ListItem> = Vec::new();
        while let Ok(mut item) = self.parse_list_item() {
            while let Some(parent_item) = list_hierarchy.pop() {
                if parent_item.level < item.level {
                    // the parent item is the actual parent of the next item
                    list_hierarchy.push(parent_item);
                    break;
                } else if parent_item.level == item.level {
                    // the parent item is a sibling and has to be appended to a parent
                    if list_hierarchy.is_empty() {
                        list.add_item(parent_item);
                    } else {
                        let mut parent_parent = list_hierarchy.pop().unwrap();
                        parent_parent.add_child(parent_item);
                        list_hierarchy.push(parent_parent);
                    }
                    break;
                } else {
                    // the parent item is a child of a sibling of the current item
                    if list_hierarchy.is_empty() {
                        item.add_child(parent_item);
                    } else {
                        let mut parent_parent = list_hierarchy.pop().unwrap();
                        parent_parent.add_child(parent_item);
                        list_hierarchy.push(parent_parent);
                    }
                }
            }
            list_hierarchy.push(item);
        }

        // the remaining items in the hierarchy need to be combined
        while let Some(item) = list_hierarchy.pop() {
            if !list_hierarchy.is_empty() {
                let mut parent_item = list_hierarchy.pop().unwrap();
                parent_item.add_child(item);
                list_hierarchy.push(parent_item);
            } else {
                list_hierarchy.push(item);
                break;
            }
        }
        list.items.append(&mut list_hierarchy);

        if list.items.len() > 0 {
            Ok(list)
        } else {
            return Err(self.revert_with_error(start_index));
        }
    }

    /// parses a single list item defined with -
    fn parse_list_item(&mut self) -> Result<ListItem, ParseError> {
        let start_index = self.index;
        self.seek_inline_whitespace();
        let level = self.index - start_index;

        if !self.check_special_group(&LIST_SPECIAL_CHARS) && !self.current_char.is_numeric() {
            return Err(self.revert_with_error(start_index));
        }
        while let Some(character) = self.next_char() {
            if character.is_whitespace() {
                break;
            }
        }
        if self.next_char() == None {
            return Err(self.revert_with_error(start_index));
        }
        self.seek_inline_whitespace();
        let item = ListItem::new(self.parse_inline()?, level as u16);

        Ok(item)
    }

    /// parses a markdown table
    fn parse_table(&mut self) -> Result<Table, ParseError> {
        let header = self.parse_row()?;
        let start_index = self.index;
        self.seek_whitespace();
        if self.check_special(&MINUS) {
            if self.next_char() != Some(PIPE) {
                return Err(self.revert_with_error(start_index));
            }
        }
        while let Some(char) = self.next_char() {
            if char == '\n' {
                break;
            }
        }
        self.seek_whitespace();
        let mut table = Table::new(header);

        while let Ok(row) = self.parse_row() {
            table.add_row(row);
            self.seek_whitespace();
        }

        Ok(table)
    }

    /// parses a table row/head
    pub fn parse_row(&mut self) -> Result<Row, ParseError> {
        let start_index = self.index;
        self.seek_inline_whitespace();

        if self.check_special(&PIPE) {
            if self.next_char() == None {
                return Err(self.revert_with_error(start_index));
            }
        } else {
            return Err(self.revert_with_error(start_index));
        }
        let mut row = Row::new();
        while let Ok(element) = self.parse_inline() {
            row.add_cell(Cell { text: element });
            if self.check_special(&PIPE) {
                if self.next_char() == None {
                    break;
                }
            }
            if self.check_linebreak() {
                break;
            }
        }

        if row.cells.len() > 0 {
            Ok(row)
        } else {
            return Err(self.revert_with_error(start_index));
        }
    }

    /// parses inline definitions
    fn parse_inline(&mut self) -> Result<Inline, ParseError> {
        if self.index > self.text.len() {
            Err(ParseError::new(self.index))
        } else {
            Ok(Inline::Text(self.parse_text()?))
        }
    }

    /// Parses a line of text
    fn parse_text(&mut self) -> Result<Text, ParseError> {
        let mut text = Text::new();
        while let Ok(subtext) = self.parse_subtext() {
            text.add_subtext(subtext);
            let current_index = self.index;
            if self.next_char() == None {
                break;
            }
            self.revert_to(current_index)?;
        }

        if self.check_linebreak() {
            parse_option!(self.next_char(), self.index);
        }

        Ok(text)
    }

    /// parses subtext, the formatting parts of a line (Text)
    fn parse_subtext(&mut self) -> Result<SubText, ParseError> {
        if self.check_linebreak() {
            return Err(ParseError::new(self.index));
        }
        if let Ok(image) = self.parse_image() {
            return Ok(SubText::Image(image));
        }
        if let Ok(url) = self.parse_url() {
            return Ok(SubText::Url(url));
        }
        match self.current_char {
            ASTERISK if !self.check_escaped() => {
                parse_option!(self.next_char(), self.index);

                if self.check_special(&ASTERISK) {
                    parse_option!(self.next_char(), self.index);
                    let subtext = self.parse_subtext()?;
                    if self.check_special(&ASTERISK) {
                        parse_option!(self.next_char(), self.index);
                        if self.check_special(&ASTERISK) {
                            parse_option!(self.next_char(), self.index);
                        }
                    }
                    Ok(SubText::Bold(BoldText {
                        value: Box::new(subtext),
                    }))
                } else {
                    let subtext = self.parse_subtext()?;
                    parse_option!(self.next_char(), self.index);
                    Ok(SubText::Italic(ItalicText {
                        value: Box::new(subtext),
                    }))
                }
            }
            UNDERSCR if !self.check_escaped() => {
                parse_option!(self.next_char(), self.index);
                let subtext = self.parse_subtext()?;
                parse_option!(self.next_char(), self.index);
                Ok(SubText::Underlined(UnderlinedText {
                    value: Box::new(subtext),
                }))
            }
            TILDE if !self.check_escaped() => {
                parse_option!(self.next_char(), self.index);
                let subtext = self.parse_subtext()?;
                if self.check_special(&TILDE) {
                    parse_option!(self.next_char(), self.index);
                }
                Ok(SubText::Striked(StrikedText {
                    value: Box::new(subtext),
                }))
            }
            BACKTICK if !self.check_escaped() => {
                parse_option!(self.next_char(), self.index);
                let plain_text = self.parse_plain_text()?;
                if self.check_special(&BACKTICK) {
                    parse_option!(self.next_char(), self.index)
                }
                Ok(SubText::Monospace(MonospaceText { value: plain_text }))
            }
            PIPE if !self.check_escaped() => Err(ParseError::new(self.index)), // handling of table cells
            _ => Ok(SubText::Plain(self.parse_plain_text()?)),
        }
    }

    /// parses an image url
    fn parse_image(&mut self) -> Result<Image, ParseError> {
        let start_index = self.index;
        self.seek_inline_whitespace();

        if !self.check_special(&IMG_START) || self.next_char() == None {
            return Err(self.revert_with_error(start_index));
        }
        if let Ok(url) = self.parse_url() {
            let metadata = if let Ok(meta) = self.parse_inline_metadata() {
                if self.check_special(&META_CLOSE) && self.next_char() == None {
                    return Err(self.revert_with_error(start_index));
                }
                Some(meta)
            } else {
                None
            };
            Ok(Image { url, metadata })
        } else {
            Err(self.revert_with_error(start_index))
        }
    }

    // parses an url
    fn parse_url(&mut self) -> Result<Url, ParseError> {
        let start_index = self.index;
        self.seek_inline_whitespace();

        let mut description = String::new();
        if self.check_special(&DESC_OPEN) {
            while let Some(character) = self.next_char() {
                if self.check_special(&DESC_CLOSE) || self.check_linebreak() {
                    break;
                }
                description.push(character);
            }
            if !self.check_special(&DESC_CLOSE) || self.next_char() == None {
                // it stopped at a linebreak or EOF
                return Err(self.revert_with_error(start_index));
            }
        }

        if !self.check_special(&URL_OPEN) {
            // the next char isn't the start of the encased url
            return Err(self.revert_with_error(start_index));
        }
        self.seek_inline_whitespace();
        let mut url = String::new();
        while let Some(character) = self.next_char() {
            if self.check_special(&URL_CLOSE) || self.check_linebreak() {
                break;
            }
            url.push(character);
        }
        if !self.check_special(&URL_CLOSE) || url.is_empty() {
            return Err(self.revert_with_error(start_index));
        }
        parse_option!(self.next_char(), self.index);
        let matcher = Regex::new(EXPR_URI).unwrap();
        if !matcher.is_match(&url) {
            return Err(self.revert_with_error(start_index));
        }

        if description.is_empty() {
            Ok(Url::new(None, url))
        } else {
            Ok(Url::new(Some(description), url))
        }
    }

    /// parses plain text as a string until it encounters an unescaped special inline char
    fn parse_plain_text(&mut self) -> Result<PlainText, ParseError> {
        let mut current_char = self.current_char;
        let mut characters = String::new();
        let mut count = 0;
        loop {
            if self.check_special_group(&INLINE_SPECIAL_CHARS)
                || (count > 0 && self.check_special_group(&INLINE_SPECIAL_CHARS_SECOND))
            {
                break;
            } else {
                characters.push(current_char)
            }
            if let Some(character) = self.next_char() {
                current_char = character;
            } else {
                break;
            }
            count += 1;
        }

        if characters.len() > 0 {
            Ok(PlainText { value: characters })
        } else {
            Err(ParseError::new(self.index))
        }
    }
}
