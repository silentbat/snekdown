use snekdown::parser::tokenizer::Tokenizer;
use snekdown::tokens::{BlockToken, InlineToken, LineToken, TextToken, Token};

#[test]
fn it_tokenizes() {
    let mut tokenizer = Tokenizer::new("# H1\n- item\n **hoho** `aha`".to_string());
    let tokens = tokenizer.parse();
    assert_eq!(
        tokens,
        vec![
            Token::Block(BlockToken::H1),
            Token::Text(TextToken::InlineWhitespace),
            Token::Text(TextToken::Plain("H1".to_string())),
            Token::Text(TextToken::LineBreak),
            Token::Line(LineToken::ListItem(false)),
            Token::Text(TextToken::Plain("item".to_string())),
            Token::Text(TextToken::LineBreak),
            Token::Text(TextToken::InlineWhitespace),
            Token::Inline(InlineToken::Bold),
            Token::Text(TextToken::Plain("hoho".to_string())),
            Token::Inline(InlineToken::Bold),
            Token::Text(TextToken::InlineWhitespace),
            Token::Inline(InlineToken::Mono),
            Token::Text(TextToken::Plain("aha".to_string())),
            Token::Inline(InlineToken::Mono),
        ]
    )
}
