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

// Take expression in as input, transform into tokens as output
pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, current_pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // Make sure we don't go over
        if self.current_pos > self.input.len() {
            return None;
        }

        // End of token stream
        if self.current_pos == self.input.len() {
            let eof_char: char = '\0';
            self.current_pos += 1;

            return Some(Token::new(TokenKind::EOF, TextSpan::new(0, 0, eof_char.to_string())));
        }

        // Check if char is number token
        let start = self.current_pos;
        let c: char = self.current_char();
        if self.is_number_start(&c) {
            self.tokenize_number();
        }

        let end = self.current_pos;
        let literal = self.input[start..end].to_string();
    }

    // Helper method to see if char is a number
    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.current_pos).unwrap()
    }
}
