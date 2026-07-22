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
    /// Carries the opening delimiter byte (`"` or `'`) so the matching closing
    /// quote can be found. XML character/entity references inside the value are
    /// decoded when the literal is emitted.
    Quote(u8),
}
