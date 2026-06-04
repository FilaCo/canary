use std::path::PathBuf;

use crate::ir::source::{BytePos, Span};

#[derive(Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub contents: String,
    pub span: Span,
    pub line_starts: Vec<BytePos>,
}

impl SourceFile {
    pub fn new(path: PathBuf, contents: String, span: Span, line_starts: Vec<BytePos>) -> Self {
        Self {
            path,
            contents,
            span,
            line_starts,
        }
    }
}
