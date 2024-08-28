use crate::ast::{ ASTBinaryOperator, ASTBinaryOperatorKind, ASTExpression, ASTStatement };
use crate::ast::lexer::{ Lexer, Token, TokenKind };
use crate::diagnostics::DiagnosticsBagCell;
use std::cell;

pub struct Counter {
    value: Cell<usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: Cell::new(0),
        }
    }

    pub fn increment(&self) {
        let current_value = self.value.get();
        self.value.set(current_value + 1);
    }

    pub fn get_value(&self) -> usize {
        self.value.get()
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: Counter, // Pointer to cur token
    diagnostics_bag: DiagnosticsBagCell, // Cool stuff: https://www.geeksforgeeks.org/error-handling-compiler-design/
}

impl Parser {
    pub fn new(tokens: Vec<Token>, diagnostics_bag: DiagnosticsBagCell) -> Self {
        Self {
            tokens: tokens
                .into_iter()
                .filter(|token| {
                    match token.kind {
                        TokenKind::Whitespace => false,
                        _ => true,
                    }
                })
                .collect(),
            current: Counter::new(),
            diagnostics_bag,
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        if self.is_at_end() {
            return None;
        }
        Some(self.parse_statement())
    }

    fn is_at_end(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    fn parse_statement(&mut self) -> ASTStatement {
        let expr = self.parse_expression();
        return ASTStatement::expression(expr);
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> ASTExpression {
        let mut left = self.parse_primary_expression();

        while let Some(operator) = self.parse_binary_operator() {
            self.consume();
            let operator_precedence = operator.precedence();

            // Base case
            if operator_precedence < precedence {
                break;
            }

            // Recursive case
            let right = self.parse_binary_expression(operator_precedence);
            left = ASTExpression::binary(operator, left, right);
        }

        return left;
    }

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        let token = self.current();

        let kind = match token.kind {
            TokenKind::Plus => { Some(ASTBinaryOperatorKind::Plus) }
            TokenKind::Minus => { Some(ASTBinaryOperatorKind::Minus) }
            TokenKind::Asterisk => { Some(ASTBinaryOperatorKind::Multiply) }
            TokenKind::Slash => { Some(ASTBinaryOperatorKind::Divide) }
            _ => { None }
        };

        return kind.map(|k| ASTBinaryOperator::new(k, token.clone()));
    }

    // For function calls, literals, strings, etc.
    fn parse_primary_expression(&mut self) -> ASTExpression {
        let token = self.consume();

        // Edge case: Reached the end of file
        if token.kind == TokenKind::Eof {
            return None;
        }

        match token.kind {
            TokenKind::Number(number) => { ASTExpression::number(number) }
            TokenKind::LeftParen => {
                let expr = self.parse_expression();
                let token = self.consume_and_check(TokenKind::RightParen);

                ASTExpression::parenthesized(expr);
            }
            _ => {
                // FIXME: handle error cases
                self.diagnostics_bag.borrow_mut().report_expected_expression(token);
                ASTExpression::error(token.span.clone())
            }
        }
    }

    fn parse_expression(&mut self) -> ASTExpression {
        return self.parse_binary_expression(0);
    }

    fn peek(&self, offset: isize) -> &Token {
        let mut index = ((self.current.get_value() as isize) + offset) as usize;

        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }
        self.tokens.get(index).unwrap()
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }

    fn consume(&self) -> &Token {
        self.current.increment();
        self.peek(-1)
    }

    fn consume_and_check(&self, kind: TokenKind) -> &Token {
        let token = self.consume();
        if token.kind != kind {
            self.diagnostics_bag.borrow_mut().report_unexpected_token(&kind, token);
        }
        token
    }
}

// 45:27
