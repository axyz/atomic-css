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

    #[regex("[@a-zA-Z][a-zA-z0-9-_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex(r#"`([^`])*`"#, |lex| lex.slice()[1..lex.slice().len() - 1].to_owned())]
    String(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_lexes() {
        let lex = Token::lexer(
            r#"
; comment
(molecule `flag` ;;another comment
  (atom `root`)
  (atom `label` (import `button/label`))
  ;yet another comment
  (rule `${root}` (padding `1rem`))
  (@foo)
  (@bar `baz`)
  (@media `(min-width: 1024px)`
    (rule `${root}` (padding `1.5rem`))))
"#,
        );

        for token in lex {
            //println!("{:?}", token)
        }
    }
}
