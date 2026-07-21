use super::*;
use jate_ast::Expr;
use jate_ast::ExprKind;
use jate_ast::expr;
use jate_error::span;
use jate_lexer::LiteralKind;
use jate_lexer::StrPrefix;
use jate_lexer::Token;
use jate_lexer::TokenKind;
use jate_lexer::tokenize;
use jate_session::SourceFile;

macro_rules! vec_str {
    ($($strs:expr),* $(,)?) => {
        vec![$($strs.to_string()),*]
    };
}

macro_rules! assert_expr_eq {
    ($left:expr, $right:expr, $idx:expr) => {
        assert_eq!(
            $left, $right,
            "\nExpression at index {} does not match:\n  left:  {:?}\n  right: {:?}",
            $idx, $left, $right
        );
    };
}

#[test]
fn test_parser_primary() {
    let input = "123 ident::some true 3.14 'c' '\\t' (2)";
    let source = SourceFile::new("main.jate".into(), input.into());
    let tokens = tokenize(input);
    let stream = TokenStream::new(Box::new(tokens));
    let mut cursor = AstCursor {
        stream,
        source: &source,
    };

    let expected = vec![
        expr!(ExprKind::Int(123), span!(0, 3)),
        expr!(ExprKind::Ident(vec_str!("ident", "some")), span!(4, 11)),
        expr!(ExprKind::Bool(true), span!(16, 4)),
        expr!(ExprKind::Float(3.14), span!(21, 4)),
        expr!(ExprKind::Char('c'), span!(26, 3)),
        expr!(ExprKind::Char('\t'), span!(30, 4)),
        expr!(ExprKind::Int(2), span!(36, 1)),
    ];

    let mut actual = Vec::with_capacity(expected.len());
    for (i, expected_expr) in expected.iter().enumerate() {
        match cursor.primary() {
            Some(Ok(expr)) => {
                actual.push(expr.clone());
                assert_expr_eq!(&expr, expected_expr, i);
            }
            Some(Err(err)) => {
                panic!("\x1b[31mParser error at index {}: {:?}\x1b[0m", i, err);
            }
            None => break,
        }
    }

    assert_eq!(actual, expected);
}

#[test]
fn test_word_to_string_format_prefix() {
    let source = "f\"text\"";
    let pref = StrPrefix::Format;
    let token = Token::new(TokenKind::Literal(LiteralKind::String(pref)), 7);

    let result = word_to_string(source, token, 0, pref);

    assert_eq!(
        result,
        Ok(expr!(ExprKind::String("text".to_string()), span!(0, 7)))
    );
}

#[test]
fn test_word_to_string_no_prefix() {
    let source = "\"some\"";
    let pref = StrPrefix::No;
    let token = Token::new(TokenKind::Literal(LiteralKind::String(pref)), 6);

    let result = word_to_string(source, token, 0, pref);

    assert_eq!(
        result,
        Ok(expr!(ExprKind::String("some".to_string()), span!(0, 6)))
    );
}

#[test]
fn test_word_to_char_simple() {
    let source = "'c'";
    let lit = LiteralKind::Char;
    let token = Token::new(TokenKind::Literal(lit), 3);

    let result = word_to_char(source, token, 0);

    assert_eq!(result, Ok(expr!(ExprKind::Char('c'), span!(0, 3))));
}

#[test]
fn test_word_to_char_escape() {
    let source = "'\\t'";
    let lit = LiteralKind::Char;
    let token = Token::new(TokenKind::Literal(lit), 4);

    let result = word_to_char(source, token, 0);

    assert_eq!(result, Ok(expr!(ExprKind::Char('\t'), span!(0, 4))));
}
