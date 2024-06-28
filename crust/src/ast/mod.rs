pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }
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

    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }
}

pub enum ASTExpressionKind {
    Number(i64),
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(number))
    }
}
