use logos::Logos;
pub use logos::{Lexer, Span};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(skip r";.*[\r\n]")]
pub enum Token {
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex("[&@a-zA-Z][a-zA-Z0-9-_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex(r#"`([^`])*`"#, |lex| lex.slice()[1..lex.slice().len() - 1].to_owned())]
    String(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn test_token_lparen() {
        let mut lexer = Token::lexer("(");
        assert_eq!(lexer.next(), Some(Ok(Token::LParen)));
    }

    #[test]
    fn test_token_rparen() {
        let mut lexer = Token::lexer(")");
        assert_eq!(lexer.next(), Some(Ok(Token::RParen)));
    }

    #[test]
    fn test_token_identifier() {
        let mut lexer = Token::lexer("identifier");
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Identifier("identifier".to_string())))
        );
    }

    #[test]
    fn test_token_string() {
        let mut lexer = Token::lexer("`string`");
        assert_eq!(lexer.next(), Some(Ok(Token::String("string".to_string()))));
    }

    #[test]
    fn test_token_complex() {
        let mut lexer = Token::lexer("(identifier1 `string1` identifier2 `string2`)");
        assert_eq!(lexer.next(), Some(Ok(Token::LParen)));
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Identifier("identifier1".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(Token::String("string1".to_string()))));
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Identifier("identifier2".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(Token::String("string2".to_string()))));
        assert_eq!(lexer.next(), Some(Ok(Token::RParen)));
    }
}
