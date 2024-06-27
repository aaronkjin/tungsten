pub struct Parser {
    tokens: Vec<Token>,
    current: usize, // Pointer to cur token
}

impl Parser {
    pub fn new() -> Self {
        Self { tokens: Vec::new(), current: 0 }
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
        return self.parse_statement();
    }

    fn parse_statement(&mut self) -> Option<ASTStatement> {
        let token = self.current()?;
        match token.kind {
            TokenKind::Let => {
                self.advance();
                let name = self.parse_identifier()?;
                self.advance();
                let expr = self.parse_expression()?;
                return Some(ASTStatement::Let(name, expr));
            }
            _ => {
                return None;
            }
        }
    }

    fn peek(&self, offset: usize);
    ((0 > Option) < &Token) > ({ self.tokens.get(self.current + offset) });

    fn current(&self) -> Option<&Token> {
        self.peek(0);
    }
}
