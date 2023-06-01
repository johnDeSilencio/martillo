use crate::apply::*;
use crate::parse::*;
use crate::types::*;

pub fn entry(command: &Commands) -> Result<(), MappingsParseError> {
    match command {
        Commands::Apply { file } => {
            if let Ok(bongo_settings) = parse(file) {
                // Settings file exists and is valid; apply the settings
                apply(&bongo_settings)
            } else {
                // Settings file either doesn't exist or is invalid; use default settings
                apply(&BongoSettings::default())
            }
        }
        Commands::Validate { file } => {
            if let Err(parse_err) = parse(file) {
                // Settings file either doesn't exist or is invalid; notify the user
                eprintln!("ERROR: {:?} is invalid", file);

                Err(parse_err)
            } else {
                // Notify the user that the settings file is valid
                println!("[*] {:?} is valid", file);

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::entry;

    use crate::types::*;
    use std::path::PathBuf;

    #[test]
    fn test_entry_valid_mappings() {
        let test_cases = [
            Commands::Validate {
                file: PathBuf::from("./testdata/valid1/mappings.toml"),
            },
            Commands::Validate {
                file: PathBuf::from("./testdata/valid2/mappings.toml"),
            },
            Commands::Validate {
                file: PathBuf::from("./testdata/valid2/mappings.toml"),
            },
        ];

        for test_case in test_cases.iter() {
            let result = entry(test_case);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_entry_invalid_mappings() {
        let test_cases = [
            Commands::Validate {
                file: PathBuf::from("./testdata/invalid1/anyotherfilename.toml"),
            },
            Commands::Validate {
                file: PathBuf::from("./testdata/invalid2/mappings.toml"),
            },
            Commands::Validate {
                file: PathBuf::from("./testdata/invalid2/mappings.toml"),
            },
        ];

        for test_case in test_cases.iter() {
            let result = entry(test_case);
            assert!(result.is_err());
        }
    }
}
