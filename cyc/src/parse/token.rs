use crate::{ir::syntax::Value, parse::Span};

use TokenKind::*;

#[derive(Clone, Copy, Debug)]
pub(super) struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span,
}

impl<'db> Token<'db> {
    pub fn new(kind: TokenKind<'db>, span: Span) -> Self {
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
pub(super) enum TokenKind<'db> {
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
    LitConst {
        kind: LitConstKind,
        value: Value<'db>,
    },

    /// Dummy token for parser needs.
    Dummy,

    /// End of input.
    EOF,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum LitConstKind {
    Int,
    Float,
}
