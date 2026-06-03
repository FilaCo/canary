use crate::ir::syntax::CanaryTokenKind;

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

impl PartialEq<CanaryTokenKind> for TokenType {
    fn eq(&self, other: &CanaryTokenKind) -> bool {
        matches!(
            (self, other),
            (NL, CanaryTokenKind::NL)
                | (Semi, CanaryTokenKind::Semi)
                | (Minus, CanaryTokenKind::Minus)
                | (Plus, CanaryTokenKind::Plus)
                | (Slash, CanaryTokenKind::Slash)
                | (Star, CanaryTokenKind::Star)
                | (LParen, CanaryTokenKind::LParen)
                | (RParen, CanaryTokenKind::RParen)
                | (LitConst, CanaryTokenKind::LitConst { kind: _, value: _ })
                | (EOF, CanaryTokenKind::EOF)
        )
    }
}

impl PartialEq<TokenType> for CanaryTokenKind {
    fn eq(&self, other: &TokenType) -> bool {
        other == self
    }
}
