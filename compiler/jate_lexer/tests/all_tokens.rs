use TokenKind::*;
use jate_lexer::*;

static EXPECTED_KINDS: [(TokenKind, u32, u32); 32] = [
    (Literal(LiteralKind::Char), 4, 0),                    // '\t'
    (Literal(LiteralKind::Char), 3, 5),                    // 'j'
    (Semi, 1, 9),                                          // ;
    (Caret, 1, 11),                                        // ^
    (Bang, 1, 13),                                         // !
    (Ne, 2, 15),                                           // !=
    (Unwrap, 2, 18),                                       // !?
    (Colon, 1, 21),                                        // :
    (Path, 2, 23),                                         // ::
    (Declare, 2, 26),                                      // :=
    (Plus, 1, 29),                                         // +
    (Minus, 1, 31),                                        // -
    (Star, 1, 33),                                         // *
    (Slash, 1, 35),                                        // /
    (Range, 2, 37),                                        // ..
    (RangeInclude, 3, 40),                                 // ..=
    (Dot, 1, 44),                                          // .
    (Assign, 1, 46),                                       // =
    (Eq, 2, 48),                                           // ==
    (LParen, 1, 51),                                       // (
    (RParen, 1, 53),                                       // )
    (LBrace, 1, 55),                                       // {
    (RBrace, 1, 57),                                       // }
    (LBracket, 1, 59),                                     // [
    (RBracket, 1, 61),                                     // ]
    (Literal(LiteralKind::Int), 3, 63),                    // 124
    (Literal(LiteralKind::Float), 6, 67),                  // 3.1412
    (Ident, 5, 74),                                        // ident
    (Ident, 8, 80),                                        // continue
    (BlockComment, 13, 89),                                // /* comment */
    (Literal(LiteralKind::String(StrPrefix::No)), 8, 103), // "string"
    (LineComment, 16, 112),                                // // line comment
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
        let (expected_kind, expected_len, expected_pos) = EXPECTED_KINDS[i];
        assert_eq!(token.kind, expected_kind, "Kind mismatch at token {}", i);
        assert_eq!(token.len, expected_len, "Length mismatch at token {}", i);
        assert_eq!(
            position - token.len,
            expected_pos,
            "Position mismatch at token {}",
            i
        );
        i += 1;
    }
    println!(
        "Expected length: {}\nLength of tokenized: {position}",
        source.len()
    );
}
