use std::io::{self, BufRead, Write};

use dish::ast::parser::Parser;

const PROMPT: &str = "-> ";
const INTERACTIVE: bool = false;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Starting input loop
    let interactive = std::env::var("INTERACT")
        .ok()
        .and_then(|interactive| Some(interactive == "true"))
        .unwrap_or(INTERACTIVE);

    let mut parser = Parser::from("".to_owned());

    if interactive {
        print!("{}", PROMPT);
        io::stdout().flush()?;
    }

    for line in io::stdin().lock().lines() {
        let input = line?;

        if input.trim_end() == "exit" {
            return Ok(());
        }

        parser.reset(input);

        let expr = parser.expr();
        println!("{}", expr.eval());
        io::stdout().flush()?;

        if interactive {
            print!("{}", PROMPT);
            io::stdout().flush()?;
        }
    }

    return Ok(());
}
