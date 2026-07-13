use crate::{TokenItem, TokenStream};
use jate_ast::Stmt;
use jate_error::Diagnostic;

/// Result wrapper for get statement when successful parse and return vector of diagnostics when errors
pub type StmtItem = Result<Stmt, Vec<Diagnostic>>;

pub fn parse(
    stream: TokenStream<impl Iterator<Item = TokenItem>>,
) -> impl Iterator<Item = StmtItem> {
    let cursor = TokenCursor { stream };
    return std::iter::from_fn(move || cursor.advance_stmt());
}

pub struct TokenCursor<I> {
    stream: TokenStream<I>,
}

impl<I> TokenCursor<I> {
    pub fn advance_stmt(&self) -> Option<StmtItem> {
        todo!()
    }
}
