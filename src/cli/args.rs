use clap::{Parser, Subcommand};

/// Git Improved
#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(aliases = &["c"])]
    Create,
}
