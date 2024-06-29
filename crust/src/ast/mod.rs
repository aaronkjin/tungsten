pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

// Diagram AST out as a... tree (I know, shocking)
impl Ast {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visualize(&self) {
        println!("digraph ast {{");
        println!("   node [shape=box]");
        for (i, statement) in self.statements.iter().enumerate() {
            println!("   {} [label=\"{}\"];", i, statement);
        }
        println!("}}");
    }
}

trait ASTVisitor {
    fn visit_statement(&mut self, statement: &ASTStatement);
    fn visit_expression(&mut self, expression: &ASTExpression);
    fn visit_number(&mut self, number: &ASTNumberExpression);
}

// AST statements
pub enum ASTStatementKind {
    // Useful for Rust's pattern-matching
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

// AST number expressions
pub struct ASTNumberExpression {
    number: i64,
}

// AST expressions
pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression))
    }
}
