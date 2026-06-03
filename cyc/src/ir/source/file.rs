use std::path::PathBuf;

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub contents: String,
}
