pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

// Useful for Rust's pattern-matching
pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTStatement {
    kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }
}

pub enum ASTExpressionKind {
    Number(i64),
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}
