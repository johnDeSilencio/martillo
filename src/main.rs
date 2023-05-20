use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const ABOUT_DESCRIPTION: &'static str = "\
    Rust subcomponent for processing & validating DK-BASIC config files.\n\n\
    If this utility is being run on on DK-BASIC hardware,\n\
    it should be run with the \"apply\" parameter; else, it\n\
    should be run with the \"validate\" parameter.";

const MIN_DEBOUNCE_TIME: u16 = 100;
const MAX_DEBOUNCE_TIME: u16 = 5000;

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

const DEFAULT_BONGO_SETTINGS: BongoSettings = BongoSettings {
    global: GlobalConfig {
        microphone_enabled: Some(false),
        debounce_beat: Some(100), // milliseconds
    },
    freestyle: FreestyleConfig {
        freestyle_rhythms: None,
    },
};

#[derive(Deserialize, Debug)]
struct FreestyleRhythm {
    character: char,
    beats: Vec<(BongoInput, Option<BeatDelay>)>,
}

#[derive(Deserialize, Debug)]
enum BongoInput {
    BLB,
    FLB,
    BRB,
    FRB,
    SPB,
    MIC,
}

#[derive(Deserialize, Debug)]
struct BeatDelay(u8);

#[derive(Deserialize, Debug)]
struct BongoSettings {
    global: GlobalConfig,
    freestyle: FreestyleConfig,
}

#[derive(Deserialize, Debug)]
struct GlobalConfig {
    microphone_enabled: Option<bool>,
    debounce_beat: Option<u16>,
}

#[derive(Deserialize, Debug)]
struct FreestyleConfig {
    freestyle_rhythms: Option<Vec<FreestyleRhythm>>,
}

fn parse(file: &PathBuf) -> Option<BongoSettings> {
    let file_name = file.file_name()?;

    if !file.exists() || file_name != "mappings.toml" {
        return None;
    }

    // Read in the data from the settings file
    let settings_data = fs::read_to_string(file).ok()?;

    println!("{:?}", settings_data);

    // Parse the settings from the data
    let mut parsed_settings: BongoSettings = toml::from_str(&settings_data).ok()?;

    println!("{:?}", parsed_settings);

    if !validate(&mut parsed_settings) {
        return None;
    }

    // Return success with the parsed settings
    Some(parsed_settings)
}

fn validate(settings: &mut BongoSettings) -> bool {
    // Validate the debounce interval if supplied by the user
    if let Some(debounce_beat) = settings.global.debounce_beat {
        if debounce_beat < 100 || debounce_beat > 5000 {
            return false;
        }
    } else {
        settings.global.debounce_beat = DEFAULT_BONGO_SETTINGS.global.debounce_beat;
    }

    println!("What about here?");

    // Validate any freestyle rhythms
    if let Some(rhythms) = &settings.freestyle.freestyle_rhythms {
        println!("I'm here!");
        for rhythm in rhythms.iter() {
            println!("{:?}", rhythm.character);
        }
    }

    // Supply clap microphone setting if not supplied by the user
    if settings.global.microphone_enabled.is_none() {
        settings.global.microphone_enabled = DEFAULT_BONGO_SETTINGS.global.microphone_enabled;
    }

    // If we've made it here, the settings are valid
    true
}

fn apply(settings: &BongoSettings) {
    println!("{:?}", settings.global.microphone_enabled);
    println!("{:?}", settings.global.debounce_beat);
    println!(
        "freestyle enabled? {:?}",
        settings.freestyle.freestyle_rhythms.is_some()
    );
}
