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

    pub fn visit(&mut self, statement: ASTStatement) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }
}

pub trait ASTVisitor {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
        }
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number(number);
            }
        }
    }

    fn visit_number(&mut self, number: &ASTNumberExpression);
}

pub struct ASTPrinter {
    indent: usize,
}

impl ASTVisitor for ASTPrinter {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.print_with_indent(self.indent, &format!("Number: {}", number.number));
    }
}

impl ASTPrinter {
    fn print_with_indent(&mut self, indent: usize, text: &str) {
        println!("{}{}", " ".repeat(indent), text);
    }
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
