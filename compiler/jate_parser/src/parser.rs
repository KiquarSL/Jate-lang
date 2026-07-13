use crate::{TokenItem, TokenStream};
use jate_ast::{Expr, Stmt};
use jate_error::{Diagnostic, diag, span};

/// Result wrapper for get statement when successful parse and return vector of diagnostics when errors
pub type StmtItem = Result<Stmt, Vec<Diagnostic>>;
pub type ExprItem = Result<Expr, Diagnostic>;

pub fn parse(
    stream: TokenStream<impl Iterator<Item = TokenItem>>,
    source: &str,
) -> impl Iterator<Item = StmtItem> {
    let cursor = TokenCursor { stream, source };
    return std::iter::from_fn(move || cursor.advance_stmt());
}

pub struct TokenCursor<I, 'a> {
    stream: TokenStream<I>,
    source: &'a str,
}

impl<I> TokenCursor<'_, I> {
    pub fn advance_stmt(&self) -> Option<StmtItem> {
        todo!()
    }

    pub(crate) fn advance_expr(&self) -> Option<Expr> {
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
            Some(Ok(token)) => {
                todo!()
            }
            Some(Err(err)) => Err(err),
            None => Err(diag!(
                "Value for expression not found",
                "E0006",
                span!(self.stream.pos, 1)
            )),
        }
    }
}
