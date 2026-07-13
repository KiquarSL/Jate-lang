use jate_error::{Diagnostic, diag, span};
use jate_lexer::{LiteralKind, Token, TokenKind};

/// Type of token
/// Option wrapper for check ends of stream
/// Result wrapper for get token or diagnostic
pub type TokenItem = Option<Result<Token, Diagnostic>>;

/// TokenStream is wrapper for token iterator from crate `jate_lexer`
/// Can be using for parse tokens with get errors
pub struct TokenStream {
    stream: Box<dyn Iterator<Item = Token>>,
    pub pos: u32,
}

impl TokenStream {
    pub fn new(stream: Box<dyn Iterator<Item = Token>>) -> Self {
        Self { stream, pos: 0 }
    }

    /// Lazy get token item or error if invalid token
    pub fn advance(&mut self) -> TokenItem {
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
            TokenKind::Literal(LiteralKind::UnterminatedChar) => Err(diag!(
                "Unterminated char",
                "E0001",
                span!(self.pos, token.len)
            )),
            TokenKind::Literal(LiteralKind::InvalidChar) => Err(diag!(
                "Invalid char literal",
                "E0002",
                span!(self.pos, token.len)
            )),
            TokenKind::Literal(LiteralKind::UnterminatedString) => Err(diag!(
                "Unterminated string",
                "E0003",
                span!(self.pos, token.len)
            )),
            TokenKind::Invalid => Err(diag!("Invalid symbol", "E0004", span!(self.pos, token.len))),
            TokenKind::Literal(LiteralKind::InvalidFloat) => Err(diag!(
                "Invalid float literal",
                "E0005",
                span!(self.pos, token.len)
            )),
            _ => Ok(token),
        }
    }

    /// Eat all tokens while not found `kind`
    /// Using for skip tokens
    pub fn eat(&mut self, kind: TokenKind) -> Vec<Diagnostic> {
        let mut diags = vec![];
        while let Some(token) = self.advance() {
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
