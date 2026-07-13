//! Declatation of expression node of AST

use crate::Ident;

use jate_error::Span;

/// Create expression and make `boxed kind`
#[macro_export]
macro_rules! expr {
    ($kind:expr, $span:expr) => {
        Expr {
            kind: Box::new($kind),
            span: $span,
        }
    };
}

/// Expr is node of AST
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    kind: Box<ExprKind>,
    span: Span,
}

/// Kind od expression
/// Using as source values
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
    Bin(Expr, BinOp, Expr),
    Unary(UnOp, Expr),
    /// chain of idents, e.g., `math::sqrt(x)`
    Ident(Vec<Ident>),
    Call(Expr, Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,

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
    /// `!=`
    Ne,

    /// `&&`
    And,
    /// `||`
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    /// `!`
    Not,
    /// `-`
    Neg,
}
