use crate::{LiteralKind, Token, TokenKind};
use std::str::Chars;

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let cursor = Cursor::new(input);
    return std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    });
}

pub struct Cursor<'a> {
    chars: Chars<'a>,
    /// Current token length
    token_len: u32,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            token_len: 0,
        }
    }

    pub fn advance_token(&mut self) -> Token {
        let Some(first) = self.bump() else {
            return Token::new(TokenKind::Eof, 0);
        };

        match first {
            _ => todo!(),
        }
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }
}
