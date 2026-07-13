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
/// Using for class names and traits
/// In future add generics, etc.
#[derive(Debug, Clone)]
pub struct ClassIdent {
    ident: Ident,
}

/// JVM class type
/// `No`       - Final class
/// `Open`     - Extendable class
/// `Abstract` - Abstract class
#[derive(Debug, Clone, Copy, Default)]
pub enum ClassType {
    #[default]
    No,
    Open,
    Abstract,
}

/// JVM class field
/// `publicity` - is public class (Visibility in other packages)
/// `staticity` - is static field
/// `ident`     - identifier of field
/// `ty`        - type of field
/// `value`.    - value of field
#[derive(Debug, Clone)]
pub struct ClassField {
    publicity: bool,
    staticity: bool,
    ident: Ident,
    ty: Type,
    value: Option<Expr>,
}

/// Function base keep general fields for functions and methods
/// Using in ClassMethod and Stmt::Function
#[derive(Debug, Clone)]
pub struct FunctionBase {
    publicity: bool,
    ident: Ident,
    ret_ty: Option<Type>,
}

/// Class method is wrapper of `FunctionBase` with fields `staticity` and `body`
#[derive(Debug, Clone)]
pub struct ClassMethod {
    staticity: bool,
    base: FunctionBase,
    body: Block,
}

/// Trait method is wrapper of `FunctionBase` with `staticity` and optional `body` (default implementation)
#[derive(Debug, Clone)]
pub struct TraitMethod {
    staticity: bool,
    base: FunctionBase,
    body: Option<Block>,
}

/// Stmt is node of AST
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
