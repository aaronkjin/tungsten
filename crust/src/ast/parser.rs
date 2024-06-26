pub struct Parser {
    tokens: Vec<super::lexer::Token>,
    current: usize, // Pointer to cur token
}

impl Parser {
    pub fn new() -> Self {
        Self { tokens: Vec::new(), current: 0 }
    }

    pub fn from_input(input: &str) -> Self {
        let mut lexer = super::lexer::Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        Self { tokens, current: 0 }
    }
}
