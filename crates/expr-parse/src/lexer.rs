#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Integer(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Ok(Token::Eof);
        }

        let ch = self.current_char().unwrap();

        match ch {
            '+' => {
                self.pos += 1;
                Ok(Token::Plus)
            }
            '-' => {
                self.pos += 1;
                Ok(Token::Minus)
            }
            '*' => {
                self.pos += 1;
                Ok(Token::Star)
            }
            '/' => {
                self.pos += 1;
                Ok(Token::Slash)
            }
            '(' => {
                self.pos += 1;
                Ok(Token::LParen)
            }
            ')' => {
                self.pos += 1;
                Ok(Token::RParen)
            }
            '0'..='9' => self.read_integer(),
            _ => Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn read_integer(&mut self) -> Result<Token, String> {
        let start = self.pos;
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.pos += 1;
            } else {
                break;
            }
        }
        let num_str = &self.input[start..self.pos];
        num_str
            .parse::<i64>()
            .map(Token::Integer)
            .map_err(|e| format!("Invalid integer: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_all(input: &str) -> Result<Vec<Token>, String> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token()?;
            if tok == Token::Eof {
                break;
            }
            tokens.push(tok);
        }
        Ok(tokens)
    }

    #[test]
    fn single_tokens() {
        assert_eq!(lex_all("+").unwrap(), vec![Token::Plus]);
        assert_eq!(lex_all("-").unwrap(), vec![Token::Minus]);
        assert_eq!(lex_all("*").unwrap(), vec![Token::Star]);
        assert_eq!(lex_all("/").unwrap(), vec![Token::Slash]);
        assert_eq!(lex_all("(").unwrap(), vec![Token::LParen]);
        assert_eq!(lex_all(")").unwrap(), vec![Token::RParen]);
    }

    #[test]
    fn integers() {
        assert_eq!(lex_all("0").unwrap(), vec![Token::Integer(0)]);
        assert_eq!(lex_all("42").unwrap(), vec![Token::Integer(42)]);
        assert_eq!(lex_all("12345").unwrap(), vec![Token::Integer(12345)]);
        assert_eq!(
            lex_all("9223372036854775807").unwrap(),
            vec![Token::Integer(i64::MAX)]
        );
    }

    #[test]
    fn multiple_tokens() {
        assert_eq!(
            lex_all("1 + 2").unwrap(),
            vec![Token::Integer(1), Token::Plus, Token::Integer(2)]
        );
        assert_eq!(
            lex_all("(1+2)*3").unwrap(),
            vec![
                Token::LParen,
                Token::Integer(1),
                Token::Plus,
                Token::Integer(2),
                Token::RParen,
                Token::Star,
                Token::Integer(3)
            ]
        );
    }

    #[test]
    fn whitespace_handling() {
        assert_eq!(lex_all("").unwrap(), vec![]);
        assert_eq!(lex_all("   ").unwrap(), vec![]);
        assert_eq!(lex_all("  42  ").unwrap(), vec![Token::Integer(42)]);
        assert_eq!(
            lex_all("1   +   2").unwrap(),
            vec![Token::Integer(1), Token::Plus, Token::Integer(2)]
        );
        assert_eq!(
            lex_all("\t\n1\t+\n2\t").unwrap(),
            vec![Token::Integer(1), Token::Plus, Token::Integer(2)]
        );
    }

    #[test]
    fn invalid_character() {
        assert!(lex_all("@").is_err());
        assert!(lex_all("#").is_err());
        assert!(lex_all("a").is_err());
        assert!(lex_all("1 + a").is_err());
    }

    #[test]
    fn integer_overflow() {
        // One more than i64::MAX
        assert!(lex_all("9223372036854775808").is_err());
    }
}
