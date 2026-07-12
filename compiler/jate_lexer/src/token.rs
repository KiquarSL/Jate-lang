//! Implements of token and token kind

/// Parsed token.
/// Does't have position
pub struct Token {
    pub len: u32,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum TokenKind {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `::`
    Path,
    /// `;`
    Semi,
    /// `!`
    Bang,
    /// `!?`
    Unwrap,
    /// `?`
    Question,
    /// `=`
    Assign,
    /// `:=`
    Declare,
    /// `!=`
    Ne,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// Identifier or keyword, e.g., `continue` or `myVar`
    Ident,
    /// Constant value, see LiteralKind
    Literal(LiteralKind),
    /// End of file
    Eof,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LiteralKind {
    /// Integer number, e.g., 1024 86
    Int,
    /// Float number, e.g., 3.1412 5863.1
    Float,
    /// String, e.g., "Hello"
    String,
    /// Format string, e.g., f"Hello, {name}!"
    FormatString,
}
