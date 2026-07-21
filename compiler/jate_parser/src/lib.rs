mod parser;
mod token_stream;

#[cfg(test)]
mod tests;

pub use parser::{StmtItem, parse};
pub use token_stream::{TokenItem, TokenStream};

#[allow(unused_imports)]
pub(crate) use {
    parser::AstCursor,
    parser::{escape_symbol, word_to_char, word_to_float, word_to_int, word_to_string},
};
