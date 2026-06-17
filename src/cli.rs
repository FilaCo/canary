use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
pub(super) struct CanaryCli {
    #[command(subcommand)]
    pub(super) cmd: Option<CanaryCommand>,
}

#[derive(Subcommand, Debug)]
pub(super) enum CanaryCommand {}
