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
    }

    // Helper method
    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn peek(&self) -> Option<&char> {
        self.input.chars().nth(self.current_pos)
    }
}
