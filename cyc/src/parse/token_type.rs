use crate::parse::TokenKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    /// `LF | (CR [LF])`
    NL,

    /// `;`
    Semi,

    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,

    /// `(`
    LParen,
    /// `)`
    RParen,

    /// A literal constant value, e.g. `42` or `321.123`.
    LitConst,

    /// End of input.
    EOF,
}

use TokenType::*;

impl From<TokenKind<'_>> for TokenType {
    fn from(value: TokenKind<'_>) -> Self {
        match value {
            TokenKind::NL => NL,
            TokenKind::Semi => Semi,
            TokenKind::Minus => Minus,
            TokenKind::Plus => Plus,
            TokenKind::Slash => Slash,
            TokenKind::Star => Star,
            TokenKind::LParen => LParen,
            TokenKind::RParen => RParen,
            TokenKind::LitConst { kind: _, value: _ } => LitConst,
            TokenKind::Dummy => todo!(),
            TokenKind::EOF => EOF,
        }
    }
}
