use crate::constants::*;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MappingsParseError {
    #[error("file does not exist")]
    InvalidFilename,

    #[error("cannot read the contents of `{0}`")]
    CannotReadFile(String),

    #[error("cannot parse mappings.toml")]
    InvalidToml,

    #[error("invalid debounce time: `{0}` ms")]
    InvalidDebounceTime(u16),

    #[error("invalid combo window: `{0}` ms")]
    InvalidComboWindow(u16),

    #[error("invalid character `{0}` for freestyle rhythm")]
    InvalidCharacter(char),

    #[error("invalid beat `{beat:?}` in freestyle rhythm for character `{character:?}`")]
    InvalidBeat { beat: String, character: char },

    #[error("invalid delay of `{delay:?}` ms in freestyle rhythm for character `{character:?}`")]
    InvalidDelay { delay: u16, character: char },

    #[error("invalid rhythm for character `{0}`: must define at least one beat")]
    EmptyRhythm(char),

    #[error(
        "invalid rhythm for character `{character:?}`: there must be one fewer delays than beats"
    )]
    TooManyDelays { character: char },

    #[error(
        "invalid rhythm for character `{character:?}`: there must be one fewer delays than beats"
    )]
    TooFewDelays { character: char },
}

#[derive(Deserialize, Debug)]
pub struct BongoConfig {
    pub global: GlobalConfig,
    pub freestyle: Option<Vec<FreestyleConfig>>,
}

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    pub debounce: Option<u16>,
    pub combo_window: Option<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FreestyleConfig {
    pub character: char,
    pub beats: Vec<String>,
    pub delays: Vec<u16>,
}

#[derive(Deserialize, Debug, Default)]
pub struct BongoSettings {
    pub global: GlobalSettings,
    pub freestyle: FreestyleSettings,
}

#[derive(Deserialize, Debug)]
pub struct GlobalSettings {
    pub debounce: u16,
    pub combo_window: u16,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            debounce: MIN_DEBOUNCE_TIME,
            combo_window: MIN_COMBO_WINDOW,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct FreestyleSettings(pub Option<Vec<FreestyleRhythm>>);

#[derive(Debug, Deserialize)]
pub struct FreestyleRhythm {
    pub character: char,
    pub beats: Vec<(BongoInput, Option<BeatDelay>)>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum BongoInput {
    BackLeftBongo,
    FrontLeftBongo,
    BackRightBongo,
    FrontRightBongo,
    StartPauseButton,
    ClapMicrophone,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BeatDelay(pub u16);
