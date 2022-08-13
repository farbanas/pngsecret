use self::args::{Cli, Commands};
use self::commands::{run_decode, run_encode, run_print, run_remove};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode(args) => run_encode(args)?,
        Commands::Decode(args) => run_decode(args)?,
        Commands::Remove(args) => run_remove(args)?,
        Commands::Print(args) => run_print(args)?,
    }

    Ok(())
}
