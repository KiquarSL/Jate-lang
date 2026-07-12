//! Declaration of statement node

use crate::expr::Expr;
use crate::{Argument, Block, Ident};
use jate_ast_ir::Type;

/// Need for grouping idents, e.g., `(i, j, z)`
#[derive(Debug, Clone)]
pub enum IdentNode {
    Ident(Ident),
    Group(Vec<IdentNode>),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ClassType {
    #[default]
    No,
    Extendable,
    Abstract,
}

#[derive(Debug, Clone)]
pub struct ClassField {
    publicity: bool,
    staticity: bool,
    ident: Ident,
    ty: Type,
    value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct FunctionBase {
    publicity: bool,
    ident: Ident,
    ret_ty: Option<Type>,
    body: Block,
}

#[derive(Debug, Clone)]
pub struct ClassMethod {
    staticity: bool,
    base: FunctionBase,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Declare {
        publicity: bool,
        ident: Ident,
        ty: Type,
        value: Expr,
    },
    Assign {
        ident: Ident,
        asaign: AssignOp,
        value: Expr,
    },
    Function(FunctionBase),
    IfElse {
        cond: Expr,
        then: Box<Stmt>,
        else_: Option<Box<Stmt>>,
    },
    WhileLoop {
        cond: Expr,
        body: Box<Stmt>,
    },
    ForLoop {
        iter: IdentNode,
        data: Expr,
        body: Box<Stmt>,
    },
    Class {
        publicity: bool,
        ty: ClassType,
        ident: Ident,
        /// Class and traits
        extends: Vec<Ident>,
        fields: Vec<ClassField>,
        methods: Vec<ClassMethod>,
    },
    Break,
    Continue,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AssignOp {
    /// `=`
    Default,
    /// `+=`
    Add,
    /// `-=`
    Sub,
    /// `*=`
    Mul,
    /// `/=`
    Div,
}
