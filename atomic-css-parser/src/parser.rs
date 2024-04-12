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
    fn test_parse_function_valid() {
        let result = parse("(myFunc arg1 arg2)");
        assert_eq!(
            result,
            Ok(vec![Node::Function(
                "myFunc".to_string(),
                vec![
                    Node::Identifier("arg1".to_string()),
                    Node::Identifier("arg2".to_string())
                ]
            )])
        );
    }

    #[test]
    fn test_parse_function_invalid() {
        let result = parse("(1234)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_function_unmatched() {
        let result = parse("(myFunc arg1 arg2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty() {
        let result = parse("");
        assert_eq!(result, Ok(vec![]));
    }

    #[test]
    fn test_parse_empty_function_invalid() {
        let result = parse("()");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_non_function() {
        let number = parse("1234");
        assert!(number.is_err());

        let string = parse("`string`");
        assert!(string.is_err());

        let identifier = parse("identifier");
        assert!(identifier.is_err());
    }

    #[test]
    fn test_node_string_valid() {
        let result = parse("(test `valid string`)");
        assert_eq!(
            result,
            Ok(vec![Node::Function(
                "test".to_string(),
                vec![Node::String("valid string".to_string())]
            )])
        );
    }

    #[test]
    fn test_node_string_invalid() {
        let result = parse("(test invalid string)");
        assert_ne!(
            result,
            Ok(vec![Node::Function(
                "test".to_string(),
                vec![Node::String("invalid string".to_string())]
            )])
        );
    }

    #[test]
    fn test_node_identifier_valid() {
        let result = parse("(validIdentifier)");
        assert_eq!(
            result,
            Ok(vec![Node::Function("validIdentifier".to_string(), vec![])])
        );
    }

    #[test]
    fn test_node_identifier_dash() {
        let result = parse("(valid-identifier)");
        assert_eq!(
            result,
            Ok(vec![Node::Function("valid-identifier".to_string(), vec![])])
        );
    }

    #[test]
    fn test_node_identifier_at() {
        let result = parse("(@identifier)");
        assert_eq!(
            result,
            Ok(vec![Node::Function("@identifier".to_string(), vec![])])
        );
    }

    #[test]
    fn test_node_identifier_ampersand() {
        let result = parse("(&)");
        assert_eq!(result, Ok(vec![Node::Function("&".to_string(), vec![])]));
    }

    #[test]
    fn test_node_identifier_invalid() {
        let result = parse("(1234)");
        assert_ne!(result, Ok(vec![Node::Function("1234".to_string(), vec![])]));
    }

    #[test]
    fn it_parses() {
        let source = r##"
(electron `red` (color `#ff0000`))
(electron `blue` (color `#0000ff`))
(electron `bg_green` (background-color `#00ff00`))

(molecule `button`
  (atom `label` (electrons `blue`)))

(molecule `flag`
  (atom `root` (electrons `red` `bg_green`))
  (atom `label` (import `button` `label`))
  (& `${root}`
    (padding `1rem`)
    (margin `0`))
  (@ `foo`)
  (@ `bar` `baz`)
  (@ `media` `(min-width: 1024px)`
    (& `${root}` (padding `1.5rem`))))

"##;

        let result = parse(source);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 5);
    }
}
