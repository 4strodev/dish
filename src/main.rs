use std::io::{self, BufRead, Write};

use crate::ast::{lexer::Lexer, parser::Parser};

mod ast;
mod command_parser;

const PROMPT: &str = "-> ";
const INTERACTIVE: bool = false;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Starting input loop
    let interactive = std::env::var("INTERACT")
        .ok()
        .and_then(|interactive| Some(interactive == "true"))
        .unwrap_or(INTERACTIVE);

    if interactive {
        print!("{}", PROMPT);
        io::stdout().flush()?;
    }

    for line in io::stdin().lock().lines() {
        let input = line?;

        if interactive {
            print!("{}", PROMPT);
            io::stdout().flush()?;
        }

        if input.trim_end() == "exit" {
            return Ok(());
        }

        let mut lexer = Lexer::new(input.trim_end());
        let mut parser = Parser::new(&mut lexer);

        let expr = parser.expr();
        println!("{}", expr.eval());
    }

    return Ok(());
}
