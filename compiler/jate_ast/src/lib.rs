//! Declarations of AST nodes

mod expr;
mod stmt;

pub type Ident = String;
pub type Argument = (String, jate_ast_ir::Type);
pub type Block = Vec<stmt::Stmt>;

pub use expr::{BinOp, Expr, UnOp};
pub use stmt::{
    AssignOp, ClassField, ClassIdent, ClassMethod, ClassType, FunctionBase, IdentNode, Stmt,
    TraitMethod,
};
