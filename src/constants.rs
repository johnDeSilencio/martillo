pub const ABOUT_DESCRIPTION: &str = "\
    Rust subcomponent for processing & validating DK-BASIC config files.\n\n\
    If this utility is being run on on DK-BASIC hardware,\n\
    it should be run with the \"apply\" parameter; else, it\n\
    should be run with the \"validate\" parameter.";

pub const MIN_DEBOUNCE_TIME: u16 = 10; // milliseconds
pub const MAX_DEBOUNCE_TIME: u16 = 500; // milliseconds

pub const MIN_COMBO_WINDOW: u16 = 100; // milliseconds
pub const MAX_COMBO_WINDOW: u16 = 5000; // milliseconds
