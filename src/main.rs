mod apply;
mod cli;
mod constants;
mod parse;
mod types;

use crate::cli::*;
use crate::types::*;
use clap::Parser;

fn main() -> Result<(), MappingsParseError> {
    // Parse the command-line input and run the program
    let args = Args::parse();
    let command = args.command;

    entry(&command)
}
