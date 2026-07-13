use crate::TokenStream;
use jate_ast::{Expr, ExprKind, Stmt, StmtKind, expr, stmt};
use jate_error::{Diagnostic, diag, span};
use jate_lexer::{LiteralKind, Token, TokenKind};
use jate_session::SourceFile;

/// Result wrapper for get statement when successful parse and return vector of diagnostics when errors
pub type StmtItem = Result<Stmt, Vec<Diagnostic>>;
pub type ExprItem = Result<Expr, Diagnostic>;

pub fn parse(stream: TokenStream, source: &SourceFile) -> impl Iterator<Item = StmtItem> {
    let mut cursor = TokenCursor { stream, source };
    return std::iter::from_fn(move || cursor.advance_stmt());
}

pub struct TokenCursor<'a> {
    stream: TokenStream,
    source: &'a SourceFile,
}

impl<'a> TokenCursor<'a> {
    pub fn advance_stmt(&mut self) -> Option<StmtItem> {
        todo!()
    }

    pub(crate) fn advance_expr(&mut self) -> Option<Expr> {
        todo!()
    }

    fn expr(&mut self) -> ExprItem {
        todo!()
    }

    fn logical(&mut self) -> ExprItem {
        todo!()
    }

    fn comparison(&mut self) -> ExprItem {
        todo!()
    }

    fn additive(&mut self) -> ExprItem {
        todo!()
    }

    fn multiplicative(&mut self) -> ExprItem {
        todo!()
    }

    fn unary(&mut self) -> ExprItem {
        todo!()
    }

    /// Parse value or error
    fn primary(&mut self) -> ExprItem {
        match self.stream.first() {
            Some(Ok(token)) => {
                let pos = self.stream.current_pos(token.len);
                let s = self.source.get_word(pos, token.len);
                match token.kind {
                    TokenKind::Literal(LiteralKind::Int) => {
                        self.stream.advance();
                        word_to_int(&s, token, pos)
                    }
                    TokenKind::Literal(LiteralKind::Float) => {
                        self.stream.advance();
                        word_to_float(&s, token, pos)
                    }
                    TokenKind::Literal(LiteralKind::Char) => {
                        self.stream.advance();
                        word_to_char(&s, token, pos)
                    }
                    TokenKind::Ident => {
                        let span = span!(pos, token.len);
                        Ok(match s.as_str() {
                            "true" => expr!(ExprKind::Bool(true), span),
                            "false" => expr!(ExprKind::Bool(false), span),
                            "null" => expr!(ExprKind::Null, span),
                            _ => self.parse_ident(pos)?,
                        })
                    }
                    _ => todo!(),
                }
            }
            Some(Err(err)) => Err(err),
            None => Err(diag!(
                "E0006",
                span!(self.stream.pos, 1),
                "Value for expression not found"
            )),
        }
    }

    /// Parse ident as vector of segments, e.g., `path::to::some`
    // TODO: add handle for calls
    pub(crate) fn parse_ident(&mut self, pos: u32) -> Result<Expr, Diagnostic> {
        let mut path = vec![];
        while let Some(token_result) = self.stream.first() {
            match token_result {
                Ok(token) => {
                    if token.kind == TokenKind::Path {
                        self.stream.advance();
                    } else if token.kind == TokenKind::Ident {
                        let ident = self.source.get_word(pos, token.len).to_string();
                        path.push(ident);
                        self.stream.advance();
                    } else {
                        break;
                    }
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr!(
            ExprKind::Ident(path),
            span!(self.stream.pos, self.stream.pos - pos)
        ))
    }
}

fn word_to_int(s: &str, token: Token, pos: u32) -> Result<Expr, Diagnostic> {
    match s.parse::<i64>() {
        Ok(num) => Ok(expr!(ExprKind::Int(num), span!(pos, token.len))),
        Err(err) => Err(diag!(
            "E0007",
            span!(pos, token.len),
            "Failed to get int from source: {err}"
        )),
    }
}

fn word_to_float(s: &str, token: Token, pos: u32) -> Result<Expr, Diagnostic> {
    match s.parse::<f64>() {
        Ok(num) => Ok(expr!(ExprKind::Float(num), span!(pos, token.len))),
        Err(err) => Err(diag!(
            "E0008",
            span!(pos, token.len),
            "Failed to get float from source: {err}"
        )),
    }
}

fn word_to_char(s: &str, token: Token, pos: u32) -> Result<Expr, Diagnostic> {
    let c = &s[1..token.len as usize - 1];
    match c.parse::<char>() {
        Ok(c) => Ok(expr!(ExprKind::Char(c), span!(pos, token.len))),
        Err(err) => Err(diag!(
            "E0009",
            span!(pos, token.len),
            "Failed to get char from source: {err}"
        )),
    }
}
