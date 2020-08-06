use crate::elements::Document;
use crate::tokens::Token;

pub struct TreeParser {
    tokens: Vec<Token>,
    index: usize,
    document: Document,
}

impl TreeParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            document: Document::new(true),
        }
    }

    /// steps if the
    fn step(&mut self) -> Option<()> {
        if self.index < (self.tokens.len() - 1) {
            self.index += 1;

            Some(())
        } else {
            None
        }
    }

    /// returns if the eof is reached
    fn check_eof(&self) -> bool {
        self.index == (self.tokens.len() - 1)
    }
}
