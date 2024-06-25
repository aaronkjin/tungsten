pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

// Useful for Rust's pattern-matching
pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTStatement {}
