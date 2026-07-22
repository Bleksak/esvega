//! XML character/entity reference decoding and encoding.
//!
//! Decoding turns references such as `&amp;`, `&lt;`, and `&#169;` into their
//! literal characters when parsing attribute values and text content. Encoding
//! is the inverse: it escapes the characters that cannot appear literally when
//! serializing, so a parsed document round-trips back to valid XML.

use std::fmt;

use memchr::memchr;

/// Decodes XML character and entity references in `bytes` into a `String`.
///
/// Handles the five predefined XML entities (`&amp;`, `&lt;`, `&gt;`, `&quot;`,
/// `&apos;`) and numeric character references (`&#DDD;` decimal, `&#xHHH;` hex).
/// Anything that does not form a recognized reference is passed through
/// verbatim, keeping the lexer lenient with imperfect input; a bare `&` that
/// survives is re-escaped to `&amp;` on the way out (see [`encode_into`]).
pub fn decode(bytes: &[u8]) -> String {
    // Fast path: without an `&` there is nothing to decode.
    let Some(first) = memchr(b'&', bytes) else {
        // SAFETY: `bytes` is a subslice of the validated UTF-8 input.
        return unsafe { std::str::from_utf8_unchecked(bytes) }.to_owned();
    };

    let mut out = Vec::with_capacity(bytes.len());
    out.extend_from_slice(&bytes[..first]);

    let mut i = first;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            // Copy the verbatim run up to the next `&` in one shot.
            let rest = &bytes[i..];
            let next = memchr(b'&', rest).unwrap_or(rest.len());
            out.extend_from_slice(&rest[..next]);
            i += next;
            continue;
        }

        // At `&`: a reference runs up to the next `;`.
        match memchr(b';', &bytes[i..]) {
            Some(semi) if decode_reference(&bytes[i + 1..i + semi], &mut out) => {
                i += semi + 1;
            }
            _ => {
                // Not a well-formed reference; emit the `&` literally.
                out.push(b'&');
                i += 1;
            }
        }
    }

    // SAFETY: verbatim runs preserve the input's UTF-8, and decoded references
    // are pushed as valid UTF-8 (see `push_char`), so the result is valid UTF-8.
    unsafe { String::from_utf8_unchecked(out) }
}

/// Decodes the body of a single reference (the bytes between `&` and `;`) and
/// appends it to `out`. Returns `false` if `body` is not a recognized reference,
/// in which case `out` is left untouched.
fn decode_reference(body: &[u8], out: &mut Vec<u8>) -> bool {
    let decoded = match body {
        b"amp" => '&',
        b"lt" => '<',
        b"gt" => '>',
        b"quot" => '"',
        b"apos" => '\'',
        [b'#', rest @ ..] => {
            let code = match rest {
                [b'x' | b'X', hex @ ..] => parse_radix(hex, 16),
                dec => parse_radix(dec, 10),
            };
            match code.and_then(char::from_u32) {
                Some(c) => c,
                None => return false,
            }
        }
        _ => return false,
    };

    push_char(out, decoded);
    true
}

/// Parses `digits` as a number in `radix`, returning `None` if empty or if any
/// byte is not a valid digit.
fn parse_radix(digits: &[u8], radix: u32) -> Option<u32> {
    if digits.is_empty() {
        return None;
    }

    let mut value: u32 = 0;
    for &byte in digits {
        let digit = (byte as char).to_digit(radix)?;
        value = value.checked_mul(radix)?.checked_add(digit)?;
    }

    Some(value)
}

/// Appends the UTF-8 encoding of `c` to `out`.
fn push_char(out: &mut Vec<u8>, c: char) {
    let mut buffer = [0u8; 4];
    out.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
}

/// Writes `value` to `f`, escaping the characters that cannot appear literally
/// in XML output: `&`, `<`, `>`, and `"`.
///
/// This is the inverse of [`decode`]: because the parser stores fully-decoded
/// text, every reserved character present here was a genuine character in the
/// source and must be re-escaped. `"` is escaped so the result is safe inside a
/// double-quoted attribute value; escaping it in text content is harmless.
pub fn encode_into(f: &mut impl fmt::Write, value: &str) -> fmt::Result {
    let bytes = value.as_bytes();
    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let replacement = match byte {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            _ => continue,
        };

        // Flush the verbatim run before this reserved byte, then the entity.
        // All reserved bytes are ASCII, so slicing on byte indices is UTF-8 safe.
        f.write_str(&value[start..i])?;
        f.write_str(replacement)?;
        start = i + 1;
    }

    f.write_str(&value[start..])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(value: &str) -> String {
        let mut out = String::new();
        encode_into(&mut out, value).unwrap();
        out
    }

    #[test]
    fn encode_escapes_reserved_characters() {
        assert_eq!(encode(r#"a & b < c > d "e""#), "a &amp; b &lt; c &gt; d &quot;e&quot;");
    }

    #[test]
    fn encode_leaves_plain_text_untouched() {
        assert_eq!(encode("plain text 123"), "plain text 123");
    }

    #[test]
    fn decode_encode_round_trips_bare_ampersand() {
        // A bare `&` (or an unrecognized entity) decodes verbatim, then encodes
        // back to a valid, escaped form.
        let decoded = decode(b"a &bogus; b");
        assert_eq!(decoded, "a &bogus; b");
        assert_eq!(encode(&decoded), "a &amp;bogus; b");
    }

    #[test]
    fn decode_preserves_multibyte_utf8() {
        assert_eq!(decode("π &amp; €".as_bytes()), "π & €");
    }
}
