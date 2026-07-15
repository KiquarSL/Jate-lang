mod tests {
    use crate::{AstCursor, TokenStream};
    use jate_ast::ExprKind;
    use jate_lexer::{LiteralKind, StrPrefix, tokenize};
    use jate_session::SourceFile;

    #[test]
    fn test_all_single_expressions() {
        let input = "a::b::c!? \"some text\" 1234 -3.4 'c' !true '\\t'";
        println!("{input}");
        let source = SourceFile::new("main.jate".into(), input.into());
        let lx = tokenize(input);
        let ts = TokenStream::new(Box::new(lx));
        let mut parsed = AstCursor {
            stream: ts,
            source: &source,
        };

        loop {
            let ex = match parsed.advance_expr() {
                Some(res) => match res {
                    Ok(expr) => expr,
                    Err(err) => {
                        eprintln!("{:?}", err);
                        break;
                    }
                },
                None => break,
            };
            println!("{:?}", ex);
        }
    }

    #[test]
    fn test_all_binary_expressions() {
        let input = "2 + 2 * 2 / 4 - 1";
        println!("{input}");
        let source = SourceFile::new("main.jate".into(), input.into());
        let lx = tokenize(input);
        let ts = TokenStream::new(Box::new(lx));
        let mut parsed = AstCursor {
            stream: ts,
            source: &source,
        };

        loop {
            let ex = match parsed.advance_expr() {
                Some(res) => match res {
                    Ok(expr) => expr,
                    Err(err) => {
                        eprintln!("{:?}", err);
                        break;
                    }
                },
                None => break,
            };
            println!("{:?}", ex);
        }
    }
}
