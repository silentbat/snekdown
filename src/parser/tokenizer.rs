use crate::tokens::constants::TokenConstant;
use crate::tokens::mappings::get_mappings;
use crate::tokens::{TextToken, Token};
use charred::tapemachine::CharTapeMachine;

pub struct Tokenizer {
    ctm: CharTapeMachine,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        let mut chars = text.chars().collect::<Vec<char>>();
        chars.push('\n');
        Self {
            ctm: CharTapeMachine::new(chars),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut temp_text = String::new();
        while !self.ctm.check_eof() {
            if let Some(token) = self.parse_literal_token() {
                if !temp_text.is_empty() {
                    tokens.push(Token::Text(TextToken::Plain(temp_text.clone())));
                    temp_text.clear();
                }
                tokens.push(token)
            } else {
                temp_text.push(self.ctm.get_current())
            }
            let _ = self.ctm.seek_one();
        }

        tokens
    }

    fn parse_literal_token(&mut self) -> Option<Token> {
        lazy_static! {
            static ref TOKEN_MAPPING: Vec<(TokenConstant, Token)> = get_mappings();
        }
        for (token_const, token) in TOKEN_MAPPING.iter() {
            if self.ctm.check_any_str_sequence(token_const) {
                return Some(token.clone());
            }
        }

        None
    }
}
