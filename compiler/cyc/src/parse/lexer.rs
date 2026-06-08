use cyc_ir::{
    source::{BytePos, Span},
    syntax::{LitConstKind, Symbol, Token, TokenKind},
};
use cyc_lexer::Cursor;

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
        let mut preceded_by_ws = false;
        let mut swallow_next_invalid = 0;
        // Skip trivial (whitespace & comments) tokens
        loop {
            let str_before = self.cursor.as_str();
            let token = self.cursor.bump_token();
            let start = self.pos;
            self.pos += BytePos(token.len);

            let kind = match token.kind {
                cyc_lexer::TokenKind::Whitespace => {
                    preceded_by_ws = true;
                    continue;
                }

                cyc_lexer::TokenKind::Newline => Newline,
                cyc_lexer::TokenKind::Semi => Semi,

                cyc_lexer::TokenKind::Minus => Minus,
                cyc_lexer::TokenKind::Plus => Plus,
                cyc_lexer::TokenKind::Slash => Slash,
                cyc_lexer::TokenKind::Star => Star,

                cyc_lexer::TokenKind::LParen => LParen,
                cyc_lexer::TokenKind::RParen => RParen,

                cyc_lexer::TokenKind::LitConst { kind } => {
                    self.cook_lexer_lit_const(start, self.pos, kind)
                }

                cyc_lexer::TokenKind::Unknown => {
                    todo!()
                }

                cyc_lexer::TokenKind::EndOfFile => EndOfFile,
            };

            let span = Span::new(start, self.pos);
            return (Token::new(kind, span), preceded_by_ws);
        }
    }

    fn cook_lexer_lit_const(
        &self,
        start: BytePos,
        end: BytePos,
        kind: cyc_lexer::LitConstKind,
    ) -> TokenKind {
        match kind {
            cyc_lexer::LitConstKind::Int { empty_int } => todo!(),
            cyc_lexer::LitConstKind::Float { empty_exp } => todo!(),
        }
        todo!()
    }
}
