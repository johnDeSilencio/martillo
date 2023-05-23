mod constants;
mod types;

use crate::constants::*;
use crate::types::*;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

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
                apply(&BongoSettings::default());
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

fn parse(file: &PathBuf) -> Option<BongoSettings> {
    let file_name = file.file_name()?;

    // Make sure the file exists and is named "mappings.toml"
    if !file.exists() || file_name != "mappings.toml" {
        return None;
    }

    // Read in the data from the settings file
    let config_data = fs::read_to_string(file).ok()?;

    // Parse the configuration from the data
    let config: BongoSettingsConfig = toml::from_str(&config_data).ok()?;

    // Parse the global settings from the configuration
    let global_settings = parse_global(&config)?;

    // Parse any freestyle rhythm settings from the configuration, if specified
    let freestyle_settings = parse_freestyle(&config)?;

    // Return success with the parsed settings
    Some(BongoSettings {
        global: global_settings,
        freestyle: freestyle_settings,
    })
}

fn parse_freestyle(config: &BongoSettingsConfig) -> Option<FreestyleSettings> {
    let mut settings = FreestyleSettings::default();

    if let Some(rhythms) = &config.freestyle {
        let mut freestyle: Vec<FreestyleRhythm> = Vec::new();

        for rhythm in rhythms.iter() {
            let character = rhythm.character;
            if !character.is_ascii() {
                return None;
            }

            if rhythm.beats.is_empty() || rhythm.beats.len() != rhythm.delays.len() + 1 {
                return None;
            }

            let mut valid = true;

            let beats = rhythm
                .beats
                .iter()
                .map(|beat| match beat.as_str() {
                    "BLB" => BongoInput::BackLeftBongo,
                    "FLB" => BongoInput::FrontLeftBongo,
                    "BRB" => BongoInput::BackRightBongo,
                    "FRB" => BongoInput::FrontRightBongo,
                    "SPB" => BongoInput::StartPauseButton,
                    "MIC" => {
                        if !config.global.microphone.unwrap() {
                            // User can only use the microphone in a rhythm if
                            // it has been explicitly enabled in the config file
                            valid = false;
                        }

                        BongoInput::ClapMicrophone
                    }
                    _ => {
                        // Signal that the file is invalid to return early
                        valid = false;

                        // Return any input to satisfy closure
                        // since we will return early anyways
                        BongoInput::ClapMicrophone
                    }
                })
                .collect::<Vec<BongoInput>>();

            if !valid {
                // Return early if there were any invalid beats
                return None;
            }

            let mut delays = rhythm
                .delays
                .iter()
                .map(|delay| {
                    if *delay < MIN_DEBOUNCE_TIME || *delay > MAX_DEBOUNCE_TIME {
                        valid = false;
                        None
                    } else {
                        Some(BeatDelay(*delay))
                    }
                })
                .collect::<Vec<Option<BeatDelay>>>();

            if !valid {
                // Return early if there were any invalid delays
                return None;
            }

            // The last delay should be None because
            // there is no delay after the last beat
            delays.push(None);

            let rhythm = beats
                .iter()
                .zip(delays.iter())
                .map(|(beat, delay)| (beat.clone(), delay.clone()))
                .collect::<Vec<(BongoInput, Option<BeatDelay>)>>();

            freestyle.push(FreestyleRhythm {
                character: character,
                beats: rhythm,
            });
        }

        // Save parsed freestyle rhytms in the settings
        settings.0 = Some(freestyle);
    }

    // Return struct with successfully parsed settings
    Some(settings)
}

fn parse_global(config: &BongoSettingsConfig) -> Option<GlobalSettings> {
    let settings = GlobalSettings::default();

    // Validate the debounce interval if supplied by the user
    if let Some(debounce) = config.global.debounce {
        if debounce < 100 || debounce > 5000 {
            return None;
        }
    }

    // If we've made it here, the settings are valid
    Some(settings)
}

fn apply(settings: &BongoSettings) {
    println!("{:?}", settings);
}
