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

pub struct ASTVisitor {
    pub ast: &'a Ast,
}

impl<'a> ASTVisitor<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self { ast }
    }

    pub fn visit(&saelf) {
        for statement in self.ast.statements.iter() {
            match &statement.kind {
                ASTStatementKind::Expression(expr) => {
                    self.visit_expression(expr);
                }
            }
        }
    }

    fn visit_expression(&self, expr: &ASTExpression) {
        match &expr.kind {
            ASTExpressionKind::Number(number) => {
                println!("Number: {}", number);
            }
        }
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
