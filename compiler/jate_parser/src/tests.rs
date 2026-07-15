mod tests {
    use crate::{TokenCursor, TokenStream, word_to_string};
    use jate_ast::ExprKind;
    use jate_lexer::{LiteralKind, StrPrefix, Token, tokenize};
    use jate_session::SourceFile;

    #[test]
    fn test_all_single_expressions() {
        let input = "a::b::c \"some text\" 1234 -3.4 'c' !true '\\t' ";
        println!("{input}");
        let source = SourceFile::new("main.jate".into(), input.into());
        let lx = tokenize(input);
        let ts = TokenStream::new(Box::new(lx));
        let mut parsed = TokenCursor {
            stream: ts,
            source: &source,
        };

        loop {
            let ex = parsed.advance_expr();
            if ex.is_none() {
                break;
            }
            println!("{:?}", ex);
        }
        assert_eq!(None, parsed.advance_expr());
    }

    #[test]
    fn test_all_binary_expressions() {
        let input = "2 + 2 * 2 / 4";
        println!("{input}");
        let source = SourceFile::new("main.jate".into(), input.into());
        let lx = tokenize(input);
        let ts = TokenStream::new(Box::new(lx));
        let mut parsed = TokenCursor {
            stream: ts,
            source: &source,
        };

        loop {
            let ex = parsed.advance_expr();
            if ex.is_none() {
                break;
            }
            println!("{:?}", ex);
        }
        assert_eq!(None, parsed.advance_expr());
    }
}
