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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Result<Expr, String> {
        Parser::new(input)?.parse()
    }

    fn binop(op: BinOp, left: Expr, right: Expr) -> Expr {
        Expr::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    #[test]
    fn single_integer() {
        assert_eq!(parse("42").unwrap(), Expr::Integer(42));
        assert_eq!(parse("0").unwrap(), Expr::Integer(0));
    }

    #[test]
    fn binary_operations() {
        assert_eq!(
            parse("1 + 2").unwrap(),
            binop(BinOp::Add, Expr::Integer(1), Expr::Integer(2))
        );
        assert_eq!(
            parse("3 - 4").unwrap(),
            binop(BinOp::Sub, Expr::Integer(3), Expr::Integer(4))
        );
        assert_eq!(
            parse("5 * 6").unwrap(),
            binop(BinOp::Mul, Expr::Integer(5), Expr::Integer(6))
        );
        assert_eq!(
            parse("7 / 8").unwrap(),
            binop(BinOp::Div, Expr::Integer(7), Expr::Integer(8))
        );
    }

    #[test]
    fn precedence() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        assert_eq!(
            parse("1 + 2 * 3").unwrap(),
            binop(
                BinOp::Add,
                Expr::Integer(1),
                binop(BinOp::Mul, Expr::Integer(2), Expr::Integer(3))
            )
        );
        // 2 * 3 + 4 should parse as (2 * 3) + 4
        assert_eq!(
            parse("2 * 3 + 4").unwrap(),
            binop(
                BinOp::Add,
                binop(BinOp::Mul, Expr::Integer(2), Expr::Integer(3)),
                Expr::Integer(4)
            )
        );
    }

    #[test]
    fn associativity() {
        // 10 - 3 - 2 should parse as (10 - 3) - 2, not 10 - (3 - 2)
        assert_eq!(
            parse("10 - 3 - 2").unwrap(),
            binop(
                BinOp::Sub,
                binop(BinOp::Sub, Expr::Integer(10), Expr::Integer(3)),
                Expr::Integer(2)
            )
        );
        // 20 / 4 / 2 should parse as (20 / 4) / 2
        assert_eq!(
            parse("20 / 4 / 2").unwrap(),
            binop(
                BinOp::Div,
                binop(BinOp::Div, Expr::Integer(20), Expr::Integer(4)),
                Expr::Integer(2)
            )
        );
    }

    #[test]
    fn parentheses() {
        // (1 + 2) * 3
        assert_eq!(
            parse("(1 + 2) * 3").unwrap(),
            binop(
                BinOp::Mul,
                binop(BinOp::Add, Expr::Integer(1), Expr::Integer(2)),
                Expr::Integer(3)
            )
        );
        // Nested: ((1))
        assert_eq!(parse("((1))").unwrap(), Expr::Integer(1));
        // Deeply nested
        assert_eq!(parse("((((42))))").unwrap(), Expr::Integer(42));
    }

    #[test]
    fn complex_expressions() {
        // (2 + 3) * (4 - 1)
        assert_eq!(
            parse("(2 + 3) * (4 - 1)").unwrap(),
            binop(
                BinOp::Mul,
                binop(BinOp::Add, Expr::Integer(2), Expr::Integer(3)),
                binop(BinOp::Sub, Expr::Integer(4), Expr::Integer(1))
            )
        );
    }

    #[test]
    fn error_empty_input() {
        assert!(parse("").is_err());
    }

    #[test]
    fn error_unclosed_paren() {
        assert!(parse("(1 + 2").is_err());
        assert!(parse("((1)").is_err());
    }

    #[test]
    fn error_extra_close_paren() {
        assert!(parse("1 + 2)").is_err());
    }

    #[test]
    fn error_missing_operand() {
        assert!(parse("1 +").is_err());
        assert!(parse("* 2").is_err());
        assert!(parse("+").is_err());
    }

    #[test]
    fn error_missing_operator() {
        assert!(parse("1 2").is_err());
    }

    #[test]
    fn error_trailing_tokens() {
        assert!(parse("1 + 2 3").is_err());
    }
}
