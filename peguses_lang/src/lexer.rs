use crate::token::{Token, TokenKind};

pub struct Lexer {
    input:Vec<char>,
    pos: usize,
}

impl Lexer{
    pub fn new (source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self, start: usize) -> Token {
        let mut value= String::new();
        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let num = value.parse::<i64>().unwrap();
        Token::new(TokenKind::Number(num), start)
    }

    fn read_identifier(&mut self, start: usize) -> Token {
        let mut value = String::new();
        while let Some(c) = self.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let kind = match value.as_str() {
            "let" => TokenKind::Let,
            "print" => TokenKind::Print,
            _ => TokenKind::Ident(value),
        };
        Token::new(kind, start)

    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        let start = self.pos;

        let token= match self.current() {
            Some(c) if c.is_ascii_digit() => self.read_number(start),
            Some(c) if c.is_ascii_alphanumeric() || c == '_' => self.read_identifier(start),
            Some('+') => {self.advance(); Token::new(TokenKind::Plus, start)},
            Some('-') => {self.advance(); Token::new(TokenKind::Minus, start)},
            Some('*') => {self.advance(); Token::new(TokenKind::Star, start)},
            Some('/') => {self.advance(); Token::new(TokenKind::Slash, start)},
            Some('=') => {self.advance(); Token::new(TokenKind::Equal, start)},
            Some(';') => {self.advance(); Token::new(TokenKind::Semicolon, start)},
            Some('(') => {self.advance(); Token::new(TokenKind::LParen, start)},
            Some(')') => {self.advance(); Token::new(TokenKind::RParen, start)},
            None => Token::new(TokenKind::Eof, start),
            Some(c) => return Err(format!("Unexpected character '{}' at position {}", c, start)),
        };
    
    Ok(token)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}