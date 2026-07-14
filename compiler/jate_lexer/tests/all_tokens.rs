use TokenKind::*;
use jate_lexer::*;

static EXPECTED_KINDS: [TokenKind; 32] = [
    Literal(LiteralKind::Char),
    Literal(LiteralKind::Char),
    Semi,
    Caret,
    Bang,
    Ne,
    Unwrap,
    Colon,
    Path,
    Declare,
    Plus,
    Minus,
    Star,
    Slash,
    Range,
    RangeInclude,
    Dot,
    Assign,
    Eq,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Literal(LiteralKind::Int),
    Literal(LiteralKind::Float),
    Ident,
    Ident,
    BlockComment,
    Literal(LiteralKind::String(StrPrefix::No)),
    LineComment,
];

#[test]
fn main() {
    let source = "'\\t' 'j' ; ^ ! != !? : :: := + - * / .. ..= . = == ( ) { } [ ] 124 3.1412 ident continue /* comment */ \"string\" // line comment";
    let mut position = 0;
    let mut i = 0;
    for token in tokenize(source) {
        println!("{:?} in {}", token, position);
        position += token.len;
        if token.kind == Whitespace {
            continue;
        }
        assert!(token.kind == EXPECTED_KINDS[i]);
        i += 1;
    }
    println!(
        "Expected length: {}\nLength of tokenized: {position}",
        source.len()
    );
}
