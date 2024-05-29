pub enum TokenKind {
    Number(i64),
    // PEMDAS operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
}

pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self { start, end, literal }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

// Take expression in as input, transform into tokens as ouput
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>
}

impl <'a> Lexer<'a> {
    
    pub fn new(input: &'a str) -> Self {
        Self { input: input.chars().peekable() }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.peek().map(|c : &char |)
    }

    // Helper method
    fn is_number_start(c: &char) -> bool {
        c.is_digit( radix: 10)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
}