//! Implements of token and token kind

/// Parsed token.
/// Does't have position
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub len: u32,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }
}

/// Raw token kinds
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `&`
    Amp,
    /// `|`
    Pipe,
    /// `&&`
    And,
    /// `||`
    Or,
    /// `->`
    Arrow,
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
    /// `==`
    Eq,
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
    /// `..`
    Range,
    /// `..=`
    RangeInclude,
    /// `^`
    Caret,
    /// Identifier or keyword, e.g., `continue` or `myVar`
    Ident,
    /// Invalid keyword, e.g., emoji
    Invalid,
    /// Line comment, starts with `//`
    LineComment,
    /// Block comment, starts with `/*`, ends with `*/`
    BlockComment,
    /// All whitespace kind, e.g., `\t`, `\n`
    Whitespace,
    /// Constant value, see LiteralKind
    Literal(LiteralKind),
    /// End of file
    Eof,
}

/// Value types using as literal
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LiteralKind {
    /// Integer number, e.g., `1024` `86`
    Int,
    /// Float number, e.g., `3.1412` `5863.1`
    Float,
    /// Invalid float number, e.g., `3.141.7.3`
    InvalidFloat,
    /// String, e.g., `"Hello"`
    String(StrPrefix),
    /// Char, e.g., `'\n'` `'j'`
    Char,
    /// Char, e.g., `'\\n'`
    InvalidChar,
    /// Char, e.g., `'j`
    UnterminatedChar,
    /// String, e.g., `"string`
    UnterminatedString,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StrPrefix {
    No,
    /// `f` - string
    Format,
    /// `r` - string
    Raw,
}
