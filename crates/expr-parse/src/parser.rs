use crate::lexer::{Lexer, Token};
use expr_core::{BinOp, Expr};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expression()?;
        if self.current != Token::Eof {
            return Err(format!("Unexpected token: {:?}", self.current));
        }
        Ok(expr)
    }

    fn advance(&mut self) -> Result<(), String> {
        self.current = self.lexer.next_token()?;
        Ok(())
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        loop {
            let op = match &self.current {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_factor()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        loop {
            let op = match &self.current {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_primary()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current {
            Token::Integer(n) => {
                let n = *n;
                self.advance()?;
                Ok(Expr::Integer(n))
            }
            Token::LParen => {
                self.advance()?;
                let expr = self.parse_expression()?;
                if self.current != Token::RParen {
                    return Err("Expected ')'".to_string());
                }
                self.advance()?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current)),
        }
    }
}
