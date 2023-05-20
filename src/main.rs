use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

static ABOUT_DESCRIPTION: &'static str = "\
    Rust subcomponent for processing & validating DK-BASIC config files.\n\n\
    If this utility is being run on on DK-BASIC hardware,\n\
    it should be run with the \"apply\" parameter; else, it\n\
    should be run with the \"validate\" parameter.";

#[derive(Parser)]
#[command(version)]
#[command(about = ABOUT_DESCRIPTION, long_about=None)]
struct Cli {
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
    // Parse the command-line input
    let cli = Cli::parse();

    // Default to returning success
    let mut exit_code = ExitCode::from(0);

    match &cli.command {
        Commands::Apply { file } => {
            println!("I'm applying {:?}", file);
            if validate(file) {
                // Settings file exists and is valid; apply the settings
                apply(file);
            } else {
                // Settings files either doesn't exist or is invalid; use default settings
            }
        }
        Commands::Validate { file } => {
            if validate(file) {
                println!("{:?} is valid!", file);
            } else {
                eprintln!("{:?} is invalid :(", file);

                // Return non-zero exit code to indicate the error
                exit_code = ExitCode::from(1);
            }
        }
    }

    // Return the exit code and end the process
    exit_code
}

struct BongoSettings {
    microphone_enabled: bool,
    debounce_beat: u16,
    freestyle_rhythms: Option<Vec<FreestyleRhythm>>,
}

struct FreestyleRhythm {
    character: char,
    beats: Vec<(BongoInput, Option<BeatDelay>)>,
}

enum BongoInput {
    BackLeftBongo,
    FrontLeftBongo,
    BackRightBongo,
    FrontRightBongo,
    StartPauseButton,
    ClapMicrophone,
}

struct BeatDelay(u8);

fn validate(file: &PathBuf) -> bool {
    file.exists() && file.to_str().unwrap() == "mappings.toml"
}

fn apply(_file: &PathBuf) {}
