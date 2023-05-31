mod apply;
mod constants;
mod parse;
mod types;

use crate::apply::*;
use crate::constants::*;
use crate::parse::*;
use crate::types::*;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(version)]
#[command(about = ABOUT_DESCRIPTION, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Apply the settings in mappings.toml (only for use on DK-BASIC hardware)")]
    Apply {
        #[arg(help = "The mappings.toml file to apply")]
        file: PathBuf,
    }, // the mappings.toml file
    #[command(about = "Validate the mappings.toml file")]
    Validate {
        #[arg(help = "The mappings.toml file to validate")]
        file: PathBuf,
    }, // the mappings.toml file
}

fn main() -> ExitCode {
    // Parse the command-line input and run the program
    main_body(Args::parse())
}

fn main_body(args: Args) -> ExitCode {
    // Default to returning success
    let mut exit_code = ExitCode::from(0);

    match &args.command {
        Commands::Apply { file } => {
            if let Ok(bongo_settings) = parse(file) {
                // Settings file exists and is valid; apply the settings
                apply(&bongo_settings);
            } else {
                // Settings file either doesn't exist or is invalid; use default settings
                apply(&BongoSettings::default());
            }
        }
        Commands::Validate { file } => {
            if parse(file).is_ok() {
                // Notify the user that the settings file is valid
                println!("[*] {:?} is valid", file);
            } else {
                // Settings file either doesn't exist or is invalid; notify the user
                eprintln!("ERROR: {:?} is invalid", file);

                // Return non-zero exit code to indicate the error
                exit_code = ExitCode::from(1);
            }
        }
    }

    // Return the exit code and end the process
    exit_code
}
