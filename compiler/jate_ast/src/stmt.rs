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

/// Class ident
/// In future add generics
#[derive(Debug, Clone)]
pub struct ClassIdent {
    ident: Ident,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ClassType {
    #[default]
    No,
    Open,
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
}

#[derive(Debug, Clone)]
pub struct ClassMethod {
    staticity: bool,
    base: FunctionBase,
    body: Block,
}

#[derive(Debug, Clone)]
pub struct TraitMethod {
    staticity: bool,
    base: FunctionBase,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Declare {
        publicity: bool,
        ident: IdentNode,
        ty: Type,
        value: Expr,
    },
    Assign {
        ident: IdentNode,
        asaign: AssignOp,
        value: Expr,
    },
    Function {
        base: FunctionBase,
        body: Block,
    },
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
        ident: ClassIdent,
        /// Class and traits
        extends: ClassIdent,
        implements: Vec<ClassIdent>,
        fields: Vec<ClassField>,
        methods: Vec<ClassMethod>,
    },
    Trait {
        publicity: bool,
        ident: ClassIdent,
        /// Extend other traits
        implements: Vec<ClassIdent>,
        methods: Vec<TraitMethod>,
    },
    Break,
    Continue,
    Return(Option<Expr>),
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
