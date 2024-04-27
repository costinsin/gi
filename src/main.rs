use clap::Parser;
use eyre::Result;
use gi::{
    cli::args::{Args, Commands},
    commands::create::create,
    project::settings::ProjectSettings,
};

fn main() -> Result<()> {
    let args = Args::parse();

    color_eyre::install()?;
    let git_client = gi::git_client::git_cli::GitCli::new()?;
    let mut project_settings = ProjectSettings::new()?;

    match args.command {
        Commands::Create => create(&git_client, &mut project_settings)?,
    }

    Ok(())
}
