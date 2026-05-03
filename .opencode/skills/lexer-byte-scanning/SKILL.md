---
name: lexer-byte-scanning
description: Tokenizes SVG source bytes using `memchr` and state modes. Use when modifying `src/lexer/mod.rs`, `src/lexer/mode.rs`, or adding new `TokenKind` variants. Handles Text/Markup/Quote transitions and span tracking. Do NOT use for AST construction or semantic validation.
---

# Lexer Byte-Scanning

## Critical
- **Always use `memchr` for initial byte scanning** — never use `str::find` or manual iteration for multi-byte delimiter detection. The `memchr` crate is a project dependency (`Cargo.toml`)
- **Span tracking is mandatory** — every `Token` must carry `span: Span` from `src/element/types.rs`. Never create tokens without span context
- **Mode transitions must be exhaustive** — every mode must have a defined next mode for all input bytes. Unhandled bytes panic in test builds
- **Quote modes (`'` and `"`) are independent** — a `"` inside `'`-quoted mode does NOT end the quote. Track quote character in state, not just boolean

## Instructions

### 1. Define new `TokenKind` variants in `src/element/types.rs`
1. Open `src/element/types.rs` and locate the `pub enum TokenKind` definition
2. Add new variants using `pub enum` syntax matching existing variants (e.g., `MarkupStart`, `MarkupEnd`, `Text`, `QuoteStart`, `QuoteEnd`, `Name`, `AttributeValue`)
3. Each variant MUST include the span: `pub struct TokenKind { pub span: Span, ... }` OR use the existing `Token` wrapper: `pub struct Token { pub kind: TokenKind, pub span: Span }`
4. Verify: `cargo check --lib` passes — no missing imports from `src/element/types.rs`

### 2. Implement the mode state machine in `src/lexer/mode.rs`
1. Open `src/lexer/mode.rs`. The mode enum must be: `pub enum Mode { Markup, Text, Quote(char), ... }`
2. For **Markup mode**: scan forward using `memchr::memchr(b'/', &self.input[self.pos..])` to find tag boundaries. On `<` at current position, emit `MarkupStart`. On `>`, emit `MarkupEnd` and switch to `Text` mode (or inner mode if `<svg`, `<text`, etc.)
3. For **Text mode**: scan using `memchr::memchr2(b'<', b'&', &self.input[self.pos..])`. On `<`, switch to `Markup` mode. On `&`, begin HTML entity parsing. Emit `Text` token for the entire run between delimiters
4. For **Quote mode**: store the opening quote char (`'` or `"`). Scan using `memchr::memchr(quote, &self.input[self.pos..])`. On escape `\`, skip next byte. On matching quote, emit `QuoteEnd` and switch to `Markup` mode
5. Verify: every `Mode` variant has a `next_mode(input: &[u8], pos: usize) -> (Mode, usize)` method. No `unimplemented!()` or `panic!()` in release builds

### 3. Implement byte-scanning in `src/lexer/mod.rs`
1. Open `src/lexer/mod.rs`. The main `Lexer` struct must hold: `input: &[u8], pos: usize, mode: Mode`
2. The `next_token(&mut self) -> Result<Token, LexerError>` method:
   - Check `self.pos >= self.input.len()` → return `Ok(Token::Eof)` with span
   - Dispatch to `match self.mode` → call the mode-specific scanner
3. For all scanner implementations:
   ```rust
   use memchr::memchr;
   let haystack = &self.input[self.pos..];
   let offset = memchr(delimiter_byte, haystack).unwrap_or(haystack.len());
   let token_end = self.pos + offset;
   let token_text = &self.input[self.pos..token_end];
   let span = Span::new(self.pos, token_end);
   self.pos = token_end;
   ```
4. Emit tokens with the correct `TokenKind` and computed `Span`
5. Verify: `cargo test --lib lexer` — all existing tokenization tests pass. Add a test for any new mode transition

### 4. Handle SVG-specific constructs
1. **Opening tags**: `<tagname` — scan `tagname` using `memchr::memchr2(b' ', b'>', &self.input[self.pos..])` to find attribute boundaries or tag end
2. **Closing tags**: `</tagname>` — scan for `>` similarly
3. **Self-closing tags**: `<tagname .../>` — scan until `/` before `>`, emit as `MarkupSelfClose`
4. **Attributes**: `name="value"` — after scanning `name` (until `=`, space, or `>`), enter `Quote` mode if next char is `"` or `'`
5. Verify: parse `simple.svg` and `invalid_child.svg` — output matches expected token sequence

