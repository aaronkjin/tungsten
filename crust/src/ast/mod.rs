pub mod lexer;
pub mod parser;
pub mod evaluator;

use crate::ast::lexer::{ TextSpan, Token };

use termion::color;
use termion::color::{ Fg, Reset };

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
        let mut printer = ASTPrinter::new();
        self.visit(&mut printer);
        println!("{}", printer.result);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
            ASTStatementKind::LetStatement(expr) => {
                self.visit_let_statement(expr);
            }
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.visit_expression(&let_statement.initializer);
    }

    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number_expression(number);
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
            ASTExpressionKind::Variable(expr) => {
                self.visit_variable_expression(expr);
            }
        }
    }

    // FIXME: Actually visit the error
    fn visit_error(&mut self, _span: &TextSpan) {}

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression);
    }

    // FIXME: Actually visit the number
    fn visit_number_expression(&mut self, number: &ASTNumberExpression);

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

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression);
}

pub struct ASTPrinter {
    indent: usize,
    result: String,
}

impl ASTPrinter {
    const NUMBER_COLOR: color::Cyan = color::Cyan;
    const TEXT_COLOR: color::LightWhite = color::LightWhite;
    const KEYWORD_COLOR: color::Magenta = color::Magenta;
    const VARIABLE_COLOR: color::Green = color::Green;

    pub fn new() -> Self {
        Self { indent: 0, result: String::new() }
    }

    fn add_whitespace(&mut self) {
        self.result.push_str(" ");
    }

    fn add_newline(&mut self) {
        self.result.push_str("
");
    }
}

impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        Self::do_visit_statement(self, statement);
        self.result.push_str(&format!("{}", Fg(Reset)));
        self.add_newline();
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.result.push_str(&format!("{}let", Self::KEYWORD_COLOR.fg_str()));

        self.add_whitespace();
        self.result.push_str(
            &format!("{}{}", Self::TEXT_COLOR.fg_str(), let_statement.identifier.span.literal)
        );

        self.add_whitespace();
        self.result.push_str(&format!("{}=", Self::TEXT_COLOR.fg_str()));

        self.add_whitespace();
        self.visit_expression(&let_statement.initializer);
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression) {
        self.result.push_str(
            &format!(
                "{}{}",
                Self::VARIABLE_COLOR.fg_str(),
                variable_expression.identifier.span.literal
            )
        )
    }

    // Change this method to visit_number_expression
    fn visit_number_expression(&mut self, number: &ASTNumberExpression) {
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

// AST statements
pub enum ASTStatementKind {
    // Useful for Rust's pattern-matching
    Expression(ASTExpression),
    LetStatement(ASTLetStatement),
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

    pub fn let_statement(identifier: Token, initializer: ASTExpression) -> Self {
        ASTStatement::new(
            ASTStatementKind::LetStatement(ASTLetStatement { identifier, initializer })
        )
    }
}

// AST let statements
pub struct ASTLetStatement {
    pub identifier: Token,
    pub initializer: ASTExpression,
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

// AST binary operators
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
    Variable(ASTVariableExpression),
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

    pub fn identifier(identifier: Token) -> Self {
        ASTExpression::new(ASTExpressionKind::Variable(ASTVariableExpression { identifier }))
    }

    pub fn error(span: TextSpan) -> Self {
        ASTExpression::new(ASTExpressionKind::Error(span))
    }
}
pub struct ASTVariableExpression {
    pub identifier: Token,
}

impl ASTVariableExpression {
    pub fn identifier(&self) -> &str {
        &self.identifier.span.literal
    }
}
