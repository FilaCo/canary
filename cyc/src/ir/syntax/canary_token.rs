use crate::ir::source::Span;

use CanaryTokenKind::*;

#[derive(Debug)]
pub struct CanaryToken {
    pub kind: CanaryTokenKind,
    pub span: Span,
}

impl CanaryToken {
    pub fn new(kind: CanaryTokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn dummy() -> Self {
        Self::new(Dummy, Span::dummy())
    }

    pub fn glue(&self, _: &Self) -> Option<Self> {
        None
    }
}

#[derive(Debug)]
pub enum CanaryTokenKind {
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
    LitConst { kind: LitConstKind, value: String },

    /// Dummy token for parser needs.
    Dummy,

    /// End of input.
    EOF,
}

#[derive(Debug)]
pub enum LitConstKind {
    Int,
    Float,
}
