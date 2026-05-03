use std::fs;

use esvega::{Input, Lexer, Parser};

fn main() {
    let content = fs::read_to_string("410_2.svg").unwrap();

    let lexer = Lexer::new(Input::new(content.as_bytes()));
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    if let Some(ast) = ast {
        print!("{}", ast.to_svg());
    }
}

