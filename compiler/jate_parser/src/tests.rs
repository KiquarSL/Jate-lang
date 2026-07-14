mod tests {
    use crate::{TokenCursor, TokenStream, word_to_string};
    use jate_ast::ExprKind;
    use jate_lexer::{LiteralKind, StrPrefix, Token, TokenKind, tokenize};
    use jate_session::SourceFile;
    #[test]
    fn test_primary() {
        let input = "a::b::c \"some text\" 1234 3.4 'c' '\\t' ";
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
        println!("{:?}", parsed.advance_expr());
    }
}
