use crate::ast::lexer::{ Token, Lexer, TokenKind };
use crate::ast::{ ASTStatement, ASTExpression };

pub struct Parser {
    tokens: Vec<Token>,
    current: usize, // Pointer to cur token
}

impl Parser {
    pub fn new() -> Self {
        Self { tokens: Vec::new(), current: 0 }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn from_input(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        Self { tokens, current: 0 }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Option<ASTStatement> {
        let _token = self.current()?;

        let expr = self.parse_expression()?;
        Some(ASTStatement::expression(expr))
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<ASTExpression> {
        let left = self.parse_primary_expression()?;

        while let Some(operator_precedence) = self.current_precedence() {
            let operator = self.consume()?;
        }
    }

    // For function calls, literals, strings, etc.
    fn parse_primary_expression(&mut self) -> Option<ASTExpression> {
        let token = self.consume()?;

        // Edge case: Reached the end of file
        if token.kind == TokenKind::Eof {
            return None;
        }

        match token.kind {
            TokenKind::Number(number) => { Some(ASTExpression::number(number)) }
            _ => { None }
        }
    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {}

    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get(((self.current as isize) + offset) as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1)?;
        Some(token)
    }
}
