use crate::{TokenItem, TokenStream};
use jate_ast::{BinOp, Expr, ExprKind, Stmt, UnOp, expr};
use jate_error::{Diagnostic, diag, span};
use jate_lexer::{LiteralKind, StrPrefix, Token, TokenKind};
use jate_session::SourceFile;

/// Automatic return from function if item is None (end of token stream) or Diagnostic
/// Return `Expr` if item valid for parsing
/// Using for functions under expression parsing
macro_rules! get_expr_from_item {
    ($expr_item:expr) => {
        match $expr_item? {
            Ok(expr) => expr,
            Err(err) => return Some(Err(err)),
        }
    };
}

/// Automatic return from function if item is None (end of token stream) or Diagnostic
/// Return `Token` if item valid for parsing
/// Using for `advance` and `first` functions
macro_rules! get_token_from_item {
    ($expr_item:expr) => {
        match $expr_item? {
            Ok(expr) => expr,
            Err(err) => return Some(Err(err)),
        }
    };
}

/// Result wrapper for get statement/expression when successful parse and return vector of diagnostics when errors
pub type StmtItem = Option<Result<Stmt, Vec<Diagnostic>>>;
pub type ExprItem = Option<Result<Expr, Diagnostic>>;

pub fn parse(
    stream: Box<dyn Iterator<Item = Token>>,
    source: &SourceFile,
) -> impl Iterator<Item = StmtItem> {
    let stream = TokenStream::new(stream);
    let mut cursor = AstCursor { stream, source };
    return std::iter::from_fn(move || cursor.advance_stmt());
}

pub(crate) struct AstCursor<'a> {
    pub stream: TokenStream,
    pub source: &'a SourceFile,
}

impl<'a> AstCursor<'a> {
    pub fn advance_stmt(&mut self) -> Option<StmtItem> {
        todo!()
    }

    /// Parse one expression
    /// Can be used for parse expressions without AST
    pub fn advance_expr(&mut self) -> ExprItem {
        self.expr()
    }

