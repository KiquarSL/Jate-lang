mod lexer;
mod token;

pub use lexer::tokenize;
pub use token::{LiteralKind, StrPrefix, Token, TokenKind};
