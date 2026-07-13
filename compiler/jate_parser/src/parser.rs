use crate::TokenStream;
use jate_ast::{Expr, Stmt, StmtKind, ExprKind, expr, stmt};
use jate_error::{Diagnostic, diag, span};
use jate_lexer::{LiteralKind, TokenKind};

/// Result wrapper for get statement when successful parse and return vector of diagnostics when errors
pub type StmtItem = Result<Stmt, Vec<Diagnostic>>;
pub type ExprItem = Result<Expr, Diagnostic>;

pub fn parse(stream: TokenStream, source: &str) -> impl Iterator<Item = StmtItem> {
    let mut cursor = TokenCursor { stream, source };
    return std::iter::from_fn(move || cursor.advance_stmt());
}

pub struct TokenCursor<'a> {
    stream: TokenStream,
    source: &'a str,
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

    fn primary(&mut self) -> ExprItem {
        match self.stream.advance() {
            Some(Ok(token)) => match token.kind {
                _ => todo!(),
            },
            Some(Err(err)) => Err(err),
            None => Err(diag!(
                "Value for expression not found",
                "E0006",
                span!(self.stream.pos, 1)
            )),
        }
    }
}
