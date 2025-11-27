use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Operator(Op),
    LParen,
    RParen,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Op::Plus => '+',
            Op::Minus => '-',
            Op::Star => '*',
            Op::Slash => '/',
        };

        write!(f, "{}", symbol)
    }
}
