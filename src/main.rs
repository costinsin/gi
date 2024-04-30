use clap::Parser;
use eyre::Result;
use gi::{
    cli::args::{Args, Commands},
    commands::create::create,
    commands::submit::submit,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    color_eyre::install()?;

    match args.command {
        Commands::Create => create()?,
        Commands::Submit => submit().await?,
    }

    Ok(())
}
