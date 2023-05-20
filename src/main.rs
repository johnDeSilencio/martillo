use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

const ABOUT_DESCRIPTION: &'static str = "\
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
            if let Some(bongo_settings) = parse(file) {
                // Settings file exists and is valid; apply the settings
                apply(&bongo_settings);
            } else {
                // Settings file either doesn't exist or is invalid; use default settings
                apply(&DEFAULT_BONGO_SETTINGS);
            }
        }
        Commands::Validate { file } => {
            if let Some(_) = parse(file) {
                // Notify the user that the settings file is valid
                println!("[*] {:?} is valid", file);
            } else {
                eprintln!("ERROR: {:?} is invalid", file);

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

const DEFAULT_BONGO_SETTINGS: BongoSettings = BongoSettings {
    microphone_enabled: false,
    debounce_beat: 100, // milliseconds
    freestyle_rhythms: None,
};

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

fn parse(file: &PathBuf) -> Option<BongoSettings> {
    return todo!();
    file.exists() && file.to_str().unwrap() == "mappings.toml";
}

fn apply(settings: &BongoSettings) {
    println!("{:?}", settings.microphone_enabled);
    println!("{:?}", settings.debounce_beat);
}
