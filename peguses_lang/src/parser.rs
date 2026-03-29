use crate::ast::*;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), String> {
        if self.current().kind == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current().kind))
        }
    }
        pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !matches!(self.current().kind, TokenKind::Eof) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match &self.current().kind {
            TokenKind::Let => self.parse_let(),
            TokenKind::Print => self.parse_print(),
            _ => Err("Unexpected statement".to_string()),
        }
    }
        fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'let'

        let name = match &self.current().kind {
            TokenKind::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => return Err("Expected identifier".to_string()),
        };

        self.expect(TokenKind::Equal)?;

        let value = self.parse_expr()?;

        self.expect(TokenKind::Semicolon)?;

        Ok(Stmt::Let { name, value })
    }
    fn parse_print(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'print'

        let expr = self.parse_expr()?;

        self.expect(TokenKind::Semicolon)?;

        Ok(Stmt::Print { value: expr })
    }
        fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_add_sub()
    }
        fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul_div()?;

        loop {
            match self.current().kind {
                TokenKind::Plus => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Add,
                        right: Box::new(right),
                    };
                }
                TokenKind::Minus => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Sub,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }
        fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        loop {
            match self.current().kind {
                TokenKind::Star => {
                    self.advance();
                    let right = self.parse_primary()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Mul,
                        right: Box::new(right),
                    };
                }
                TokenKind::Slash => {
                    self.advance();
                    let right = self.parse_primary()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Div,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }
        fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current().kind {
            TokenKind::Number(n) => {
                let value = *n;
                self.advance();
                Ok(Expr::Number(value))
            }
            TokenKind::Ident(name) => {
                let n = name.clone();
                self.advance();
                Ok(Expr::Ident(n))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            _ => Err("Unexpected expression".to_string()),
        }
    }

}