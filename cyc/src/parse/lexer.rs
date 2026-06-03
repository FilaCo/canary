use std::str::Chars;

use crate::ir::{
    source::Span,
    syntax::{CanaryToken, CanaryTokenKind},
};

use CanaryTokenKind::*;

pub(super) struct Lexer<'src> {
    input: Chars<'src>,
    /// Byte position in the input stream.
    pos: usize,
    token: CanaryToken,
}

impl<'db> Lexer<'db> {
    pub fn new(input: &'db str) -> Self {
        let mut lexer = Self {
            input: input.chars(),
            pos: 0,
            token: CanaryToken::dummy(),
        };
        let _ = lexer.bump();
        lexer
    }

    pub fn first(&self) -> &CanaryToken {
        &self.token
    }

    pub fn bump(&mut self) -> CanaryToken {
        let next_tok = loop {
            let (next_tok, is_next_tok_preceded_by_ws) = self.next_token_from_input();

            if is_next_tok_preceded_by_ws {
                break next_tok;
            } else if let Some(glued) = self.token.glue(&next_tok) {
                self.token = glued;
            } else {
                break next_tok;
            }
        };
        std::mem::replace(&mut self.token, next_tok)
    }

    fn next_token_from_input(&mut self) -> (CanaryToken, bool) {
        let mut preceeded_by_ws = false;
        let mut swallow_next_invalid = 0;
        // Skip trivial (whitespace & comments) tokens
        loop {
            let str_before = self.input.as_str();
            let start = self.pos;
            let Some(first_char) = self.bump_char() else {
                return (
                    CanaryToken::new(EOF, Span::new(start, self.pos)),
                    preceeded_by_ws,
                );
            };

            let kind = match first_char {
                c if is_whitespace(c) => {
                    self.ws();
                    preceeded_by_ws = true;
                    continue;
                }

                CR_CHAR => match self.first_char() {
                    LF_CHAR => {
                        self.bump();

                        NL
                    }
                    _ => {
                        self.ws();
                        preceeded_by_ws = true;
                        continue;
                    }
                },
                LF_CHAR => NL,

                // '0'..='9' => self.int(str_before, start),
                ';' => Semi,

                '-' => Minus,
                '+' => Plus,
                '/' => Slash,
                '*' => Star,
                '(' => LParen,
                ')' => RParen,
                _ => {
                    preceeded_by_ws = true;
                    continue;
                }
            };

            let span = Span::new(start, self.pos);
            return (CanaryToken::new(kind, span), preceeded_by_ws);
        }
    }

    fn ws(&mut self) {
        self.eat_char_while(is_whitespace)
    }

    fn int(&mut self, str_before: &'db str, start: usize) -> CanaryTokenKind {
        todo!()
        // self.eat_dec_digits();
        // let end = self.pos - start;
        // Int {
        //     value: &str_before[..end],
        // }
    }

    fn eat_dec_digits(&mut self) {
        loop {
            match self.first_char() {
                '0'..='9' | '_' => {
                    self.bump_char();
                }
                _ => break,
            }
        }
    }

    fn eat_char_while(&mut self, predicate: impl Fn(char) -> bool) {
        while !self.is_at_eof() && predicate(self.first_char()) {
            self.bump_char();
        }
    }

    fn bump_char(&mut self) -> Option<char> {
        let c = self.input.next();
        self.pos += c.map(|c| c.len_utf8()).unwrap_or(0);
        c
    }

    fn first_char(&self) -> char {
        self.input.clone().next().unwrap_or(EOF_CHAR)
    }

    fn is_at_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }
}

const EOF_CHAR: char = '\0';
const LF_CHAR: char = '\u{000A}';
const CR_CHAR: char = '\u{000D}';

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        | '\u{0020}' // space
        | '\u{0009}' // tab
        | '\u{000C}' // form feed
    )
}

impl<'src> Iterator for Lexer<'src> {
    type Item = CanaryToken;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.bump();
        match t.kind {
            EOF => None,
            _ => Some(t),
        }
    }
}
