use crate::constants::*;
use crate::types::*;
use std::fs;
use std::path::PathBuf;

pub fn parse(file: &PathBuf) -> Result<BongoSettings, MappingsParseError> {
    // Get the file name from the path buf
    let file_name = file
        .file_name()
        .ok_or(MappingsParseError::InvalidFilename)?;

    // Get an OsString from the &OsStr
    let file_name = file_name.to_os_string();

    // Get a string slice from the OsString
    let file_name = file_name
        .to_str()
        .ok_or(MappingsParseError::InvalidFilename)?;

    // Make sure the file exists and is named "mappings.toml"
    if !file.exists() || file_name != "mappings.toml" {}

    // Read in the data from the settings file
    let config_data = fs::read_to_string(file)
        .ok()
        .ok_or(MappingsParseError::CannotReadFile(file_name.to_owned()))?;

    // Parse the configuration from the data
    let config: BongoConfig = toml::from_str(&config_data)
        .ok()
        .ok_or(MappingsParseError::InvalidToml)?;

    // Parse the global settings from the configuration
    let global_settings = parse_global(&config.global)?;

    // Parse any freestyle rhythm settings from the configuration, if specified
    let freestyle_settings: FreestyleSettings;

    if let Some(freestyle) = config.freestyle {
        freestyle_settings = parse_freestyle(freestyle)?;
    } else {
        freestyle_settings = FreestyleSettings(None);
    }

    // Return success with the parsed settings
    Ok(BongoSettings {
        global: global_settings,
        freestyle: freestyle_settings,
    })
}

pub fn parse_global(config: &GlobalConfig) -> Result<GlobalSettings, MappingsParseError> {
    let settings = GlobalSettings::default();

    // Validate the debounce interval if supplied by the user
    if let Some(debounce) = config.debounce {
        if !(MIN_DEBOUNCE_TIME..=MAX_DEBOUNCE_TIME).contains(&debounce) {
            return Err(MappingsParseError::InvalidDebounceTime(debounce));
        }
    }

    // Validate the combo window if supplied by the user
    if let Some(combo_window) = config.combo_window {
        if !(MIN_COMBO_WINDOW..=MAX_COMBO_WINDOW).contains(&combo_window) {
            return Err(MappingsParseError::InvalidComboWindow(combo_window));
        }
    }

    // If we've made it here, the settings are valid
    Ok(settings)
}

pub fn parse_freestyle(
    config: Vec<FreestyleConfig>,
) -> Result<FreestyleSettings, MappingsParseError> {
    let mut settings = FreestyleSettings::default();

    let mut freestyle: Vec<FreestyleRhythm> = Vec::new();

    for rhythm in config.iter() {
        let character = rhythm.character;
        if !character.is_ascii() {
            return Err(MappingsParseError::InvalidCharacter(character));
        }

        if rhythm.beats.is_empty() {
            return Err(MappingsParseError::EmptyRhythm(character));
        }

        if rhythm.beats.len() < rhythm.delays.len() + 1 {
            return Err(MappingsParseError::TooFewDelays { character });
        }

        if rhythm.beats.len() > rhythm.delays.len() + 1 {
            return Err(MappingsParseError::TooManyDelays { character });
        }

        let beats = rhythm
            .beats
            .iter()
            .map(|beat| -> Result<BongoInput, MappingsParseError> {
                match beat.as_str() {
                    "BLB" => Ok(BongoInput::BackLeftBongo),
                    "FLB" => Ok(BongoInput::FrontLeftBongo),
                    "BRB" => Ok(BongoInput::BackRightBongo),
                    "FRB" => Ok(BongoInput::FrontRightBongo),
                    "SPB" => Ok(BongoInput::StartPauseButton),
                    "MIC" => Ok(BongoInput::ClapMicrophone),
                    _ => Err(MappingsParseError::InvalidBeat {
                        beat: beat.to_string(),
                        character,
                    }),
                }
            })
            .collect::<Result<Vec<BongoInput>, MappingsParseError>>()?;

        let mut delays = rhythm
            .delays
            .iter()
            .map(|delay| -> Result<Option<BeatDelay>, MappingsParseError> {
                if *delay < MIN_DEBOUNCE_TIME || *delay > MAX_DEBOUNCE_TIME {
                    Err(MappingsParseError::InvalidDelay {
                        delay: *delay,
                        character,
                    })
                } else {
                    Ok(Some(BeatDelay(*delay)))
                }
            })
            .collect::<Result<Vec<Option<BeatDelay>>, MappingsParseError>>()?;

        // The last delay should be None because
        // there is no delay after the last beat
        delays.push(None);

        let rhythm = beats
            .iter()
            .zip(delays.iter())
            .map(|(beat, delay)| (beat.clone(), delay.clone()))
            .collect::<Vec<(BongoInput, Option<BeatDelay>)>>();

        freestyle.push(FreestyleRhythm {
            character,
            beats: rhythm,
        });
    }

    // Save parsed freestyle rhytms in the settings
    settings.0 = Some(freestyle);

    // Return struct with successfully parsed settings
    Ok(settings)
}

#[cfg(test)]
mod tests {
    use crate::parse::*;

    #[test]
    fn test_parse_global_valid() {
        let test_cases = [
            GlobalConfig {
                debounce: None,
                combo_window: None,
            },
            GlobalConfig {
                debounce: Some(MIN_DEBOUNCE_TIME),
                combo_window: None,
            },
            GlobalConfig {
                debounce: Some(MAX_DEBOUNCE_TIME),
                combo_window: None,
            },
            GlobalConfig {
                debounce: None,
                combo_window: Some(MIN_COMBO_WINDOW),
            },
            GlobalConfig {
                debounce: None,
                combo_window: Some(MAX_DEBOUNCE_TIME),
            },
        ];

        for test_case in test_cases.iter() {
            let settings = parse_global(test_case);

            assert!(settings.is_ok());
        }
    }

    #[test]
    fn test_parse_global_invalid() {
        let test_cases = [
            GlobalConfig {
                debounce: Some(MIN_DEBOUNCE_TIME - 1),
                combo_window: None,
            },
            GlobalConfig {
                debounce: Some(MAX_DEBOUNCE_TIME + 1),
                combo_window: None,
            },
            GlobalConfig {
                debounce: None,
                combo_window: Some(MIN_COMBO_WINDOW - 1),
            },
            GlobalConfig {
                debounce: None,
                combo_window: Some(MAX_COMBO_WINDOW + 1),
            },
        ];

        for test_case in test_cases.iter() {
            let settings = parse_global(test_case);

            assert!(settings.is_err());
        }
    }

    #[test]
    fn test_parse_freestyle_valid() {
        let test_cases = [
            Vec::new(),
            vec![FreestyleConfig {
                character: '$',
                beats: vec!["MIC".to_string()],
                delays: vec![],
            }],
        ];

        for test_case in test_cases.iter() {
            let settings = parse_freestyle(test_case.to_vec());

            assert!(settings.is_ok());
        }
    }
}
