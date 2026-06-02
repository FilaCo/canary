use crate::ir::source::SourceFile;

#[salsa::tracked(debug)]
pub struct Span<'db> {
    /// inclusive
    #[tracked]
    pub start: usize,
    /// exclusive
    #[tracked]
    pub end: usize,
    #[tracked]
    #[returns(ref)]
    pub file: SourceFile,
}
