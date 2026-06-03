use std::path::PathBuf;

#[derive(Debug)]
pub struct Canary {
    pub cfg: CanaryConfig,
}

#[derive(Debug)]
pub struct CanaryConfig {
    pub input: PathBuf,
}
