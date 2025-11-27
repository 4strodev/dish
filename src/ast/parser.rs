use std::fmt::Display;

use crate::ast::{
    lexer::Lexer,
    token::{Op, Token},
};

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    BinaryOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Self::Number(value) => *value,
            Self::BinaryOp { left, op, right } => match op {
                Op::Plus => left.eval() + right.eval(),
                Op::Minus => left.eval() - right.eval(),
                Op::Star => left.eval() * right.eval(),
                Op::Slash => left.eval() / right.eval(),
            },
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::BinaryOp { left, op, right } => write!(f, "({} {} {})", op, left, right),
        };
    }
}

#[derive(Debug)]
pub struct Parser<T: AsRef<str>> {
    lexer: Lexer<T>,
    current_token: Token,
}

impl<T: AsRef<str>> Parser<T> {
    pub fn new(mut lexer: Lexer<T>) -> Self {
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    pub fn from(input: T) -> Parser<T> {
        let lexer = Lexer::new(input);
        return Parser::new(lexer);
    }

    pub fn reset(&mut self, input: T) {
        let lexer = Lexer::new(input);
        self.lexer = lexer;
        self.current_token = self.lexer.get_next_token();
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!(
                "Syntax error: expected {:?}, got {:?}",
                token, self.current_token
            );
        }
    }

    fn factor(&mut self) -> Expr {
        match self.current_token {
            Token::Number(n) => {
                self.eat(Token::Number(n));
                Expr::Number(n)
            }
            Token::LParen => {
                self.eat(Token::LParen);
                let node = self.expr();
                self.eat(Token::RParen);
                node
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn term(&mut self) -> Expr {
        let mut node = self.factor();
        while matches!(
            self.current_token,
            Token::Operator(Op::Star) | Token::Operator(Op::Slash)
        ) {
            if let Token::Operator(op) = self.current_token.clone() {
                self.eat(Token::Operator(op.clone()));
                node = Expr::BinaryOp {
                    left: Box::new(node),
                    op: op,
                    right: Box::new(self.factor()),
                };
            }
        }
        node
    }

    pub fn expr(&mut self) -> Expr {
        let mut node = self.term();
        while matches!(
            self.current_token,
            Token::Operator(Op::Plus) | Token::Operator(Op::Minus)
        ) {
            if let Token::Operator(op) = self.current_token.clone() {
                self.eat(Token::Operator(op.clone()));
                node = Expr::BinaryOp {
                    left: Box::new(node),
                    op,
                    right: Box::new(self.term()),
                };
            }
        }
        node
    }
}
