use crate::constants::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BongoSettingsConfig {
    pub global: GlobalConfig,
    pub freestyle: Option<Vec<FreestyleRhythmConfig>>,
}

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    pub microphone: Option<bool>,
    pub debounce: Option<u16>,
}

#[derive(Deserialize, Debug)]
pub struct FreestyleRhythmConfig {
    pub character: char,
    pub beats: Vec<String>,
    pub delays: Vec<u16>,
}

#[derive(Deserialize, Debug)]
pub struct BongoSettings {
    pub global: GlobalSettings,
    pub freestyle: FreestyleSettings,
}

impl Default for BongoSettings {
    fn default() -> Self {
        Self {
            global: GlobalSettings::default(),
            freestyle: FreestyleSettings::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GlobalSettings {
    pub debounce: u16,
    pub microphone: bool,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            debounce: MIN_DEBOUNCE_TIME,
            microphone: false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FreestyleSettings(pub Option<Vec<FreestyleRhythm>>);

impl Default for FreestyleSettings {
    fn default() -> Self {
        Self(None)
    }
}

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
