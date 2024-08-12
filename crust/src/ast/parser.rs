use crate::ast::lexer::{ Lexer, Token, TokenKind };
use crate::ast::{ ASTStatement, ASTExpression, ASTBinaryOperator, ASTBinaryOperatorKind };

pub struct Parser {
    tokens: Vec<Token>,
    current: usize, // Pointer to cur token
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
            current: 0,
            diagnostics_bag,
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        if self.is_at_end() {
            return None;
        }

        return self.parse_statement();
    }

    fn is_at_end(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    fn parse_statement(&mut self) -> ASTStatement {
        let _token = self.current();

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
        let token = self.current()?;

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
                let expr = self.parse_expression()?;
                let token = self.consume()?;

                if token.kind != TokenKind::RightParen {
                    panic!("Expected right parenthesis!");
                }
                ASTExpression::parenthesized(expr);
            }
            _ => {
                // FIXME: handle error cases
            }
        }
    }

    fn parse_expression(&mut self) -> ASTExpression {
        return self.parse_binary_expression(0);
    }

    fn peek(&self, offset: isize) -> &Token {
        let mut index = ((self.current as isize) + offset) as usize;

        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }
        self.tokens.get(index).unwrap()
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }

    fn consume(&mut self) -> &Token {
        self.current += 1;
        self.peek(-1)
    }
}
