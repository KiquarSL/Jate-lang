//! Declarations of AST nodes

mod expr;
mod stmt;

pub type Ident = String;
pub type Block = Vec<stmt::Stmt>;

#[derive(Debug, Clone)]
pub struct Argument {
    ident: String,
    ty: jate_ast_ir::Type,
}

pub use expr::{BinOp, Expr, ExprKind, UnOp};
pub use stmt::{
    AssignOp, ClassField, ClassIdent, ClassMethod, ClassType, FunctionBase, IdentNode, Stmt,
    StmtKind, TraitMethod,
};
