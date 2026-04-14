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
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::For => self.parse_for(),
            TokenKind::Break => self.parse_break(),
            TokenKind::Continue => self.parse_continue(),
            TokenKind::Ident(_) => self.parse_assignment(),
            _ => Err(format!("Unexpected statement: {:?}", self.current().kind)),
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
    
    fn parse_assignment(&mut self) -> Result<Stmt, String> {
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

        Ok(Stmt::Assign { name, value })
    }
    
    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'if'

        let condition = self.parse_expr()?;

        self.expect(TokenKind::LBrace)?;
        let then_block = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;

        let else_block = if matches!(self.current().kind, TokenKind::Else) {
            self.advance(); // consume 'else'
            self.expect(TokenKind::LBrace)?;
            let block = self.parse_block()?;
            self.expect(TokenKind::RBrace)?;
            Some(block)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_block,
            else_block,
        })
    }
    
    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'while'

        let condition = self.parse_expr()?;

        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;

        Ok(Stmt::While { condition, body })
    }
    
    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'for'

        let var = match &self.current().kind {
            TokenKind::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => return Err("Expected identifier after 'for'".to_string()),
        };

        self.expect(TokenKind::In)?;

        let start = self.parse_expr()?;

        self.expect(TokenKind::DotDot)?;

        let end = self.parse_expr()?;

        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;

        Ok(Stmt::For { var, start, end, body })
    }
    
    fn parse_break(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'break'
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Break)
    }
    
    fn parse_continue(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'continue'
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Continue)
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }
        fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_logical_or()
    }
    
    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_and()?;

        while matches!(self.current().kind, TokenKind::Or) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    
    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        while matches!(self.current().kind, TokenKind::And) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_add_sub()?;

        while let Some(op) = match self.current().kind {
            TokenKind::EqualEqual => Some(BinOp::Equal),
            TokenKind::NotEqual => Some(BinOp::NotEqual),
            TokenKind::Less => Some(BinOp::Less),
            TokenKind::Greater => Some(BinOp::Greater),
            TokenKind::LessEqual => Some(BinOp::LessEqual),
            TokenKind::GreaterEqual => Some(BinOp::GreaterEqual),
            _ => None,
        } {
            self.advance();
            let right = self.parse_add_sub()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
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
        let mut left = self.parse_unary()?;

        loop {
            match self.current().kind {
                TokenKind::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Mul,
                        right: Box::new(right),
                    };
                }
                TokenKind::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Div,
                        right: Box::new(right),
                    };
                }
                TokenKind::Percent => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Mod,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.current().kind {
            TokenKind::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            TokenKind::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_primary(),
        }
    }
        fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current().kind {
            TokenKind::Number(n) => {
                let value = *n;
                self.advance();
                Ok(Expr::Number(value))
            }
            TokenKind::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expr::String(value))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Boolean(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Boolean(false))
            }
            TokenKind::Input => {
                self.advance(); // consume 'input'
                self.expect(TokenKind::LParen)?;
                self.expect(TokenKind::RParen)?;
                Ok(Expr::Input)
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
            _ => Err(format!("Unexpected token in expression: {:?}", self.current().kind)),
        }
    }

}