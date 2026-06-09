use crate::{source::Span, syntax::Symbol};

use TokenKind::*;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn dummy() -> Self {
        Self::new(Dummy, Span::dummy())
    }

    pub fn glue(&self, _: &Self) -> Option<Self> {
        None
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    /// `LF | (CR [LF])`
    Newline,

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
    LitConst(LiteralConst),

    /// Dummy token for parser needs.
    Dummy,

    Error,

    /// End of input.
    EndOfFile,
}

#[derive(Clone, Copy, Debug)]
pub struct LiteralConst {
    pub kind: LitConstKind,
    pub sym: Symbol,
}

#[derive(Clone, Copy, Debug)]
pub enum LitConstKind {
    Int,
    Float,
    Error,
}
