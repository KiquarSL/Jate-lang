use crate::{TokenItem, TokenStream};
use jate_ast::{BinOp, Expr, ExprKind, Stmt, StmtKind, UnOp, expr, stmt};
use jate_error::{Diagnostic, diag, span};
use jate_lexer::{LiteralKind, StrPrefix, Token, TokenKind};
use jate_session::SourceFile;

/// Result wrapper for get statement when successful parse and return vector of diagnostics when errors
pub type StmtItem = Option<Result<Stmt, Vec<Diagnostic>>>;
pub type ExprItem = Option<Result<Expr, Diagnostic>>;

pub fn parse(stream: TokenStream, source: &SourceFile) -> impl Iterator<Item = StmtItem> {
    let mut cursor = TokenCursor { stream, source };
    return std::iter::from_fn(move || cursor.advance_stmt());
}

pub(crate) struct TokenCursor<'a> {
    pub stream: TokenStream,
    pub source: &'a SourceFile,
}

impl<'a> TokenCursor<'a> {
    pub fn advance_stmt(&mut self) -> Option<StmtItem> {
        todo!()
    }

    /// Parse one expression
    /// Can be used for parse expressions without AST
    pub fn advance_expr(&mut self) -> ExprItem {
        self.expr()
    }

    fn expr(&mut self) -> ExprItem {
        let left = self.logical();
        return left;
    }

