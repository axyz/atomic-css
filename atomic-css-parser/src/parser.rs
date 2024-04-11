use crate::lexer::{Lexer, Span, Token};
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use derive_more::Display;
use logos::Logos;

type Error = (String, Span);

pub fn pretty_print_error(error: &Error, src: &str) {
    let (msg, span) = error;
    let mut colors = ColorGenerator::new();
    let a = colors.next();
    Report::build(ReportKind::Error, "<source>", 12)
        .with_message("Invalid document".to_string())
        .with_label(
            Label::new(("<source>", span.clone()))
                .with_message(msg)
                .with_color(a),
        )
        .finish()
        .eprint(("<source>", Source::from(src)))
        .unwrap();
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Display)]
pub enum Node {
    #[display(fmt = "Function({}, {:?})", _0, _1)]
    Function(String, Vec<Node>),
    String(String),
    Identifier(String),
}

fn parse_function(lexer: &mut Lexer<'_, Token>) -> Result<Node> {
    let id;
    let mut nodes = vec![];

    if let Some(Ok(Token::Identifier(name))) = lexer.next() {
        id = name;
    } else {
        return Err(("Identifier expected".to_owned(), lexer.span()));
    }

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::RParen) => return Ok(Node::Function(id.to_string(), nodes)),
            Ok(Token::String(text)) => nodes.push(Node::String(text)),
            Ok(Token::LParen) => nodes.push(parse_function(lexer)?),
            Ok(Token::Identifier(name)) => nodes.push(Node::Identifier(name)),
            _ => return Err(("Invalid token".to_owned(), lexer.span())),
        }
    }

    Err(("Unmatched open parenthesis".to_owned(), lexer.span()))
}

pub fn parse(source: &str) -> Result<Vec<Node>> {
    let mut lexer = Token::lexer(source);
    let mut result: Vec<Node> = vec![];

    while let Some(token) = lexer.next() {
        if let Ok(Token::LParen) = token {
            match parse_function(&mut lexer) {
                Ok(function) => result.push(function),
                Err(error) => {
                    pretty_print_error(&error, source);
                    return Err(error);
                }
            }
            continue;
        }
        let error = ("Invalid token".to_owned(), lexer.span());
        pretty_print_error(&error, source);
        return Err(error);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let source = r##"
(electron `bg_green` (background-color `#00ff00`))
; comment
(electron `red` (color `#ff0000`))
(organism `test` ;comment
  (molecule `flag`
    (atom `root`)
    (atom `label` (import `button.label`))
    (rule `${root}` (padding `1rem`))
    (@foo)
    (@bar `baz`)
    (@media `(min-width: 1024px)`
      (@rule `${root}` (padding `1.5rem`)))))
"##;

        let result = parse(source);
        println!("### {:?}", result);
    }
}
