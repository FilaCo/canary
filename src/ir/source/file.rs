use std::{path::PathBuf, sync::Arc};

use crate::ir::source::BytePos;

#[salsa::input(debug)]
pub struct File {
    #[returns(ref)]
    pub path: PathBuf,
    pub contents: Arc<str>,
    // pub span: Span,
    #[returns(ref)]
    pub line_starts: Vec<BytePos>,
}
