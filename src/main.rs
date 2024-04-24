use clap::Parser;
use eyre::Result;
use gi::cli::args::{Args, Commands};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match args.command {
        Commands::Create => {
            println!("Create command");
        }
    }

    Ok(())
}
