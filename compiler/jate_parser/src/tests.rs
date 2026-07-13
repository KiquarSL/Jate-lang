mod tests {
    use crate::{TokenCursor, TokenStream, word_to_string};
    use jate_ast::ExprKind;
    use jate_lexer::{LiteralKind, StrPrefix, TokenKind, Token, tokenize};
    use jate_session::SourceFile;
    #[test]
    fn test_primary() {
        let input = "\"some text\" 1234 3.4 'c' '\\t'";
        println!("{input}");
        let source = SourceFile::new("main.jate".into(), input.into());
        let lx = tokenize(input);
        let ts = TokenStream::new(Box::new(lx));
        let mut parsed = TokenCursor {
            stream: ts,
            source: &source,
        };

        while let Ok(ex) = parsed.primary() {
            let string_value = match *ex.clone().kind {
                ExprKind::String(s) => s.to_string(),
                _ => String::new(),
            };
            println!(
                "{:?} :: {:?}",
                ex.clone(),
                string_value
            );

            parsed.stream.advance();
        }
    }
}