### 5. Error handling and edge cases
1. **Unterminated quotes**: if EOF reached in Quote mode, return `Err(LexerError::UnterminatedQuote { expected: char, span })`
2. **Malformed tags**: `<` not followed by valid name (letter, `/`, `!`) → return `Err(LexerError::InvalidMarkupStart { span })`
3. **Nested markup**: `<text><b>bold</b></text>` — track nesting depth in `Markup` mode if needed by downstream parser. Currently, mode-based scanning handles this via mode transitions
4. **Unicode**: SVG identifiers use ASCII. Text content may be UTF-8. Use `self.input[self.pos]` for byte-level checks, never `self.input[self.pos..].chars().next()` unless extracting text content
5. Verify: `cargo clippy --lib` — no warnings in lexer module. All `Err` variants match the error type in `src/element/types.rs`

## Examples

**User says**: "Add support for CDATA sections in SVG text content"
**Actions taken**:
1. Add `TokenKind::CDataStart` and `TokenKind::CDataEnd` to `src/element/types.rs`
2. In Markup mode scanner, detect `<![CDATA[` using `memchr` chain: first find `<`, then `!`, then `[`, then `CDATA[` (byte-by-byte comparison)
3. On `<![CDATA[`, emit `CDataStart`, switch to a new `Mode::CData` state
4. `Mode::CData` scans for `]]>` using `memchr2(b']', b'>', ...)` then verifies the sequence `]]>`
5. Emit `CDataEnd`, return to `Text` mode
**Result**: New token stream includes `CDataStart`, `Text`, `CDataEnd` for `<![CDATA[...]]>` sections

**User says**: "Fix the lexer hanging on large SVG files"
**Actions taken**:
1. Check `src/lexer/mod.rs` for `str::find` or `input[start..].find(...)` patterns — replace with `memchr::memchr`
2. Ensure `Span` computation uses byte offsets, not char offsets: `Span::new(self.pos, self.pos + chunk_len)`
3. Verify no allocation in hot path — `token_text` should be `&[u8]` slice, never `String`
**Result**: Lexer processes 10MB SVG files in <200ms instead of hanging

**User says**: "Add a test for unterminated attribute quotes"
**Actions taken**:
```rust
#[test]
fn unterminated_quote() {
    let input = r#"<circle cx="50" cy=50" r=30""#;
    let mut lexer = Lexer::new(input.as_bytes());
    // ... consume valid tokens ...
    let err = lexer.next_token().unwrap_err();
    assert!(matches!(err, LexerError::UnterminatedQuote { .. }));
}
```
**Result**: Test suite catches the error early; `cargo test --lib lexer` shows the new test passing

## Common Issues

| Error Message | Cause | Fix |
|---|---|---|
| `index out of bounds: the len is N but the index is M` | `memchr` returns `None`, code tries `unwrap()` on slice with no delimiter | Use `unwrap_or(haystack.len())` — the `memchr` result should default to end of input |
| `cannot borrow self.input as mutable` | Trying to advance `pos` while holding immutable slice reference | Advance `self.pos` directly; do not re-slice with `&mut` — `self.input` is `&[u8]` |
| `TokenKind::SomeVariant does not have a span field` | Token struct changed but variant definition did not | Check `src/element/types.rs` — `Token` wraps `TokenKind` and `Span`; do not put span in `TokenKind` itself |
| Lexer produces no tokens for valid SVG | Mode never transitions from initial state | Verify `Lexer::new()` sets initial mode to `Mode::Markup` (not `Mode::Text`). Check `src/lexer/mod.rs` line ~15 |
| `unterminated quote` on valid `""` (empty attribute) | Quote scanner skips the closing `"` thinking it's escaped | Ensure escape check is `self.pos + 1 < self.input.len() && self.input[self.pos + 1] == quote` before advancing |
| Panic on `</>` (invalid closing tag) | No validation of tag name after `/` in closing tag scan | Add check: after finding `/`, the next byte must be a valid identifier start (letter or `_`). Return `LexerError::InvalidMarkupStart { span }` |
| Slow parsing on large files (>5MB) | Using `input.find()` or iterating byte-by-byte | Replace all `input[pos..].find(byte)` with `memchr::memchr(byte, &input[pos..])`. Verify with `cargo bench --bench lexer` |
| `Span` offsets are wrong after Unicode text | Using `char` length instead of `byte` length for text tokens | `Span` is byte-based. For text content, use `self.pos..self.pos + text_slice.len()` NOT `.chars().count()` |