    /// Handle logical operators
    /// `&&` - And
    /// `||` - Or
    fn logical(&mut self) -> ExprItem {
        let mut left = match self.unary()? {
            Ok(left) => left,
            Err(err) => return Some(Err(err)),
        };
        loop {
            let (token_op, op) = match self.advance()? {
                Ok(token) => (
                    token,
                    match token.kind {
                        TokenKind::And => BinOp::And,
                        TokenKind::Or => BinOp::Or,
                        _ => break,
                    },
                ),
                Err(err) => return Some(Err(err)),
            };
            let start = self.stream.current_pos(token_op.len);
            let right = match self.unary()? {
                Ok(right) => right,
                Err(err) => return Some(Err(err)),
            };

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
        let mut left = match self.unary()? {
            Ok(left) => left,
            Err(err) => return Some(Err(err)),
        };
        loop {
            let (token_op, op) = match self.advance()? {
                Ok(token) => (
                    token,
                    match token.kind {
                        TokenKind::Gt => BinOp::Gt,
                        TokenKind::Ge => BinOp::Ge,
                        TokenKind::Lt => BinOp::Lt,
                        TokenKind::Le => BinOp::Le,
                        TokenKind::Eq => BinOp::Eq,
                        TokenKind::Ne => BinOp::Ne,
                        _ => break,
                    },
                ),
                Err(err) => return Some(Err(err)),
            };
            let start = self.stream.current_pos(token_op.len);
            let right = match self.unary()? {
                Ok(right) => right,
                Err(err) => return Some(Err(err)),
            };

            left = expr!(ExprKind::Bin(left, op, right), span!(start, token_op.len));
        }
        return Some(Ok(left));
    }

    /// Handle `+` - add and `-` - subtract operators
    fn additive(&mut self) -> ExprItem {
        let mut left = match self.unary()? {
            Ok(left) => left,
            Err(err) => return Some(Err(err)),
        };
        loop {
            let (token_op, op) = match self.advance()? {
                Ok(token) => (
                    token,
                    match token.kind {
                        TokenKind::Plus => BinOp::Add,
                        TokenKind::Minus => BinOp::Sub,
                        _ => break,
                    },
                ),
                Err(err) => return Some(Err(err)),
            };
            let start = self.stream.current_pos(token_op.len);
            let right = match self.unary()? {
                Ok(right) => right,
                Err(err) => return Some(Err(err)),
            };

            left = expr!(ExprKind::Bin(left, op, right), span!(start, token_op.len));
        }
        return Some(Ok(left));
    }

    /// Handle `*` - multiply and `/` - divide operators
    fn multiplicative(&mut self) -> ExprItem {
        let mut left = match self.unary()? {
            Ok(left) => left,
            Err(err) => return Some(Err(err)),
        };
        loop {
            let (token_op, op) = match self.advance()? {
                Ok(token) => (
                    token,
                    match token.kind {
                        TokenKind::Star => BinOp::Mul,
                        TokenKind::Slash => BinOp::Div,
                        _ => break,
                    },
                ),
                Err(err) => return Some(Err(err)),
            };
            let start = self.stream.current_pos(token_op.len);
            let right = match self.unary()? {
                Ok(right) => right,
                Err(err) => return Some(Err(err)),
            };

            left = expr!(ExprKind::Bin(left, op, right), span!(start, token_op.len));
        }
        return Some(Ok(left));
    }

    /// Handle unary operators
    /// Supported operators: `!` - not, `-` - negative
    fn unary(&mut self) -> ExprItem {
        match self.first()? {
            Ok(token) => match token.kind {
                TokenKind::Bang => {
                    let start = self.stream.pos;
                    let _bang = self.advance()?;
                    let expr = match self.primary()? {
                        Ok(expr) => expr,
                        Err(err) => return Some(Err(err)),
                    };

                    Some(Ok(expr!(
                        ExprKind::Unary(UnOp::Not, expr),
                        span!(start, self.stream.pos - start)
                    )))
                }
                TokenKind::Minus => {
                    let start = self.stream.pos;
                    let _minus = self.advance()?;
                    let expr = match self.primary()? {
                        Ok(expr) => expr,
                        Err(err) => return Some(Err(err)),
                    };

                    Some(Ok(expr!(
                        ExprKind::Unary(UnOp::Neg, expr),
                        span!(start, self.stream.pos - start)
                    )))
                }
                _ => self.primary(),
            },
            Err(err) => Some(Err(err)),
        }
    }

    /// Parse source value
    pub(crate) fn primary(&mut self) -> ExprItem {
        let token_result = self.first()?;
        match token_result {
            Ok(token) => {
                let pos = self.stream.current_pos(token.len);
                let s = self.source.get_word(pos, token.len);

                match token.kind {
                    TokenKind::Literal(LiteralKind::Int) => {
                        let _ = self.advance()?;
                        Some(word_to_int(&s, token, pos))
                    }
                    TokenKind::Literal(LiteralKind::Float) => {
                        let _ = self.advance()?;
                        Some(word_to_float(&s, token, pos))
                    }
                    TokenKind::Literal(LiteralKind::Char) => {
                        let _ = self.advance()?;
                        Some(word_to_char(&s, token, pos))
                    }
                    TokenKind::Literal(LiteralKind::String(prefix)) => {
                        let _ = self.advance()?;
                        Some(word_to_string(&s, token, pos, prefix))
                    }
                    TokenKind::Ident => {
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
                    TokenKind::Whitespace => {
                        let _ = self.advance()?;
                        self.primary()
                    }
                    _ => Some(Err(diag!(
                        "E0006",
                        span!(self.stream.pos, 1),
                        vec![],
                        "Expacted value, getted: {:?}",
                        token
                    ))),
                }
            }
            Err(err) => Some(Err(err)),
        }
    }

    /// Parse ident as vector of segments, e.g., `path::to::some`
    // TODO: add handle for calls
    pub(crate) fn parse_ident(&mut self, start: u32) -> Result<Expr, Diagnostic> {
        let mut pos = start;
        let mut path = vec![];
        while let Some(token_result) = self.first() {
            match token_result {
                Ok(token) => {
                    if token.kind == TokenKind::Path {
                        self.advance();
                        pos += 2;
                    } else if token.kind == TokenKind::Ident {
                        let ident = self.source.get_word(pos, token.len).to_string();
                        path.push(ident);
                        self.advance();
                        pos += token.len;
                    } else {
                        break;
                    }
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr!(ExprKind::Ident(path), span!(start, pos - start)))
    }

    /// Recusrion variant of advnace for skip whitespaces
    fn advance(&mut self) -> TokenItem {
        let token = match self.stream.advance()? {
            Ok(token) => token,
            Err(err) => return Some(Err(err)),
        };
        // Skip whitespaces for AST
        if token.kind == TokenKind::Whitespace {
            return self.advance();
        }
        return Some(Ok(token));
    }

    /// Recusrion variant of first for skip whitespaces
    fn first(&mut self) -> TokenItem {
        let token = match self.stream.first()? {
            Ok(token) => token,
            Err(err) => return Some(Err(err)),
        };
        // Skip whitespaces for AST
        if token.kind == TokenKind::Whitespace {
            self.stream.advance();
            return self.first();
        }
        return Some(Ok(token));
    }
}

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

fn escape_symbol(s: &str, pos: u32, len: u32) -> Result<char, Diagnostic> {
    Ok(match s {
        "t" => '\t',
        "r" => '\r',
        "0" => '\0',
        "\\" => '\\',
        "\'" => '\'',
        "\"" => '\"',
        // TODO: add handle for ANSII escapes
        _ => return Err(diag!("E0010", span!(pos, len), "Unknown escape sequence")),
    })
}

fn word_to_char(s: &str, token: Token, pos: u32) -> Result<Expr, Diagnostic> {
    // cute `'` symbols
    let c = &s[1..token.len as usize - 1];
    if token.len >= 4 {
        // >= 4 because `'\t'`, ANSII have greater of 1 symbols
        // Handle escape sequence

        let escape = &c[1..c.len()];
        let escape = escape_symbol(escape, pos, token.len)?;
        return Ok(expr!(ExprKind::Char(escape), span!(pos, token.len)));
    }
    match c.parse::<char>() {
        Ok(c) => Ok(expr!(ExprKind::Char(c), span!(pos, token.len))),
        Err(err) => Err(diag!(
            "E0009",
            span!(pos, token.len),
            "Failed to get char from source: {err}"
        )),
    }
}

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
    let end = start + token.len as usize - 2;
    let s = &s[start..end];
    Ok(expr!(ExprKind::String(s.into()), span!(pos, token.len)))
}
