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
    next_token: Option<TokenItem>,
    pub pos: u32,
}

impl TokenStream {
    pub fn new(stream: Box<dyn Iterator<Item = Token>>) -> Self {
        let mut s = Self {
            stream,
            pos: 0,
            next_token: None,
        };
        s.advance();
        return s;
    }

    /// Lazy get token item or error if invalid token
    pub fn advance(&mut self) -> TokenItem {
        let current = self.first();
        if let Some(token) = self.stream.next() {
            self.next_token = Some(Some(self.check(token)));
            self.pos += token.len;
            return current;
        } else {
            self.next_token = None;
            current
        }
    }

    pub fn first(&mut self) -> TokenItem {
        match &self.next_token {
            Some(item) => item.clone(),
            None => None,
        }
    }

    /// Using in parser for eval positon of current token
    /// Using after advance!
    pub fn current_pos(&self, len: u32) -> u32 {
        self.pos - len
    }

    /// Check token
    /// Return token if success
    /// Return Diagnostic if failure
    fn check(&self, token: Token) -> Result<Token, Diagnostic> {
        match token.kind {
            TokenKind::Literal(LiteralKind::UnterminatedChar) => Err(diag!(
                "E0001",
                span!(self.pos, token.len),
                "Unterminated char"
            )),
            TokenKind::Literal(LiteralKind::InvalidChar) => Err(diag!(
                "E0002",
                span!(self.pos, token.len),
                "Invalid char literal"
            )),
            TokenKind::Literal(LiteralKind::UnterminatedString) => Err(diag!(
                "E0003",
                span!(self.pos, token.len),
                "Unterminated string"
            )),
            TokenKind::Invalid => Err(diag!("E0004", span!(self.pos, token.len), "Invalid symbol")),
            TokenKind::Literal(LiteralKind::InvalidFloat) => Err(diag!(
                "E0005",
                span!(self.pos, token.len),
                "Invalid float literal"
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
