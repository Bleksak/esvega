/// The lexer's current scanning mode.
///
/// Controls how the lexer interprets bytes during tokenization.
#[derive(Default, Clone, Debug)]
pub enum LexerMode {
    /// Scanning text content (character data between tags).
    ///
    /// Looks for `<` or `</` to transition into markup mode.
    #[default]
    Text,
    /// Scanning markup (element names, attributes, delimiters).
    ///
    /// Parses identifiers, `=` signs, quoted values, and `>` / `/>` closers.
    Markup,
    /// Scanning a quoted attribute value.
    ///
    /// Looks for the closing quote `"`, handling backslash escapes.
    Quote,
}
