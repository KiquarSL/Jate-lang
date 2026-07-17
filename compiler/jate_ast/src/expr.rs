//! Declatation of expression node of AST

use crate::Ident;

use jate_error::Span;
use std::fmt;

/// Create expression and make `boxed kind`
/// Arguments: ExprKind, Span
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
    pub kind: Box<ExprKind>,
    pub span: Span,
}

/// Kind od expression
/// Using as source values
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Null,
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
    /// Unwrap operator, e.g., `nullableObj!?`
    Unwrap(Expr),
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
impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Not => '!',
                Self::Neg => '-',
            }
        )
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Sub => "-",
                Self::Div => "/",
                Self::Mul => "*",
                Self::And => "&&",
                Self::Or => "||",
                Self::Eq => "==",
                Self::Ne => "!=",
                Self::Gt => ">",
                Self::Ge => ">=",
                Self::Lt => "<",
                Self::Le => "<=",
            }
        )
    }
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{}", n),
            Self::Float(n) => write!(f, "{}", n),
            Self::Char(c) => write!(f, "'{}'", c),
            Self::Bool(truth) => write!(f, "{}", truth),
            Self::Bin(l, op, r) => write!(f, "({} {} {})", l, op, r),
            Self::Unary(op, expr) => write!(f, "{}{}", op, expr),
            Self::Unwrap(expr) => write!(f, "{}!?", expr),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Call(expr, args) => {
                write!(f, "{}(", expr)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Self::Ident(path) => {
                let mut iter = path.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{}", first)?;
                    for part in iter {
                        write!(f, "::{}", part)?;
                    }
                }
                Ok(())
            }
            Self::Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.kind, self.span)
    }
}
