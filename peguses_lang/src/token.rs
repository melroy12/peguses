#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    Let,
    Print,
    Input,
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    True,
    False,
    
    // Identifiers and literals
    Ident(String),
    Number(i64),
    String(String),
    
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
    DotDot,  // .. for ranges

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