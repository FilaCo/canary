use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use crate::ir::source::{BytePos, SourceFile, Span};

#[derive(Debug)]
pub struct SourceMap {
    inner: RwLock<SourceMapImpl>,
}

impl SourceMap {
    pub fn new() -> Self {
        let src_map_impl = SourceMapImpl::new();
        let inner = RwLock::new(src_map_impl);
        Self { inner }
    }

    pub fn add(&self, path: &Path) -> Arc<SourceFile> {
        if let Some(src_file_ptr) = self.get_by_path(path) {
            return src_file_ptr;
        }

        self.register(read_source_file(&path))
    }

    pub fn get_by_pos(&self, pos: BytePos) -> Option<Arc<SourceFile>> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        let n = inner.files.partition_point(|f| f.span.lo <= pos);
        inner
            .files
            .get(n.checked_sub(1)?)
            .filter(|file| pos < file.span.hi)
            .cloned()
    }

    pub fn get_by_path(&self, path: &Path) -> Option<Arc<SourceFile>> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        inner.files_by_path.get(path).cloned()
    }

    fn register(&self, mut src_file: SourceFile) -> Arc<SourceFile> {
        let mut inner = self.inner.write().expect("unable to acquire write lock");
        if let Some(src_file_ptr) = inner.files_by_path.get(&src_file.path).cloned() {
            // Somebody added file while we were waiting for write lock
            return src_file_ptr;
        }

        let lo = inner.bytes_len;
        src_file.span = Span::new(lo, lo + src_file.span.hi);
        for line_pos in &mut src_file.line_starts {
            *line_pos += lo;
        }
        inner.bytes_len = src_file.span.hi + BytePos(1);

        let src_file_ptr = Arc::new(src_file);

        inner.files.push(src_file_ptr.clone());
        inner
            .files_by_path
            .insert(src_file_ptr.path.clone(), src_file_ptr.clone());

        src_file_ptr
    }
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct SourceMapImpl {
    files: Vec<Arc<SourceFile>>,
    files_by_path: HashMap<PathBuf, Arc<SourceFile>>,
    bytes_len: BytePos,
}

impl SourceMapImpl {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            files_by_path: HashMap::new(),
            bytes_len: BytePos(0),
        }
    }
}

impl Default for SourceMapImpl {
    fn default() -> Self {
        Self::new()
    }
}

fn read_source_file(path: &Path) -> SourceFile {
    let contents = std::fs::read_to_string(path).expect("unable to read file");
    source_file_from_contents(path.to_path_buf(), contents)
}

/// Build a SourceFile with a *file-local* span `[0, len)` from in-memory contents.
fn source_file_from_contents(path: PathBuf, contents: String) -> SourceFile {
    let line_starts = compute_line_starts(&contents);
    let hi = BytePos::from_usize(contents.len());
    SourceFile::new(path, contents, Span::new(BytePos(0), hi), line_starts)
}

/// Byte offsets of each line start. `lines[0]` is always `BytePos(0)` so every file
/// (even empty) has line 0. A new line starts after each terminator.
///
/// INVARIANT: the set of terminators here MUST match the lexer cursor's `Newline`
/// rule (parse/lexer/cursor/tokenize.rs): `\r\n` (one break), lone `\r`, and `\n`.
/// If you change one, change the other — otherwise spans and line/col desync.
fn compute_line_starts(contents: &str) -> Vec<BytePos> {
    let mut chars = contents.chars();
    let mut pos = BytePos(0);
    let mut lines = vec![pos];

    while let Some(c) = chars.next() {
        match c {
            CR_CHAR => {
                if chars.clone().next() == Some(LF_CHAR) {
                    chars.next();
                    pos += BytePos(2);
                } else {
                    pos += BytePos(1);
                }
                lines.push(pos);
            }
            LF_CHAR => {
                pos += BytePos(1);
                lines.push(pos);
            }
            c => {
                pos += BytePos::from_usize(c.len_utf8());
            }
        }
    }

    lines
}

const LF_CHAR: char = '\u{000A}';
const CR_CHAR: char = '\u{000D}';

#[cfg(test)]
mod tests {
    use super::*;
}
