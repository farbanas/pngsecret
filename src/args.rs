use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args, Debug, Clone)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
    #[clap(value_parser)]
    pub message: String,
    #[clap(value_parser)]
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug, Clone)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args, Debug, Clone)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
}
