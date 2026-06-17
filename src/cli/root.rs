use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
pub struct CanaryCli {}
