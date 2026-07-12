use crate::{LiteralKind, StringPrefix, Token, TokenKind};
use std::str::Chars;

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let mut cursor = Cursor::new(input);
    return std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    });
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        // End-of-line characters
        | '\u{000A}' // line feed (\n)
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // carriage return (\r)
        | '\u{0085}' // next line (from latin1)
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR

        // `Default_Ignorable_Code_Point` characters
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Horizontal space characters
        | '\u{0009}' // tab (\t)
        | '\u{0020}' // space
    )
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

        let kind = match first {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '^' => TokenKind::Caret,
            ';' => TokenKind::Semi,
            '?' => TokenKind::Question,
            '>' => self.gt_or_ge(),
            '<' => self.lt_or_le(),
            '/' => self.comment_or_slash(),
            '!' => self.bang_or_unwrap_or_ne(),
            '.' => self.dot_or_range(),
            ':' => self.colon_or_declare_or_path(),
            '=' => self.assign_or_eq(),
            '\'' => self.character(),
            '"' => self.string(),
            'a'..='z' | 'A'..='Z' => self.prefix_or_ident(),
            '0'..='9' => self.number(),
            _ => TokenKind::Invalid,
        };
        let token_len = self.token_len;
        self.token_len = 0;
        return Token::new(kind, token_len);
    }

    fn bump(&mut self) -> Option<char> {
        self.token_len += 1;
        self.chars.next()
    }

    /// `!` `!?` `!=`
    fn bang_or_unwrap_or_ne(&mut self) -> TokenKind {
        todo!()
    }

    /// `'c'`
    fn character(&mut self) -> TokenKind {
        todo!()
    }

    /// `.` `..` `..=
    fn dot_or_range(&mut self) -> TokenKind {
        todo!()
    }

    fn comment_or_slash(&mut self) -> TokenKind {
        todo!()
    }

    fn colon_or_declare_or_path(&mut self) -> TokenKind {
        todo!()
    }

    fn assign_or_eq(&mut self) -> TokenKind {
        todo!()
    }

    fn string(&mut self) -> TokenKind {
        todo!()
    }

    fn number(&mut self) -> TokenKind {
        todo!()
    }

    fn prefix_or_ident(&mut self) -> TokenKind {
        todo!()
    }

    fn gt_or_ge(&mut self) -> TokenKind {
        todo!()
    }

    fn lt_or_le(&mut self) -> TokenKind {
        todo!()
    }
}
