pub struct Parser {
    tokens: Vec<super::lexer::Token>,
    current: usize, // Pointer to cur token
}

impl Parser {
    pub fn new() -> Self {
        Self { tokens: Vec::new(), current: 0 }
    }
}
