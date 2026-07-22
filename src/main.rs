use std::fs;

use esvega::{Input, Lexer, Parser};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "410_2.svg".to_string());
    let content = fs::read_to_string(path).unwrap();

    let lexer = Lexer::new(Input::new(content.as_bytes()));
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(ast) => print!("{}", ast.to_svg()),
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    }
}

