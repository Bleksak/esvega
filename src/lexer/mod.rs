mod mode;
use std::ops::Range;

use memchr::memchr;
use memchr::memmem::find;

use crate::{
    lexer::mode::LexerMode,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Input<'a> {
    pub bytes: &'a [u8],
    pub length: usize,
    pub offset: usize,
}

impl<'a> Input<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            length: bytes.len(),
            offset: 0,
        }
    }

    pub const fn has_reached_eof(&self) -> bool {
        self.offset >= self.length
    }

    pub fn skip(&mut self, n: usize) {
        self.offset = (self.offset + n).min(self.length);
    }

    pub fn next(&mut self) {
        if !self.has_reached_eof() {
            self.offset += 1;
        }
    }

    const fn calculate_bound(&self, n: usize) -> (usize, usize) {
        if self.has_reached_eof() {
            return (self.length, self.length);
        }

        let mut until = self.offset + n;

        if until >= self.length {
            until = self.length;
        }

        (self.offset, until)
    }

    pub fn consume(&mut self, n: usize) -> &'a [u8] {
        let (from, until) = self.calculate_bound(n);

        self.skip(n);

        &self.bytes[from..until]
    }

    pub fn consume_remaining(&mut self) -> &'a [u8] {
        if self.has_reached_eof() {
            return &[];
        }

        let from = self.offset;
        self.offset = self.length;

        &self.bytes[from..]
    }

    pub fn consume_whitespace(&mut self) {
        while !self.has_reached_eof() && self.bytes[self.offset].is_ascii_whitespace() {
            self.offset += 1;
        }
    }

    pub fn is_at(&self, search: &[u8], ignore_ascii_case: bool) -> bool {
        let (from, until) = self.calculate_bound(search.len());
        let slice = &self.bytes[from..until];

        if ignore_ascii_case {
            slice.eq_ignore_ascii_case(search)
        } else {
            slice == search
        }
    }

    pub fn consume_until(&mut self, search: &[u8], ignore_ascii_case: bool) -> &'a [u8] {
        let start = self.offset;
        if !ignore_ascii_case {
            // For a single-byte search, use memchr.
            if search.len() == 1 {
                if let Some(pos) = memchr(search[0], &self.bytes[self.offset..]) {
                    self.offset += pos;
                    &self.bytes[start..self.offset]
                } else {
                    self.offset = self.length;
                    &self.bytes[start..self.length]
                }
            } else if let Some(pos) = find(&self.bytes[self.offset..], search) {
                self.offset += pos;
                &self.bytes[start..self.offset]
            } else {
                self.offset = self.length;
                &self.bytes[start..self.length]
            }
        } else {
            while !self.has_reached_eof() && !self.is_at(search, ignore_ascii_case) {
                self.offset += 1;
            }

            &self.bytes[start..self.offset]
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'input> {
    pub input: Input<'input>,
    pub mode: LexerMode,
}

// TODO: The lexer does not produce attributes like autofocus that do not need a value
impl<'input> Lexer<'input> {
    pub fn new(input: Input<'input>) -> Self {
        Self {
            input,
            mode: LexerMode::default(),
        }
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.input.consume_whitespace();

        let token_start = self.input.offset;

        while !self.input.has_reached_eof() {
            let byte = self.input.bytes[self.input.offset];

            if self.input.is_at(b"<!--", false) {
                self.input.consume_until(b"-->", false);

                if !self.input.is_at(b"-->", false) {
                    // TODO: Error
                    return None;
                }
                self.input.consume(3);

                return Some(self.token(TokenKind::Comment, token_start..self.input.offset));
            }

            match self.mode {
                LexerMode::Quote => {
                    // this emits the ending quote and sets the mode back to markup
                    if self.input.is_at(b"\"", false) {
                        self.input.consume(1);
                        self.mode = LexerMode::Markup;

                        return Some(self.token(TokenKind::Quote, token_start..self.input.offset));
                    }
                    loop {
                        self.input.consume_until(b"\"", false);
                        if !self.input.is_at(b"\"", false) {
                            // TODO: Error
                            return None;
                        }

                        if self.input.bytes[self.input.offset - 1] != b'\\' {
                            break;
                        }

                        self.input.consume(1);
                    }

                    return Some(self.token(TokenKind::Literal, token_start..self.input.offset));
                }
                LexerMode::Markup => {
                    if self.input.is_at(b">", false) {
                        self.mode = LexerMode::Text;
                        self.input.consume(1);

                        return Some(
                            self.token(TokenKind::GreaterThan, token_start..self.input.offset),
                        );
                    }

                    if self.input.is_at(b"/", false) {
                        self.input.consume_until(b">", false);

                        if !self.input.is_at(b">", false) {
                            // TODO: Error - we're at something like "<div /x
                            return None;
                        }
                        self.input.consume(1);

                        self.mode = LexerMode::Text;

                        return Some(
                            self.token(TokenKind::SlashGreaterThan, token_start..self.input.offset),
                        );
                    }

                    match byte {
                        // here we need to find attributes, equal signs, and values
                        b'=' => {
                            self.input.consume(1);
                            return Some(
                                self.token(TokenKind::Equals, token_start..self.input.offset),
                            );
                        }
                        _ => {
                            // here we will find attribute values inside of quotes, we need to be able
                            // to handle escaped quotes as proper values
                            if self.input.is_at(b"\"", false) {
                                self.mode = LexerMode::Quote;
                                self.input.consume(1);
                                return Some(
                                    self.token(TokenKind::Quote, token_start..self.input.offset),
                                );
                            }

                            // this is the last part -> we are looking for identifiers(attribute names)
                            // we know that we are at the start of an identifier, and we need to find
                            // the end of the word - which is either a space, equal sign, or >
                            loop {
                                if self.input.is_at(b" ", false) {
                                    return Some(self.token(
                                        TokenKind::Identifier,
                                        token_start..self.input.offset,
                                    ));
                                }
                                if self.input.is_at(b"=", false) {
                                    return Some(self.token(
                                        TokenKind::Identifier,
                                        token_start..self.input.offset,
                                    ));
                                }
                                if self.input.is_at(b">", false) {
                                    return Some(self.token(
                                        TokenKind::Identifier,
                                        token_start..self.input.offset,
                                    ));
                                }
                                self.input.consume(1);
                            }
                        }
                    }
                }
                LexerMode::Text => {
                    if self.input.is_at(b"</", false) {
                        self.input.consume(2);
                        self.mode = LexerMode::Markup;

                        return Some(
                            self.token(TokenKind::LessThanSlash, token_start..self.input.offset),
                        );
                    }

                    if self.input.is_at(b"<", false) {
                        self.mode = LexerMode::Markup;
                        self.input.consume(1);

                        return Some(
                            self.token(TokenKind::LessThan, token_start..self.input.offset),
                        );
                    }

                    // here we just consume until eof or LessThan sign
                    self.input.consume_until(b"<", false);
                    return Some(self.token(TokenKind::Text, token_start..self.input.offset));
                }
            }
        }

        None
    }

    #[inline]
    fn token(&mut self, kind: TokenKind, span: Range<usize>) -> Token {
        // SAFETY: The input bytes are guaranteed to be valid UTF-8 because:
        // All byte slices here are subslices of the validated input
        let string =
            unsafe { std::str::from_utf8_unchecked(&self.input.bytes[span.clone()]) }.to_owned();

        Token {
            kind,
            value: string,
            span: span,
        }
    }
}
