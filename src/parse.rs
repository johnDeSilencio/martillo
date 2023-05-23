use crate::constants::*;
use crate::types::*;
use std::fs;
use std::path::PathBuf;

pub fn parse(file: &PathBuf) -> Option<BongoSettings> {
    let file_name = file.file_name()?;

    // Make sure the file exists and is named "mappings.toml"
    if !file.exists() || file_name != "mappings.toml" {
        return None;
    }

    // Read in the data from the settings file
    let config_data = fs::read_to_string(file).ok()?;

    // Parse the configuration from the data
    let config: BongoConfig = toml::from_str(&config_data).ok()?;

    // Parse the global settings from the configuration
    let global_settings = parse_global(&config)?;

    // Parse any freestyle rhythm settings from the configuration, if specified
    let freestyle_settings = parse_freestyle(&config, &global_settings)?;

    // Return success with the parsed settings
    Some(BongoSettings {
        global: global_settings,
        freestyle: freestyle_settings,
    })
}

pub fn parse_global(config: &BongoConfig) -> Option<GlobalSettings> {
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

pub fn parse_freestyle(
    config: &BongoConfig,
    global_settings: &GlobalSettings,
) -> Option<FreestyleSettings> {
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
                        if !global_settings.microphone {
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

#[cfg(test)]
mod tests {
    use crate::parse::*;

    #[test]
    fn test_parse_global_valid() {
        let test_cases = [
            BongoConfig {
                global: GlobalConfig {
                    debounce: None,
                    microphone: None,
                },
                freestyle: None,
            },
            BongoConfig {
                global: GlobalConfig {
                    debounce: Some(MIN_DEBOUNCE_TIME),
                    microphone: None,
                },
                freestyle: None,
            },
            BongoConfig {
                global: GlobalConfig {
                    debounce: Some(MAX_DEBOUNCE_TIME),
                    microphone: None,
                },
                freestyle: None,
            },
            BongoConfig {
                global: GlobalConfig {
                    debounce: None,
                    microphone: Some(false),
                },
                freestyle: None,
            },
            BongoConfig {
                global: GlobalConfig {
                    debounce: None,
                    microphone: Some(true),
                },
                freestyle: None,
            },
        ];

        for test_case in test_cases.iter() {
            let settings = parse_global(test_case);

            assert_eq!(true, settings.is_some());
        }
    }

    #[test]
    fn test_parse_global_invalid() {
        let test_cases = [
            BongoConfig {
                global: GlobalConfig {
                    debounce: Some(MIN_DEBOUNCE_TIME - 1),
                    microphone: None,
                },
                freestyle: None,
            },
            BongoConfig {
                global: GlobalConfig {
                    debounce: Some(MAX_DEBOUNCE_TIME + 1),
                    microphone: None,
                },
                freestyle: None,
            },
        ];

        for test_case in test_cases.iter() {
            let settings = parse_global(test_case);

            assert_eq!(true, settings.is_none());
        }
    }

    #[test]
    fn test_parse_freestyle_valid() {
        let test_cases = [
            BongoConfig {
                global: GlobalConfig {
                    debounce: None,
                    microphone: None,
                },
                freestyle: None,
            },
            BongoConfig {
                global: GlobalConfig {
                    debounce: None,
                    microphone: None,
                },
                freestyle: Some(vec![]),
            },
        ];

        for test_case in test_cases.iter() {
            let settings = parse_freestyle(
                test_case,
                &GlobalSettings {
                    debounce: 100,
                    microphone: false,
                },
            );

            assert_eq!(true, settings.is_some());
        }
    }
}
