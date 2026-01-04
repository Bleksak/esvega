use std::ops::Range;

#[derive(Clone, Debug)]
pub enum TokenKind {
    LessThan,
    LessThanSlash,
    GreaterThan,
    SlashGreaterThan,
    Identifier,
    Literal,
    Quote,
    Equals,
    Comment,
    Text,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Box<String>,
    pub span: Range<usize>,
}
