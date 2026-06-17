use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
pub(super) struct CanaryCli {
    #[command(subcommand)]
    pub(super) cmd: Option<CanaryCommand>,
    /// Use verbose output
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub(super) verbose: u8,
}

#[derive(Subcommand, Debug)]
pub(super) enum CanaryCommand {
    Test,
}
