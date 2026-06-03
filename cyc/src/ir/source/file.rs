use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub contents: String,
}
