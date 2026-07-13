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
        match self.stream.advance() {
            Some(Ok(token)) => {
                let pos = self.stream.current_pos(token.len);
                let s = self.source.get_word(pos, token.len);
                match token.kind {
                    TokenKind::Literal(LiteralKind::Int) => word_to_int(&s, &token, pos),
                    TokenKind::Literal(LiteralKind::Float) => word_to_float(&s, &token, pos),
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
}

fn word_to_int(s: &str, token: &Token, pos: u32) -> Result<Expr, Diagnostic> {
    match s.parse::<i64>() {
        Ok(num) => Ok(expr!(ExprKind::Int(num), span!(pos, token.len))),
        Err(err) => Err(diag!(
            "E0007",
            span!(pos, token.len),
            "Failed to get int from source: {err}"
        )),
    }
}

fn word_to_float(s: &str, token: &Token, pos: u32) -> Result<Expr, Diagnostic> {
    match s.parse::<f64>() {
        Ok(num) => Ok(expr!(ExprKind::Float(num), span!(pos, token.len))),
        Err(err) => Err(diag!(
            "E0007",
            span!(pos, token.len),
            "Failed to get int from source: {err}"
        )),
    }
}
