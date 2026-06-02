use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CanaryDriver {
    // Input source.
    pub input: PathBuf,
}

impl Default for CanaryDriver {
    fn default() -> Self {
        CanaryDriver::parse()
    }
}
