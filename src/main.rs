mod element;
mod lexer;
mod parser;
mod token;

use std::fs;

pub use element::Element;

fn main() {
    let content = fs::read_to_string("simple.svg").unwrap();

    let mut lexer = lexer::Lexer::new(lexer::Input::new(content.as_bytes()));
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse();
}
