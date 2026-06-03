use std::str::Chars;

use crate::{
    ir::{
        source::{BytePos, Span},
        syntax::{Token, TokenKind},
    },
    parse::lexer::cursor::{self, Cursor},
};

use TokenKind::*;

#[derive(Debug)]
pub(super) struct Lexer<'src> {
    start_pos: BytePos,
    /// Byte position in the input stream.
    pos: BytePos,
    /// Cursor for getting lexer tokens.
    cursor: Cursor<'src>,
    /// Current token.
    token: Token,
}

impl<'src> Lexer<'src> {
    pub fn first(&self) -> &Token {
        &self.token
    }

    pub fn bump(&mut self) -> Token {
        let next_tok = loop {
            let (next_tok, is_next_tok_preceded_by_ws) = self.next_token_from_cursor();

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

    fn next_token_from_cursor(&mut self) -> (Token, bool) {
        let mut preceeded_by_ws = false;
        let mut swallow_next_invalid = 0;
        // Skip trivial (whitespace & comments) tokens
        loop {
            let str_before = self.cursor.as_str();
            let token = self.cursor.bump_token();
            let start = self.pos;
            self.pos += BytePos(token.len);

            let kind = match token.kind {
                cursor::TokenKind::Whitespace => {
                    preceeded_by_ws = true;
                    continue;
                }

                cursor::TokenKind::Newline => Newline,
                cursor::TokenKind::Semi => Semi,

                cursor::TokenKind::Minus => Minus,
                cursor::TokenKind::Plus => Plus,
                cursor::TokenKind::Slash => Slash,
                cursor::TokenKind::Star => Star,

                cursor::TokenKind::LParen => LParen,
                cursor::TokenKind::RParen => RParen,

                cursor::TokenKind::LitConst { kind } => {
                    todo!()
                }

                cursor::TokenKind::Unknown => {
                    todo!()
                }

                cursor::TokenKind::EndOfFile => EndOfFile,
            };

            todo!()
        }
    }
}
