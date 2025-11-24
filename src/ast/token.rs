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
