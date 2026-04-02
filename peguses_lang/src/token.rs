#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    Let,
    Print,
    If,
    Else,
    While,
    True,
    False,
    
    // Identifiers and literals
    Ident(String),
    Number(i64),
    
    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,  // Modulo
    
    // Comparison operators
    EqualEqual,  // ==
    NotEqual,    // !=
    Less,        // <
    Greater,     // >
    LessEqual,   // <=
    GreaterEqual, // >=
    
    // Logical operators
    And,  // &&
    Or,   // ||
    Not,  // !
    
    // Assignment
    Equal,
    
    // Delimiters
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos : usize,
}

impl Token {
    pub fn new(kind: TokenKind, pos: usize) -> Self {
        Self { kind, pos }
    }
}