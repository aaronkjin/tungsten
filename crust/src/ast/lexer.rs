use std::fmt::{ Display, Formatter };

#[derive(Debug, PartialEq, Clone)]
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
    Let,
    Identifier(String),
    Equals,
    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Number(_) => write!(f, "Number"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::Bad => write!(f, "Bad"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Identifier(_) => write!(f, "Identifier"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
        // let mut kind = TokenKind::Bad; // Char that we don't understand
        let kind: TokenKind;
        if Lexer::is_number_start(&c) {
            let number: i64 = self.consume_number();
            kind = TokenKind::Number(number);
        } else if Self::is_whitespace(&c) {
            // Edge case: Whitespace as token
            self.consume();
            kind = TokenKind::Whitespace;
        } else if Self::is_identifier_start(&c) {
            let identifier = self.consume_identifier();
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
            '=' => TokenKind::Equals,
            _ => TokenKind::Bad,
        }
    }

    // Helper method to see if char is a number
    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic()
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

    // To consume non-numeric, alphabetical identifiers
    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(c) = self.current_char() {
            if Self::is_identifier_start(&c) {
                self.consume().unwrap();
                identifier.push(c);
            } else {
                break;
            }
        }
    }
}
