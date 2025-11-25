use std::io::{self, Write};

use crate::ast::{lexer::Lexer, parser::Parser};

mod ast;
mod command_parser;

const PROMPT: &str = "-> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let stdin = io::stdin();

    // Starting input loop
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;

        stdin.read_line(&mut input)?;

        if input.trim_end() == "exit" {
            break Ok(());
        }

        let mut lexer = Lexer::new(input.trim_end());
        let mut parser = Parser::new(&mut lexer);

        let expr = parser.expr();
        println!("{}", expr.value());

        input.clear();
    }
}
