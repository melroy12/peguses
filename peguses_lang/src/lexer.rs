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

    fn skip_comment(&mut self) {
        // Skip single-line comments starting with //
        if self.current() == Some('/') {
            let next_pos = self.pos + 1;
            if self.input.get(next_pos).copied() == Some('/') {
                // Skip until end of line
                while let Some(c) = self.current() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
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
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Ident(value),
        };
        Token::new(kind, start)

    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        
        // Handle comments
        loop {
            self.skip_comment();
            self.skip_whitespace();
            
            // Check if we're at another comment
            if self.current() == Some('/') && self.peek() == Some('/') {
                continue;
            }
            break;
        }
        
        let start = self.pos;

        let token = match self.current() {
            Some(c) if c.is_ascii_digit() => self.read_number(start),
            Some(c) if c.is_ascii_alphanumeric() || c == '_' => self.read_identifier(start),
            Some('+') => {self.advance(); Token::new(TokenKind::Plus, start)},
            Some('-') => {self.advance(); Token::new(TokenKind::Minus, start)},
            Some('*') => {self.advance(); Token::new(TokenKind::Star, start)},
            Some('%') => {self.advance(); Token::new(TokenKind::Percent, start)},
            Some('/') => {self.advance(); Token::new(TokenKind::Slash, start)},
            Some('=') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::new(TokenKind::EqualEqual, start)
                } else {
                    Token::new(TokenKind::Equal, start)
                }
            },
            Some('!') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::new(TokenKind::NotEqual, start)
                } else {
                    Token::new(TokenKind::Not, start)
                }
            },
            Some('<') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::new(TokenKind::LessEqual, start)
                } else {
                    Token::new(TokenKind::Less, start)
                }
            },
            Some('>') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::new(TokenKind::GreaterEqual, start)
                } else {
                    Token::new(TokenKind::Greater, start)
                }
            },
            Some('&') => {
                self.advance();
                if self.current() == Some('&') {
                    self.advance();
                    Token::new(TokenKind::And, start)
                } else {
                    return Err(format!("Expected '&&' at position {}", start));
                }
            },
            Some('|') => {
                self.advance();
                if self.current() == Some('|') {
                    self.advance();
                    Token::new(TokenKind::Or, start)
                } else {
                    return Err(format!("Expected '||' at position {}", start));
                }
            },
            Some(';') => {self.advance(); Token::new(TokenKind::Semicolon, start)},
            Some('(') => {self.advance(); Token::new(TokenKind::LParen, start)},
            Some(')') => {self.advance(); Token::new(TokenKind::RParen, start)},
            Some('{') => {self.advance(); Token::new(TokenKind::LBrace, start)},
            Some('}') => {self.advance(); Token::new(TokenKind::RBrace, start)},
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