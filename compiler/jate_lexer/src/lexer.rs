use crate::{LiteralKind, Prefix, Token, TokenKind};
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

        if is_whitespace(first) {
            return self.whitespace();
        }

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
            '>' => self.gt(),
            '<' => self.lt(),
            '/' => self.slash(),
            '!' => self.bang(),
            '.' => self.dot(),
            ':' => self.colon(),
            '=' => self.assign(),
            '\'' => self.char_lit(),
            '"' => self.string(),
            'a'..='z' | 'A'..='Z' => self.ident(),
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

    fn first(&self) -> Option<char> {
        // next() better optimized then nth(0)
        self.chars.clone().next()
    }

    fn second(&self) -> Option<char> {
        // next() better optimized then nth(1)
        let mut iter = self.chars.clone();
        iter.next();
        iter.next()
    }

    /// Skip all types whitespaces
    fn whitespace(&mut self) -> Token {
        while let Some(first) = self.bump() {
            if !is_whitespace(first) {
                break;
            }
        }
        let token_len = self.token_len;
        self.token_len = 0;
        return Token::new(TokenKind::Whitespace, token_len);
    }

    /// `!` `!?` `!=`
    fn bang(&mut self) -> TokenKind {
        let Some(first) = self.first() else {
            return TokenKind::Eof;
        };
        match first {
            '?' => {
                self.bump();
                return TokenKind::Unwrap;
            }
            '=' => {
                self.bump();
                return TokenKind::Ne;
            }
            _ => TokenKind::Bang,
        }
    }

    /// `'c'`
    fn char_lit(&mut self) -> TokenKind {
        let Some(first) = self.bump() else {
            return TokenKind::Eof;
        };
        if first == '\\' {
            self.bump();
            self.bump();
        } else {
            self.bump();
        }
        if let Some(first) = self.bump() {
            if first != '\'' {
                return TokenKind::Literal(LiteralKind::InvalidChar);
            }
            return TokenKind::Literal(LiteralKind::Char);
        } else {
            return TokenKind::Literal(LiteralKind::InvalidChar);
        }
    }

    /// `.` `..` `..=
    fn dot(&mut self) -> TokenKind {
        let Some(first) = self.bump() else {
            return TokenKind::Eof;
        };
        if first == '.' {
            let Some(second) = self.second() else {
                return TokenKind::Range;
            };
            match second {
                '=' => {
                    self.bump();
                    self.bump();
                    return TokenKind::RangeInclude;
                }
                _ => TokenKind::Range,
            }
        } else {
            return TokenKind::Dot;
        }
    }

    /// `/` `//` `/*`
    fn slash(&mut self) -> TokenKind {
        todo!()
    }

    /// `:` `:=` `::`
    fn colon(&mut self) -> TokenKind {
        todo!()
    }

    /// `=` `==`
    fn assign(&mut self) -> TokenKind {
        todo!()
    }

    /// `"..."`
    fn string(&mut self) -> TokenKind {
        todo!()
    }

    /// `123` `123.45`
    fn number(&mut self) -> TokenKind {
        todo!()
    }

    /// Identifier or keyword, e.g., `continue` or `myVar`
    fn ident(&mut self) -> TokenKind {
        todo!()
    }

    /// `>` `>=`
    fn gt(&mut self) -> TokenKind {
        todo!()
    }

    /// `<` `<=`
    fn lt(&mut self) -> TokenKind {
        todo!()
    }
}
