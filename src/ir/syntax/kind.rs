#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, salsa::Update)]
#[repr(u8)]
pub enum SyntaxKind {
    BlockComment,
    LineComment,
    WS,
    NL,
    LParen,
}
