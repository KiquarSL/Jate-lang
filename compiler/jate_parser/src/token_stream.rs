use jate_error::{Diagnostic, diag, span};
use jate_lexer::{LiteralKind, Token, TokenKind};

/// Type of token
/// Option wrapper for check ends of stream
/// Result wrapper for get token or diagnostic
pub type TokenItem = Option<Result<Token, Diagnostic>>;

/// TokenStream is wrapper for token iterator from crate `jate_lexer`
/// Can be using for parse tokens with get errors
pub struct TokenStream<I> {
    stream: I,
    pub pos: u32,
}

impl<I> TokenStream<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(stream: I) -> Self {
        Self { stream, pos: 0 }
    }

    /// Lazy get token item or error if invalid token
    pub fn next(&mut self) -> TokenItem {
        if let Some(token) = self.stream.next() {
            let result = Some(self.check(token));
            self.pos += token.len;
            return result;
        } else {
            None
        }
    }

    /// Check token
    /// Return token if success
    /// Return Diagnostic if failure
    fn check(&self, token: Token) -> Result<Token, Diagnostic> {
        match token.kind {
            TokenKind::Literal(LiteralKind::InvalidChar) => Err(diag!(
                "Invalid char literal",
                "E0001",
                span!(self.pos, token.len)
            )),
            _ => Ok(token),
        }
    }

    /// Eat all tokens while not found `kind`
    pub fn eat(&mut self, kind: TokenKind) -> Vec<Diagnostic> {
        let mut diags = vec![];
        while let Some(token) = self.next() {
            match token {
                Ok(ok) => {
                    if ok.kind == kind {
                        break;
                    }
                }
                Err(diag) => {
                    diags.push(diag);
                }
            }
        }
        return diags;
    }
}
