use crate::ast::token::{Op, Token};

#[derive(Debug)]
pub struct Lexer<T: AsRef<str>> {
    input: T,
    pos: usize,
    current_char: Option<char>,
}

impl<T: AsRef<str>> Lexer<T> {
    pub fn new(input: T) -> Self {
        let current_char = input.as_ref().chars().next();
        Lexer {
            input,
            pos: 0,
            current_char,
        }
    }

    pub fn reset(&mut self, input: T) {
        self.current_char = input.as_ref().chars().next();
        self.input = input;
        self.pos = 0;
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.input.as_ref().chars().nth(self.pos);
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current_char, Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    fn number(&mut self) -> f64 {
        let mut num = String::new();
        while matches!(self.current_char, Some(c) if c.is_digit(10)) {
            num.push(self.current_char.unwrap());
            self.advance();
        }

        num.parse::<f64>().unwrap()
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if c.is_digit(10) {
                return Token::Number(self.number());
            }

            self.advance();
            return match c {
                '+' => Token::Operator(Op::Plus),
                '-' => Token::Operator(Op::Minus),
                '*' => Token::Operator(Op::Star),
                '/' => Token::Operator(Op::Slash),
                '(' => Token::LParen,
                ')' => Token::RParen,
                _ => panic!("Unknown character: {}", c),
            };
        }

        return Token::EOF;
    }
}
