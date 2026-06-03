use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct File {
    pub path: PathBuf,
    pub contents: String,
}
