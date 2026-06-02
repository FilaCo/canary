use std::str::Chars;

use crate::{
    db::CanaryDb,
    ir::source::BytePos,
    parse::{Span, Token, TokenKind},
};

use TokenKind::*;

pub(super) struct Lexer<'db> {
    db: &'db dyn CanaryDb,
    input: Chars<'db>,
    pos: BytePos,
    token: Token<'db>,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn CanaryDb, input: &'db str) -> Self {
        let mut lexer = Self {
            db,
            input: input.chars(),
            pos: BytePos(0),
            token: Token::dummy(),
        };
        let _ = lexer.bump();
        lexer
    }

    pub fn first(&self) -> &Token<'db> {
        &self.token
    }

    pub fn bump(&mut self) -> Token<'db> {
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

    fn next_token_from_input(&mut self) -> (Token<'db>, bool) {
        let mut preceeded_by_ws = false;
        let mut swallow_next_invalid = 0;
        // Skip trivial (whitespace & comments) tokens
        loop {
            let str_before = self.input.as_str();
            let start = self.pos;
            let Some(first_char) = self.bump_char() else {
                return (
                    Token::new(EOF, self.mk_sp(start, self.pos)),
                    preceeded_by_ws,
                );
            };

            let kind = match first_char {
                c if is_whitespace(c) => {
                    self.ws();
                    preceeded_by_ws = true;
                    continue;
                }
                // '0'..='9' => self.int(str_before, start),
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

            let span = self.mk_sp(start, self.pos);
            return (Token::new(kind, span), preceeded_by_ws);
        }
    }

    fn ws(&mut self) {
        self.eat_char_while(is_whitespace)
    }

    fn int(&mut self, str_before: &'db str, start: BytePos) -> TokenKind<'db> {
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

    fn mk_sp(&self, start: BytePos, end: BytePos) -> Span {
        Span::new(start, end)
    }

    fn eat_char_while(&mut self, predicate: impl Fn(char) -> bool) {
        while !self.is_at_eof() && predicate(self.first_char()) {
            self.bump_char();
        }
    }

    fn bump_char(&mut self) -> Option<char> {
        let c = self.input.next();
        self.pos += BytePos(c.map(|c| c.len_utf8()).unwrap_or(0));
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

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        | '\u{0020}' // space
        | '\u{0009}' // tab
        | '\u{000C}' // form feed
    )
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.bump();
        match t.kind {
            EOF => None,
            _ => Some(t),
        }
    }
}