    /// Parse ident as vector of segments, e.g., `path::to::some`
    // TODO: add handle for calls
    pub(crate) fn parse_ident(&mut self, start: u32) -> Result<Expr, Diagnostic> {
        let mut path = vec![];
        while let Some(token_res) = self.first() {
            let token = token_res?;
            self.skip_whitespaces();
            if token.kind == TokenKind::Ident {
                let ident = self.source.get_word(self.stream.pos, token.len);
                path.push(ident);
                self.advance();
                self.skip_whitespaces();

                match self.first() {
                    Some(token_res) => {
                        if token_res?.kind == TokenKind::Path {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            } else {
                break;
            }
        }
        Ok(expr!(
            ExprKind::Ident(path),
            span!(start, self.stream.pos - start - 1)
        ))
    }

    fn skip_whitespaces(&mut self) -> Option<Diagnostic> {
        while let Some(token_res) = self.first() {
            match token_res {
                Ok(token) => match token.kind {
                    TokenKind::Whitespace | TokenKind::LineComment | TokenKind::BlockComment => {
                        self.advance();
                    }
                    _ => break,
                },
                Err(err) => return Some(err),
            }
        }
        return None;
    }

    fn advance(&mut self) -> TokenItem {
        return self.stream.advance();
    }

    /// Recusrion variant of first for skip whitespaces
    pub(crate) fn first(&mut self) -> TokenItem {
        return self.stream.first();
    }

    fn advance_get_word(&mut self, token: Token) -> (u32, String) {
        let pos = self.stream.pos;
        let _ = self.advance();
        let s = self.source.get_word(pos, token.len);
        (pos, s)
    }
}

/// Block methods for statements
impl<'a> AstCursor<'a> {
    fn stmt(&mut self) -> StmtItem {
        todo!()
    }
}

/// Block methods for expressions
impl<'a> AstCursor<'a> {
    fn expr(&mut self) -> ExprItem {
        return self.logical();
    }

    /// Handle logical operators
    /// `&&` - And
    /// `||` - Or
    fn logical(&mut self) -> ExprItem {
        let mut left = get_expr_from_item!(self.comparison());
        loop {
            let token_op = get_token_from_item!(self.first());
            let op = match token_op.kind {
                TokenKind::And => BinOp::And,
                TokenKind::Or => BinOp::Or,
                _ => break,
            };

            let _ = self.advance();
            let start = self.stream.pos;
            let right = get_expr_from_item!(self.comparison());

            left = expr!(ExprKind::Bin(left, op, right), span!(start, token_op.len));
        }
        return Some(Ok(left));
    }

    /// Handle comparison operators
    /// `>` - Greater
    /// `>=` - Greater equal
    /// `<` - Less
    /// `<=` - Less equal
    /// `==` - Equals
    /// `!=` - Not equals
    fn comparison(&mut self) -> ExprItem {
        let mut left = get_expr_from_item!(self.additive());
        loop {
            let token_op = get_token_from_item!(self.first());
            let op = match token_op.kind {
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Ge => BinOp::Ge,
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Eq => BinOp::Eq,
                TokenKind::Ne => BinOp::Ne,
                _ => break,
            };

            let _ = self.advance();
            let start = self.stream.pos;
            let right = get_expr_from_item!(self.additive());

            left = expr!(ExprKind::Bin(left, op, right), span!(start, token_op.len));
        }
        return Some(Ok(left));
    }

    /// Handle `+` - add and `-` - subtract operators
    fn additive(&mut self) -> ExprItem {
        let mut left = get_expr_from_item!(self.multiplicative());
        loop {
            let token_op = get_token_from_item!(self.first());
            let op = match token_op.kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };

            let _ = self.advance();
            let start = self.stream.pos;
            let right = get_expr_from_item!(self.multiplicative());

            left = expr!(ExprKind::Bin(left, op, right), span!(start, token_op.len));
        }
        return Some(Ok(left));
    }

    /// Handle `*` - multiply and `/` - divide operators
    fn multiplicative(&mut self) -> ExprItem {
        let mut left = get_expr_from_item!(self.unary());
        loop {
            let token_op = get_token_from_item!(self.first());
            let op = match token_op.kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => break,
            };

            let _ = self.advance();
            let start = self.stream.pos;
            let right = get_expr_from_item!(self.unary());

            left = expr!(ExprKind::Bin(left, op, right), span!(start, token_op.len));
        }
        return Some(Ok(left));
    }

    /// Handle unary operators
    /// Supported operators: `!` - not, `-` - negative
    fn unary(&mut self) -> ExprItem {
        let token = get_token_from_item!(self.first());
        let start = self.stream.pos;
        match token.kind {
            TokenKind::Bang => {
                let _bang = self.advance()?;
                let expr = get_expr_from_item!(self.postfix());

                Some(Ok(expr!(
                    ExprKind::Unary(UnOp::Not, expr),
                    span!(start, self.stream.pos - start)
                )))
            }
            TokenKind::Minus => {
                let _minus = self.advance()?;
                let expr = get_expr_from_item!(self.postfix());

                Some(Ok(expr!(
                    ExprKind::Unary(UnOp::Neg, expr),
                    span!(start, self.stream.pos - start)
                )))
            }
            _ => self.postfix(),
        }
    }

    /// Handle posifix operators
    /// Supported operators: `!?` - unwrap
    pub(crate) fn postfix(&mut self) -> ExprItem {
        let expr_item = self.primary();
        let start = self.stream.pos;
        match get_token_from_item!(self.first()).kind {
            TokenKind::Unwrap => {
                let _unwrap = self.advance()?;
                Some(Ok(expr!(
                    ExprKind::Unwrap(get_expr_from_item!(expr_item)),
                    span!(start, self.stream.pos - start)
                )))
            }
            _ => expr_item,
        }
    }

    /// Parse source value
    pub(crate) fn primary(&mut self) -> ExprItem {
        self.skip_whitespaces();
        let token = get_expr_from_item!(self.first());

        match token.kind {
            TokenKind::Literal(LiteralKind::Int) => {
                let (pos, s) = self.advance_get_word(token);
                Some(word_to_int(&s, token, pos))
            }
            TokenKind::Literal(LiteralKind::Float) => {
                let (pos, s) = self.advance_get_word(token);
                Some(word_to_float(&s, token, pos))
            }
            TokenKind::Literal(LiteralKind::Char) => {
                let (pos, s) = self.advance_get_word(token);
                Some(word_to_char(&s, token, pos))
            }
            TokenKind::Literal(LiteralKind::String(prefix)) => {
                let (pos, s) = self.advance_get_word(token);
                Some(word_to_string(&s, token, pos, prefix))
            }
            TokenKind::Ident => {
                let pos = self.stream.pos;
                let s = self.source.get_word(pos, token.len);
                let span = span!(pos, token.len);
                Some(match s.as_str() {
                    "true" => {
                        let _ = self.advance()?;
                        Ok(expr!(ExprKind::Bool(true), span))
                    }
                    "false" => {
                        let _ = self.advance()?;
                        Ok(expr!(ExprKind::Bool(false), span))
                    }
                    "null" => {
                        let _ = self.advance()?;
                        Ok(expr!(ExprKind::Null, span))
                    }
                    _ => self.parse_ident(pos),
                })
            }
            TokenKind::LParen => {
                let _ = self.advance()?;
                let pos = self.stream.pos;
                let expr = self.expr();
                let token = get_expr_from_item!(self.advance());
                match token.kind {
                    TokenKind::RParen => {}
                    _ => {
                        return Some(Err(diag!(
                            "E0011",
                            span!(pos, token.len),
                            vec![],
                            "Expected ')', found: {:?}",
                            token
                        )));
                    }
                };

                return expr;
            }
            TokenKind::Whitespace => {
                let _ = self.advance()?;
                self.primary()
            }
            _ => Some(Err(diag!(
                "E0006",
                span!(self.stream.pos, 1),
                vec![],
                "Expacted value, found: {:?}",
                token
            ))),
        }
    }
}

// Help functions for translate text to required type

/// Transform integer loteral from text
/// Return expression if successful
/// Return diagnostic if failed
fn word_to_int(s: &str, token: Token, pos: u32) -> Result<Expr, Diagnostic> {
    match s.parse::<i64>() {
        Ok(num) => Ok(expr!(ExprKind::Int(num), span!(pos, token.len))),
        Err(err) => Err(diag!(
            "E0007",
            span!(pos, token.len),
            "Failed to get int from source: {err}"
        )),
    }
}
/// Transform float literal from text
/// Return expression if successful
/// Return diagnostic if failed
fn word_to_float(s: &str, token: Token, pos: u32) -> Result<Expr, Diagnostic> {
    match s.parse::<f64>() {
        Ok(num) => Ok(expr!(ExprKind::Float(num), span!(pos, token.len))),
        Err(err) => Err(diag!(
            "E0008",
            span!(pos, token.len),
            "Failed to get float from source: {err}"
        )),
    }
}

/// Transform escape symbol to non-escape char
fn escape_symbol(s: &str, pos: u32, len: u32) -> Result<char, Diagnostic> {
    Ok(match s {
        "\\t" => '\t',
        "\\r" => '\r',
        "\\0" => '\0',
        "\\\\" => '\\',
        "\\\'" => '\'',
        "\\\"" => '\"',
        // TODO: add handle for ANSII escapes
        _ => return Err(diag!("E0010", span!(pos, len), "Unknown escape sequence")),
    })
}

/// Transform char literal from text
/// Return expression if successful
/// Return diagnostic if failed
pub(crate) fn word_to_char(s: &str, token: Token, pos: u32) -> Result<Expr, Diagnostic> {
    let inner = &s[1..token.len as usize - 1];
    if inner.len() >= 2 {
        let escape = escape_symbol(inner, pos, token.len)?;
        return Ok(expr!(ExprKind::Char(escape), span!(pos, token.len)));
    }
    match inner.chars().next() {
        Some(c) => Ok(expr!(ExprKind::Char(c), span!(pos, token.len))),
        None => Err(diag!(
            "E0009",
            span!(pos, token.len),
            "Empty character literal"
        )),
    }
}

/// Transform string literal from text
/// Return expression if successful
/// Return diagnostic if failed
pub(crate) fn word_to_string(
    s: &str,
    token: Token,
    pos: u32,
    pref: StrPrefix,
) -> Result<Expr, Diagnostic> {
    let start = match pref {
        StrPrefix::Format | StrPrefix::Raw => 2,
        StrPrefix::No => 1,
    };
    let end = token.len as usize - 1;
    let s = &s[start..end];
    Ok(expr!(ExprKind::String(s.into()), span!(pos, token.len)))
}
