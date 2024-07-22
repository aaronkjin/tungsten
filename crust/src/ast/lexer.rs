#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Number(i64),
    // PEMDAS operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Bad,
    Whitespace,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
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
        Self {
            input,
            current_pos: 0,
        }
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

            return Some(Token::new(TokenKind::Eof, TextSpan::new(0, 0, eof_char.to_string())));
        }

        let c: char = self.current_char()?;

        // Check if char is number token
        let start = self.current_pos;
        let mut kind = TokenKind::Bad; // Char that we don't understand
        if Lexer::is_number_start(&c) {
            let number: i64 = self.consume_number();
            kind = TokenKind::Number(number);
        } else if Self::is_whitespace(&c) {
            // Edge case: Whitespace as token
            self.consume();
            kind = TokenKind::Whitespace;
        } else {
            // Edge case: Invalid token
            kind = self.consume_symbol();
        }

        let end = self.current_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);
        Some(Token::new(kind, span))
    }

    fn consume_symbol(&mut self) -> TokenKind {
        let c = self.consume().unwrap();

        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => TokenKind::Bad,
        }
    }

    // Helper method to see if char is a number
    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    // Helper method to consume char for consume_number
    fn consume(&mut self) -> Option<char> {
        let c = self.current_char()?;
        self.current_pos += 1;

        if self.current_pos > self.input.len() {
            return None;
        }

        Some(c)
    }

    fn consume_number(&mut self) -> i64 {
        let start = self.current_pos;

        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume();
            } else {
                break;
            }
        }

        self.input[start..self.current_pos].parse::<i64>().unwrap()
    }
}
