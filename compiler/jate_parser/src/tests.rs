use super::*;
use jate_ast::ExprKind::*;
use jate_ast::*;
use jate_error::*;
use jate_lexer::*;
use jate_session::SourceFile;

macro_rules! vec_str {
    ($($strs:expr),* $(,)?) => {
        vec![$($strs.to_string()),*]
    };
}

#[test]
fn test_parser_primary() {
    let input = "123 ident::some true 3.14 'c' '\\t' (2)";
    println!("\x1b[32mSource:\x1b[0m {input}");
    let source = SourceFile::new("main.jate".into(), input.into());
    let lx = tokenize(input);
    let ts = TokenStream::new(Box::new(lx));
    let mut parsed = AstCursor {
        stream: ts,
        source: &source,
    };
    let mut exprs = vec![];
    let expected = vec![
        expr!(Int(123), span!(0, 3)),
        expr!(Ident(vec_str!("ident", "some")), span!(4, 11)),
        expr!(Bool(true), span!(16, 4)),
        expr!(Float(3.14), span!(21, 4)),
        expr!(Char('c'), span!(26, 3)),
        expr!(Char('\t'), span!(30, 4)),
        expr!(Int(2), span!(36, 1)),
    ];
    for i in 0..expected.len() {
        match parsed.primary() {
            Some(res) => match res {
                Ok(expr) => {
                    exprs.push(expr.clone());
                    println!("\x1b[32m{i}.\x1b[0m {}", expr);
                    assert_eq!(expr, expected[i]);
                }
                Err(err) => {
                    eprintln!("\x1b[34m{i}.\x1b[0m {:?}", err);
                }
            },
            None => break,
        }
    }
    assert_eq!(exprs, expected);
}

#[test]
fn test_word_to_string() {
    let pref = StrPrefix::Format;
    let token = Token::new(TokenKind::Literal(LiteralKind::String(pref)), 7);
    let source = "f\"text\"";
    let extracted = word_to_string(source, token, 0, pref);
    assert_eq!(
        extracted,
        Ok(expr!(ExprKind::String("text".to_string()), span!(0, 7)))
    );
}

/*#[test]
fn test_all_single_expressions() {
    let input = "3.4 a::b::c someNullable!? \"some text\" 1234 'c' !true '\\t'";
    let exprs = parse_test(input);
    let expected = vec![
        // 3.4
        expr!(ExprKind::Float(3.4), span!(0, 3)),
        // a::b::c
        expr!(
            ExprKind::Ident(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
            span!(4, 7)
        ),
        // someNullable!?
        expr!(
            ExprKind::Unwrap(expr!(
                ExprKind::Ident(vec!["someNullable".to_string()]),
                span!(12, 12)
            )),
            span!(12, 14)
        ),
        // "some text"
        expr!(ExprKind::String("some text".to_string()), span!(27, 11)),
        // 1234
        expr!(ExprKind::Int(1234), span!(39, 4)),
        // 'c'
        expr!(ExprKind::Char('c'), span!(44, 3)),
        // !true
        expr!(
            ExprKind::Unary(UnOp::Not, expr!(ExprKind::Bool(true), span!(49, 4))),
            span!(48, 5)
        ),
        // '\t'
        expr!(ExprKind::Char('\t'), span!(54, 4)),
    ];
    assert_eq!(exprs, expected);
}*/

/*#[test]
fn test_all_binary_expressions() {
    let input = "2 + 2 * 2 / 4 - 1";
    let exprs = parse_test(input);

    let expected = vec![Expr {
        kind: Box::new(ExprKind::Bin(
            Expr {
                kind: Box::new(ExprKind::Int(2)),
                span: span!(0, 1),
            },
            BinOp::Add,
            Expr {
                kind: Box::new(ExprKind::Bin(
                    Expr {
                        kind: Box::new(ExprKind::Bin(
                            Expr {
                                kind: Box::new(ExprKind::Int(2)),
                                span: span!(4, 1),
                            },
                            BinOp::Mul,
                            Expr {
                                kind: Box::new(ExprKind::Int(2)),
                                span: span!(8, 1),
                            },
                        )),
                        span: span!(4, 6),
                    },
                    BinOp::Div,
                    Expr {
                        kind: Box::new(ExprKind::Int(4)),
                        span: span!(12, 1),
                    },
                )),
                span: span!(4, 10),
            },
        )),
        span: span!(0, 18),
    }];

    assert_eq!(exprs, expected);
}
*/
