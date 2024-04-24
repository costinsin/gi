use clap::Parser;
use gi::cli::args::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Create => {
            println!("Create command");
        }
    }
}
