//! Declarations of AST nodes

pub mod expr;
pub mod stmt;

pub type Ident = String;
pub type Argument = (String, jate_ast_ir::Type);
pub type Block = Vec<stmt::Stmt>;
