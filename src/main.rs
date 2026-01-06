mod element;
mod lexer;
mod parser;
mod token;

use std::{convert::Infallible, fs, str::FromStr};

pub use element::Element;

#[derive(Clone, Debug)]
pub struct Svg {
    pub children: Vec<Element>,
}

impl FromStr for Svg {
    // TODO: add error type
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lexer = lexer::Lexer::new(lexer::Input::new(input.as_bytes()));
        let mut parser = parser::Parser::new(lexer);
        let ast = parser.parse();

        Ok(Svg { children: vec![] })
    }
}

fn main() {
    let content = fs::read_to_string("simple.svg").unwrap();
    let svg = Svg::from_str(&content).unwrap();
}
