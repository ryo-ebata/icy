mod ast;
mod parser;

pub use ast::{ASTNode, AST};
pub use parser::parse_hcl;
