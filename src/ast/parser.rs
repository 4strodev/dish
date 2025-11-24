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
    pub fn value(&self) -> f64 {
        match self {
            Self::Number(value) => *value,
            Self::BinaryOp { left, op, right } => match op {
                Op::Plus => left.value() + right.value(),
                Op::Minus => left.value() - right.value(),
                Op::Star => left.value() * right.value(),
                Op::Slash => left.value() / right.value(),
            },
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a, T: AsRef<str>> {
    lexer: &'a mut Lexer<T>,
    current_token: Token,
}

impl<'a, T: AsRef<str>> Parser<'a, T> {
    pub fn new(lexer: &'a mut Lexer<T>) -> Self {
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
        }
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
    pub fn reset(&mut self, input: T) {
        self.lexer.reset(input);
        self.current_token = self.lexer.get_next_token();
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
