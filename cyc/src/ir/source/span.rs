use crate::ir::source::{BytePos, SourceFile};

#[salsa::tracked(debug)]
pub struct Span<'db> {
    /// inclusive
    #[tracked]
    pub start: BytePos,
    /// exclusive
    #[tracked]
    pub end: BytePos,
    #[tracked]
    #[returns(ref)]
    pub file: SourceFile,
}
