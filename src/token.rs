//! Token types emitted by the lexer.

use std::ops::Range;

/// The kind of a [`Token`].
#[derive(Clone, Debug)]
pub enum TokenKind {
    /// A `<` delimiter.
    LessThan,
    /// A `</` delimiter (tag close start).
    LessThanSlash,
    /// A `>` delimiter (tag close).
    GreaterThan,
    /// A `/>` delimiter (self-closing tag).
    SlashGreaterThan,
    /// An identifier: element name or attribute name.
    Identifier,
    /// A literal value inside quotes (attribute value).
    Literal,
    /// The opening quote `"` of an attribute value.
    Quote,
    /// An `=` sign (attribute equals).
    Equals,
    /// A complete comment (`<!-- ... -->`).
    Comment,
    /// Text content between tags.
    Text,
}

/// A single token emitted by the lexer.
///
/// Contains the token type, its string value, and the byte span
/// within the original source.
#[derive(Clone, Debug)]
pub struct Token {
    /// The kind of this token.
    pub kind: TokenKind,
    /// The token's text value.
    pub value: String,
    /// Byte range in the original source (inclusive start, exclusive end).
    #[allow(dead_code)]
    pub span: Range<usize>,
}
