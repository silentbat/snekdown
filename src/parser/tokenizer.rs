use crate::tokens::Token;
use charred::tapemachine::CharTapeMachine;

pub struct Tokenizer {
    ctm: CharTapeMachine,
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new(text: String) -> Self {
        Self {
            ctm: CharTapeMachine::new(text.chars().collect()),
            tokens: Vec::new(),
        }
    }
}
