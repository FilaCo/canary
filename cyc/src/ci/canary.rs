use std::path::PathBuf;

#[derive(Debug)]
pub struct Canary {
    pub cfg: CanaryConfig,
}

impl Canary {
    pub fn parse_seed(&self) {}
}

#[derive(Debug)]
pub struct CanaryConfig {
    pub input: PathBuf,
}
