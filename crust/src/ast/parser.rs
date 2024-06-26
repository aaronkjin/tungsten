pub struct Parser {
    tokens: Vec<super::lexer::Token>,
    current: usize, // Pointer to cur token
}
