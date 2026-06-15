use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::ci::DumpKind;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CanaryDriver {
    // Input source.
    pub input: PathBuf,

    /// Specify the name of the nest.
    #[arg(long, value_name = "NAME")]
    pub nest_name: Option<String>,

    #[arg(long)]
    pub dump: Vec<DumpKind>,
}

impl Default for CanaryDriver {
    fn default() -> Self {
        CanaryDriver::parse()
    }
}
