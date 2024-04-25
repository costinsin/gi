use clap::Parser;
use eyre::Result;
use gi::{
    cli::args::{Args, Commands},
    commands::create::create,
};

fn main() -> Result<()> {
    let args = Args::parse();

    color_eyre::install()?;
    let git_client = gi::git_client::git_cli::GitCli::new()?;

    match args.command {
        Commands::Create => create(&git_client),
    }

    Ok(())
}
