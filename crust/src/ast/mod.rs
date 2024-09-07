pub mod lexer;
pub mod parser;
pub mod evaluator;

use crate::ast::lexer::{ TextSpan, Token };

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

    pub fn visit(&self, visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) -> () {
        let mut printer = ASTPrinter { indent: 0 };
        self.visit(&mut printer);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number(number);
            }
            ASTExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            ASTExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
            ASTExpressionKind::Error(span) => {
                self.visit_error(span);
            }
        }
    }

    // FIXME: Actually visit the error
    fn visit_error(&mut self, span: &TextSpan) {}

    // FIXME: Actually visit the number
    fn visit_number(&mut self, number: &ASTNumberExpression);

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression
    ) {
        self.visit_expression(&parenthesized_expression.expression);
    }
}

pub struct ASTPrinter {
    indent: usize,
    result: String,
}

impl ASTPrinter {
    const NUMBER_COLOR: color::Cyan = color::Cyan;
    const TEXT_COLOR: color::White = color::White;

    fn add_whitespace(&mut self) {
        self.result.push_str(" ");
    }

    fn add_newline(&mut self) {
        self.result.push_str("
");
    }
}

impl ASTVisitor for ASTPrinter {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.result.push_str(&format!("{}{}", Self::NUMBER_COLOR.fg_str(), number.number));
    }

    fn visit_error(&mut self, span: &TextSpan) {
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), span.literal));
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        // Left
        self.visit_expression(&binary_expression.left);
        self.add_whitespace();

        // Operator
        self.result.push_str(
            &format!(
                "{}{}",
                Self::TEXT_COLOR.fg_str(),
                binary_expression.operator.token.span.literal
            )
        );

        // Right
        self.add_whitespace();
        self.visit_expression(&binary_expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression
    ) {
        // Left
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), "("));

        self.visit_expression(&parenthesized_expression.expression);

        // Right
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), ")"));
    }
}

impl ASTPrinter {
    fn print_with_indent(&mut self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text);
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

// AST parenthesized expressions
pub struct ASTParenthesizedExpression {
    expression: Box<ASTExpression>,
}

// AST binary expressions
pub struct ASTBinaryExpression {
    left: Box<ASTExpression>,
    operator: ASTBinaryOperator,
    right: Box<ASTExpression>,
}

#[derive(Debug)]
pub enum ASTBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub struct ASTBinaryOperator {
    kind: ASTBinaryOperatorKind,
    token: Token,
}

impl ASTBinaryOperator {
    pub fn new(kind: ASTBinaryOperatorKind, token: Token) -> Self {
        ASTBinaryOperator { kind, token }
    }

    // Precedence of +/- 0, 1, 2...
    pub fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Plus => 1,
            ASTBinaryOperatorKind::Minus => 1,
            ASTBinaryOperatorKind::Multiply => 2,
            ASTBinaryOperatorKind::Divide => 2,
        }
    }
}

// AST expressions
pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
    Error(TextSpan),
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number }))
    }

    pub fn binary(operator: ASTBinaryOperator, left: ASTExpression, right: ASTExpression) -> Self {
        ASTExpression::new(
            ASTExpressionKind::Binary(ASTBinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })
        )
    }

    pub fn parenthesized(expression: ASTExpression) -> Self {
        ASTExpression::new(
            ASTExpressionKind::Parenthesized(ASTParenthesizedExpression {
                expression: Box::new(expression),
            })
        )
    }

    pub fn error(span: TextSpan) -> Self {
        ASTExpression::new(ASTExpressionKind::Error(span))
    }
}
